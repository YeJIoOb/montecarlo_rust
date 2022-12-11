#[macro_use]
extern crate lazy_static;

mod mc;
mod sim;

use std::{collections::HashMap, sync::Arc};

use mc::{MonteCarlo};
use sim::{ignis::SimulationIgnis};
fn main() {

    lazy_static! {
        static ref PARAMETERS: HashMap<&'static str, &'static str> = {
            let mut m = HashMap::new();
            m.insert("start_modif", "0");
            m.insert("end_modif", "6");
            m
        };
    }
    let mc = MonteCarlo::<u32>::new_with(
        2_000_000usize, 
        Arc::new(Box::new(SimulationIgnis {}))
    );

    let avg = mc.get_avg_t(&PARAMETERS);

    // let avg = mc.get_avg(&PARAMETERS);
    println!("avg is {}", avg);
}
