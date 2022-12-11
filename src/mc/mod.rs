use std::sync::Arc;
use std::thread::JoinHandle;
use std::{collections::HashMap};
use std::marker::PhantomData;

pub struct MonteCarlo<T>
{
    pub n: usize,
    pub simulation: Simulation<T>,
    phantom_type: PhantomData<T>
}

type Simulation<T> = Arc<Box<dyn Runnable<T> + Send + Sync>>;

impl<T> MonteCarlo<T>
{
    pub fn new_with(n: usize, simulation: Simulation<T>) -> Self {
        Self {
            n,
            simulation,
            phantom_type: PhantomData
        }
    }
}

pub trait Runnable<T> {
    fn run(&self, params: &'static HashMap<&str, &str>) -> T;
}



impl MonteCarlo<u32> {
    pub fn get_avg(&self, params: &'static HashMap<&str, &str>) -> f32 {
        let mut sum = 0.0f32;

        for _ in 0..self.n {
            let x = self.simulation.run(params);
            sum += x as f32;
        }
        
        sum / (self.n as f32)
    }

    pub fn get_avg_t(&self, params: &'static HashMap<&str, &str>) -> f32 {
        let mut sum = 0.0f32;

        let num_cpu = num_cpus::get();
        let chunk_size = self.n / num_cpu;
        let mut ths = Vec::<JoinHandle<f32>>::new();

        for _ in 0..num_cpu {
            let sim = Arc::clone(&self.simulation);
            let handle = std::thread::spawn(move || {
                let mut sum = 0.0f32;

                for _ in 0..chunk_size {
                    let x = sim.run(params);
                    sum += x as f32;
                }

                sum / (chunk_size as f32)
            });
            ths.push(handle);
        }

        for th in ths {
            let res = th.join().unwrap();
            sum += res;
        }

        sum / (num_cpu as f32)
    }
}

