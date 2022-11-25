mod mc;
mod sim;

use std::thread::{self, JoinHandle};
use std::{ collections::HashMap };

use mc::{MonteCarlo};
use sim::SimulationIgnis;

impl<'a> MonteCarlo<'a, u32> {
    pub fn get_avg(&'a self, params: HashMap<&str, &str>) -> f32 {
        let mut sum = 0.0f32;

        let num_cpus = num_cpus::get() - 1;
        let chunks_size = self.n / num_cpus;

        let threads: Vec::<JoinHandle::<f32>> = vec![];

        for num_cpu in 0..num_cpus {
            let start = num_cpu * chunks_size;
            let end = (num_cpu + 1) * chunks_size;

            let th = thread::spawn(|| {
                let mut sum = 0.0f32;
                for _ in start..end {
                    let x = self.simulation.run(&params);
                    sum += x as f32;
                }
                sum
            });
            threads.push(th);
        }

        for thread in threads {
            let val = thread.join().unwrap();
            sum += val;
        }
        sum / (num_cpus as f32)
    }
}

fn main() {
    let sim = SimulationIgnis {};
    let mc = MonteCarlo::<u32> {
        n: 2_000_000usize,
        simulation: &sim,
    };

    let avg = mc.get_avg(
        HashMap::from([
            ("start_modif", "0"),
            ("end_modif", "6"),
        ])
    );
    println!("avg is {}", avg);
}