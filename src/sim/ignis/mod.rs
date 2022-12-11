use std::{collections::HashMap};
use rand::{ thread_rng, Rng };

use crate::mc::Runnable;

#[derive(Debug, Clone)]
pub struct SimulationIgnis {}

const CHANCES: &[f32] = &[1.0, 0.5, 0.5, 0.4, 0.4, 0.3, 0.3, 0.2, 0.1, 0.03, 0.01];

impl Runnable<u32> for SimulationIgnis {
    fn run(&self, params: &'static HashMap<&str, &str>) -> u32 {
        let mut random_gen = thread_rng();
        let mut spent = 0u32;

        let start_modif = match params.get("start_modif") {
            Some(x) => x.parse::<u8>().unwrap(),
            None => 0u8,
        };
        let end_modif = match params.get("end_modif") {
            Some(x) => x.parse::<u8>().unwrap(),
            None => 10u8,
        };
        let mut cur_modif = start_modif;

        loop {
            let val = random_gen.gen_range(0.0f32..1.0f32);

            spent += 1;

            if CHANCES[(cur_modif + 1) as usize] > val {
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