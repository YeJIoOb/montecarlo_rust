use std::collections::HashMap;

pub struct MonteCarlo<'a, T> {
    pub n: usize,
    pub simulation: &'a (dyn Runnable<T> + Send + Sync),
}

pub trait Runnable<T> {
    fn run(&self, params: &HashMap<&str, &str>) -> T;
    fn run_mut(&mut self, params: &HashMap<&str, &str>) -> T;
}