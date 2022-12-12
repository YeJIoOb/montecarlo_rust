use crate::dice::Dice;
use crate::mc::Runnable;
use rand::thread_rng;
use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, Clone)]
pub struct SimulationIgnis {}

fn make_chances() -> Vec<Dice> {
    vec![
        Dice::new_with(1.0, thread_rng()),
        Dice::new_with(0.5, thread_rng()),
        Dice::new_with(0.5, thread_rng()),
        Dice::new_with(0.4, thread_rng()),
        Dice::new_with(0.4, thread_rng()),
        Dice::new_with(0.3, thread_rng()),
        Dice::new_with(0.3, thread_rng()),
        Dice::new_with(0.2, thread_rng()),
        Dice::new_with(0.1, thread_rng()),
        Dice::new_with(0.03, thread_rng()),
        Dice::new_with(0.01, thread_rng()),
    ]
}

impl Runnable<u32> for SimulationIgnis {
    fn run(&self, params: &'static Mutex<HashMap<&str, &str>>) -> u32 {
        let mut spent = 0u32;
        let mut chances = make_chances();

        let start_modif = match params.lock().unwrap().get("start_modif") {
            Some(x) => x.parse::<u8>().unwrap(),
            None => 0u8,
        };
        let end_modif = match params.lock().unwrap().get("end_modif") {
            Some(x) => x.parse::<u8>().unwrap(),
            None => 10u8,
        };
        let mut cur_modif = start_modif;

        loop {
            spent += 1;

            if chances[(cur_modif + 1) as usize].check() > 0 {
                cur_modif += 1;
            } else {
                cur_modif = 0;
            }

            if cur_modif == end_modif {
                break;
            }
        }
        spent
    }
}
