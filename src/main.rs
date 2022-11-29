mod mc;
mod sim;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use mc::{MonteCarlo, Runnable};
use sim::SimulationIgnis;

impl MonteCarlo<SimulationIgnis, u32> {
    pub fn get_avg(&self, params: Arc<HashMap<&str, &str>>) -> f32 {
        let mut sum = 0.0f32;

        let num_cpus = num_cpus::get() - 1;
        let chunks_size = self.n / num_cpus;

        let mut threads: Vec<JoinHandle<f32>> = vec![];

        for num_cpu in 0..num_cpus {
            let start: usize = num_cpu * chunks_size;
            let end: usize = (num_cpu + 1) * chunks_size;
            let sim = self.simulation.clone();
            let th = thread::spawn(&|| {
                let mut sum = 0.0f32;
                for _ in start..end {
                    let x = sim.run(params.clone());
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
    let mc = Box::new(MonteCarlo::<SimulationIgnis, u32> {
        n: 2_000_000usize,
        simulation: SimulationIgnis {},
        phantom_type: PhantomData
    });
    let params: Arc<HashMap<&str, &str>> =
        Arc::new(HashMap::from([("start_modif", "0"), ("end_modif", "6")]));
    let avg = mc.get_avg(params);
    println!("avg is {}", avg);
}
