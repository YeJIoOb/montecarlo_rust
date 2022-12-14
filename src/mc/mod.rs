use crossbeam::thread::{self, ScopedJoinHandle};
use rand::rngs::ThreadRng;
use rand::thread_rng;
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
    fn run(&self, params: &'static Mutex<HashMap<&str, &str>>, th_rng: &mut ThreadRng) -> Result<T, SimulationErrorKind>;
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SimulationErrorKind {
    IncorrectModifyRange(String)
}

impl MonteCarlo<u32> {
    pub fn get_avg(&self, params: &'static Mutex<HashMap<&str, &str>>) -> Result<f32, SimulationErrorKind> {
        let mut th_rng = thread_rng();
        let mut sum = 0.0f32;
        for _ in 0..self.n {
            let res = self.simulation.run(params, &mut th_rng);
            
            match res {
                Ok(x) => {
                    if self.collect_values {
                        self.values.lock().unwrap().push(x);
                    }
                    sum += x as f32;
                },
                Err(err) => { return Err(err); }
            }
        }

        Ok(sum / (self.n as f32))
    }

    pub fn get_avg_t(&self, params: &'static Mutex<HashMap<&str, &str>>) -> Result<f32, SimulationErrorKind> {
        let num_cpu = num_cpus::get();
        let chunk_size = self.n / num_cpu;
        let values = Arc::new(&self.values);

        let sim = Arc::clone(&self.simulation);
        let values = Arc::clone(&values);
        let res = thread::scope(|s| {
            let mut sum = 0.0f32;
            let mut ths = Vec::<ScopedJoinHandle<Result<f32, SimulationErrorKind>>>::new();
            for _ in 0..num_cpu {
                let handle: ScopedJoinHandle<Result<f32, SimulationErrorKind>> = s.spawn(|_| {
                    let mut th_rng = thread_rng();
                    let mut in_sum = 0f32;
                    for _ in 0..chunk_size {
                        let res = sim.run(params, &mut th_rng);
                        match res {
                            Ok(x) => {
                                if self.collect_values {
                                    values.lock().unwrap().push(x);
                                }
                                in_sum += x as f32;
                            },
                            Err(err) => { return Err(err); }
                        }
                    }
                    Ok(in_sum / (chunk_size as f32))
                });
                ths.push(handle);
            }
            for th in ths {
                if let Ok(th_res) = th.join() {
                    match th_res {
                        Ok(x) => {
                            sum += x;
                        },
                        Err(err) => { return Err(err); }
                    }
                }
                
            }
            Ok(sum / (num_cpu as f32))
        });

        res.unwrap()
    }
}
