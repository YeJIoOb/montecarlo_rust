use rand::{rngs::ThreadRng};
use std::{collections::HashMap, sync::Mutex};

use crate::{dice::{Dice, ModifyResult}, mc::{Runnable, SimulationErrorKind}};

#[derive(Debug, Clone)]
pub struct SimulationBenir {}

fn make_chances() -> Vec<Dice> {
    vec![
        Dice::new_with(1.0),
        Dice::new_with(1.0),
        Dice::new_with(0.5),
        Dice::new_with(0.45),
        Dice::new_with(0.4),
        Dice::new_with(0.35),
        Dice::new_with(0.5),
        Dice::new_with(0.45),
        Dice::new_with(0.4),
        Dice::new_with(0.35),
        Dice::new_with(0.3),
        Dice::new_with(0.25),
        Dice::new_with(0.5),
        Dice::new_with(0.35),
        Dice::new_with(0.3),
        Dice::new_with(0.25),
        Dice::new_with(0.2),
        Dice::new_with(0.15),
        Dice::new_with(0.5),
        Dice::new_with(0.35),
        Dice::new_with(0.3),
        Dice::new_with(0.25),
        Dice::new_with(0.2),
        Dice::new_with(0.15),
        Dice::new_with(0.5),
    ]
}

impl Runnable<u32> for SimulationBenir {
    fn run(&self, params: &'static Mutex<HashMap<&str, &str>>, th_rng: &mut ThreadRng) -> Result<u32, SimulationErrorKind> {
        lazy_static! { static ref CHANCES: Vec<Dice> = make_chances(); }
        let mut spent = 0u32;

        let start_modif = match params.lock().unwrap().get("start_modif") {
            Some(x) => x.parse::<u8>().unwrap(),
            None => 1u8,
        };
        let end_modif = match params.lock().unwrap().get("end_modif") {
            Some(x) => x.parse::<u8>().unwrap(),
            None => 24u8,
        };
        if start_modif == 0 || end_modif > 24 {
            return Err(SimulationErrorKind::IncorrectModifyRange("Incorrect value of modification lvl, must be between 1 and 24".to_string()));
        }

        let mut cur_modif = start_modif;
        loop {
            spent += 1;
            if let ModifyResult::Success(modify_value) = CHANCES[(cur_modif + 1) as usize].check(th_rng)  {
                cur_modif += modify_value;
            } else {
                match cur_modif {
                    0..=5 => cur_modif = 1,
                    6..=11 => cur_modif = 6,
                    12..=17 => cur_modif = 12,
                    18..=23 => cur_modif = 18,
                    _ => panic!("out of range"),
                }
            }

            if cur_modif == end_modif {
                break;
            }
        }
        Ok(spent)
    }
}
