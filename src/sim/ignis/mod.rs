use crate::dice::{Dice, ModifyResult};
use crate::mc::{Runnable, SimulationErrorKind};
use rand::rngs::ThreadRng;
use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, Clone)]
pub struct SimulationIgnis {}

fn make_chances() -> Vec<Dice> {
    vec![
        Dice::new_with(1.0),
        Dice::new_with(0.5),
        Dice::new_with(0.5),
        Dice::new_with(0.4),
        Dice::new_with(0.4),
        Dice::new_with(0.3),
        Dice::new_with(0.3),
        Dice::new_with(0.2),
        Dice::new_with(0.1),
        Dice::new_with(0.03),
        Dice::new_with(0.01),
    ]
}

impl Runnable<u32> for SimulationIgnis {
    fn run(&self, params: &'static Mutex<HashMap<&str, &str>>, th_rng: &mut ThreadRng) -> Result<u32, SimulationErrorKind> {
        lazy_static! { static ref CHANCES: Vec<Dice> = make_chances(); }
        let mut spent = 0u32;

        let start_modif = match params.lock().unwrap().get("start_modif") {
            Some(x) => x.parse::<u8>().unwrap(),
            None => 0u8,
        };
        let end_modif = match params.lock().unwrap().get("end_modif") {
            Some(x) => x.parse::<u8>().unwrap(),
            None => 10u8,
        };

        if start_modif == 0 || end_modif > 10 {
            return Err(SimulationErrorKind::IncorrectModifyRange("Incorrect value of modification lvl, must be between 0 to 10".to_string()));
        }

        let mut cur_modif = start_modif;
        loop {
            spent += 1;

            if let ModifyResult::Success(modify_lvl) = CHANCES[(cur_modif + 1) as usize].check(th_rng) {
                cur_modif += modify_lvl;
            } else {
                cur_modif = 0;
            }

            if cur_modif == end_modif {
                break;
            }
        }
        Ok(spent)
    }
}
