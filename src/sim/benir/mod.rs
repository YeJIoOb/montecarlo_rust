use std::{collections::HashMap, sync::Mutex};
use rand::{ thread_rng };

use crate::{mc::Runnable, dice::Dice};

#[derive(Debug, Clone)]
pub struct SimulationBenir {}

// const CHANCES: &[f32] = &[
//     1.0,
//     1.0,
//     0.5,
//     0.45,
//     0.4,
//     0.35,
//     0.5,
//     0.45,
//     0.4,
//     0.35,
//     0.3,
//     0.25,
//     0.5,
//     0.35,
//     0.3,
//     0.25,
//     0.2,
//     0.15,
//     0.5,
//     0.35,
//     0.3,
//     0.25,
//     0.2,
//     0.15,
//     0.5
// ];

// lazy_static! {
//     static ref CHANCES : Vec<Dice> = {
//         vec![
//             Dice::new_with(1.0),
//             Dice::new_with(1.0),
//             Dice::new_with(0.5),
//             Dice::new_with(0.45),
//             Dice::new_with(0.4),
//             Dice::new_with(0.35),
//             Dice::new_with(0.5),
//             Dice::new_with(0.45),
//             Dice::new_with(0.4),
//             Dice::new_with(0.35),
//             Dice::new_with(0.3),
//             Dice::new_with(0.25),
//             Dice::new_with(0.5),
//             Dice::new_with(0.35),
//             Dice::new_with(0.3),
//             Dice::new_with(0.25),
//             Dice::new_with(0.2),
//             Dice::new_with(0.15),
//             Dice::new_with(0.5),
//             Dice::new_with(0.35),
//             Dice::new_with(0.3),
//             Dice::new_with(0.25),
//             Dice::new_with(0.2),
//             Dice::new_with(0.15),
//             Dice::new_with(0.5),
//         ]
//     };
// }

fn make_chances() -> Vec<Dice> {
    vec![
        Dice::new_with(1.0, thread_rng()),
        Dice::new_with(1.0, thread_rng()),
        Dice::new_with(0.5, thread_rng()),
        Dice::new_with(0.45, thread_rng()),
        Dice::new_with(0.4, thread_rng()),
        Dice::new_with(0.35, thread_rng()),
        Dice::new_with(0.5, thread_rng()),
        Dice::new_with(0.45, thread_rng()),
        Dice::new_with(0.4, thread_rng()),
        Dice::new_with(0.35, thread_rng()),
        Dice::new_with(0.3, thread_rng()),
        Dice::new_with(0.25, thread_rng()),
        Dice::new_with(0.5, thread_rng()),
        Dice::new_with(0.35, thread_rng()),
        Dice::new_with(0.3, thread_rng()),
        Dice::new_with(0.25, thread_rng()),
        Dice::new_with(0.2, thread_rng()),
        Dice::new_with(0.15, thread_rng()),
        Dice::new_with(0.5, thread_rng()),
        Dice::new_with(0.35, thread_rng()),
        Dice::new_with(0.3, thread_rng()),
        Dice::new_with(0.25, thread_rng()),
        Dice::new_with(0.2, thread_rng()),
        Dice::new_with(0.15, thread_rng()),
        Dice::new_with(0.5, thread_rng()),
    ]
}

impl Runnable<u32> for SimulationBenir {
    fn run(&self, params: &'static Mutex<HashMap<&str, &str>>) -> u32 {
        let mut spent = 0u32;

        let start_modif = match params.lock().unwrap().get("start_modif") {
            Some(x) => x.parse::<u8>().unwrap(),
            None => 1u8,
        };
        let end_modif = match params.lock().unwrap().get("end_modif") {
            Some(x) => x.parse::<u8>().unwrap(),
            None => 24u8,
        };
        let mut cur_modif = start_modif;
        let mut chances = make_chances();

        loop {

            spent += 1;

            if chances[(cur_modif + 1) as usize].check() > 0 {
                cur_modif += 1;
            } else {
                match cur_modif {
                    0..=5 => cur_modif = 1,
                    6..=11 => cur_modif = 6,
                    12..=17 => cur_modif = 12,
                    18..=23 => cur_modif = 18,
                    _ => panic!("out of range")
                }
            }

            if cur_modif == end_modif {
                break;
            }
        }
        spent
    }
}