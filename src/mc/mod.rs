use std::sync::Arc;
use std::{collections::HashMap};
use std::marker::PhantomData;

pub struct MonteCarlo<TSim, T>
where TSim : Runnable<T> + Send + Sync
{
    pub n: usize,
    pub simulation: TSim,
    phantom_type: PhantomData<T>
}

pub trait Runnable<T> {
    fn run(&self, params: Arc<HashMap<&str, &str>>) -> T;
    fn run_mut(&mut self, params: Arc<HashMap<&str, &str>>) -> T;
}