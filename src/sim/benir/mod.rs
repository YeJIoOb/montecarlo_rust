use std::{collections::HashMap, sync::Mutex};
use rand::{ thread_rng, Rng };

use crate::mc::Runnable;

#[derive(Debug, Clone)]
pub struct SimulationBenir {}

const CHANCES: &[f32] = &[
    1.0,
    1.0,
    0.5,
    0.45,
    0.4,
    0.35,
    0.5,
    0.45,
    0.4,
    0.35,
    0.3,
    0.25,
    0.5,
    0.35,
    0.3,
    0.25,
    0.2,
    0.15,
    0.5,
    0.35,
    0.3,
    0.25,
    0.2,
    0.15,
    0.5
];

impl Runnable<u32> for SimulationBenir {
    fn run(&self, params: &'static Mutex<HashMap<&str, &str>>) -> u32 {
        let mut random_gen = thread_rng();
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

        loop {
            let val = random_gen.gen_range(0.0f32..1.0f32);

            spent += 1;

            if CHANCES[(cur_modif + 1) as usize] > val {
                cur_modif += 1;
            } else {
                if cur_modif < 6 {
                    cur_modif = 1;
                } else if cur_modif < 12 {
                    cur_modif = 6;
                } else if cur_modif < 18 {
                    cur_modif = 12;
                } else if cur_modif < 24 {
                    cur_modif = 18;
                }
            }

            if cur_modif == end_modif {
                break;
            }
        }
        spent
    }
}