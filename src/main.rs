#[macro_use]
extern crate lazy_static;

use error_chain::error_chain;

mod mc;
mod sim;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use mc::{MonteCarlo};
use sim::{ignis::SimulationIgnis, benir::SimulationBenir};
fn main() {

    lazy_static! {
        static ref PARAMETERS: Mutex<HashMap<&'static str, &'static str>> = {
            Mutex::new(HashMap::new())
        };
    }
    let mc = MonteCarlo::<u32>::new_with(
        200_000usize, 
        Arc::new(Box::new(SimulationBenir {}))
    );
    PARAMETERS.lock().unwrap().insert("start_modif", "18");
    PARAMETERS.lock().unwrap().insert("end_modif", "24");
    let avg = mc.get_avg_t(&PARAMETERS);
    // let avg = mc.get_avg(&PARAMETERS);

    println!("avg is {}", avg);
}
