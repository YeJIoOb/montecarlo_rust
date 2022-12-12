use crossbeam::thread::{self, ScopedJoinHandle};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct MonteCarlo<T> {
    pub n: usize,
    pub simulation: Simulation<T>,
    pub values: Mutex<Vec<T>>,
    collect_values: bool,
}

pub type Simulation<T> = Arc<Box<dyn Runnable<T> + Send + Sync>>;

impl<T> MonteCarlo<T> {
    pub fn new_with(n: usize, simulation: Simulation<T>, collect_values: bool) -> Self {
        Self {
            n,
            simulation,
            values: Mutex::new(Vec::with_capacity(n)),
            collect_values,
        }
    }
}

pub trait Runnable<T> {
    fn run(&self, params: &'static Mutex<HashMap<&str, &str>>) -> T;
}

impl MonteCarlo<u32> {
    pub fn get_avg(&self, params: &'static Mutex<HashMap<&str, &str>>) -> f32 {
        let sum = (0..self.n).into_iter().fold(0.0f32, move |acc, _| {
            let x = self.simulation.run(params);
            if self.collect_values {
                self.values.lock().unwrap().push(x);
            }
            acc + x as f32
        });

        sum / (self.n as f32)
    }

    pub fn get_avg_t(&self, params: &'static Mutex<HashMap<&str, &str>>) -> f32 {
        let num_cpu = num_cpus::get();
        let chunk_size = self.n / num_cpu;
        let values = Arc::new(&self.values);

        let sim = Arc::clone(&self.simulation);
        let values = Arc::clone(&values);
        let res = thread::scope(|s| {
            let sum = (0..num_cpu)
                .into_iter()
                .fold(Vec::<ScopedJoinHandle<f32>>::new(), |mut acc, _| {
                    let handle = s.spawn(|_| {
                        (0..chunk_size).into_iter().fold(0f32, |acc, _| {
                            let x = sim.run(params);
                            if self.collect_values {
                                values.lock().unwrap().push(x);
                            }
                            acc + x as f32
                        }) / (chunk_size as f32)
                    });
                    acc.push(handle);
                    acc
                })
                .into_iter()
                .fold(0.0f32, |acc, th| acc + th.join().unwrap());
            sum / (num_cpu as f32)
        });

        res.unwrap()
    }
}
