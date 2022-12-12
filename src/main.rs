#[macro_use]
extern crate lazy_static;
use std::fs::{self, OpenOptions};
use clap::{command, arg, Parser};

mod mc;
mod sim;
mod dice;

use std::io::Write;
use std::path::Path;
use std::{collections::HashMap, sync::{Arc, Mutex}};

use mc::{MonteCarlo};
use serde::{Serializer, Serialize};
use sim::{ignis::SimulationIgnis, benir::SimulationBenir};

use crate::mc::{Simulation};

pub struct MutexWrapper<T: ?Sized>(pub Mutex<T>);

impl<T: ?Sized + Serialize> Serialize for MutexWrapper<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0
            .lock()
            .expect("mutex is poisoned")
            .serialize(serializer)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the simulation
    #[arg(long, value_names=["benir | ignis"])]
    sim: String,
    /// Count of iterations to simulate
    #[arg(long)]
    n: usize,
    /// Command to compute, avaliable: avg
    #[arg(long, default_value_t = ("avg").to_string())]
    command: String,
    /// Start simulation from modify lvl
    #[arg(long)]
    start: u8,
    /// Finish simulation with modify lvl
    #[arg(long)]
    end: u8,
    /// Have to collect values to json
    #[arg(long, default_value_t = false)]
    collect_values: bool,
    /// Collect to file
    #[arg(long, default_value_t = ("./values.json").to_string())]
    collect_file: String,
}

fn write_values(values: Mutex<Vec<u32>>, file_name: &str) -> Result<usize, std::io::Error> {
    if Path::new(file_name).exists() {
        fs::remove_file(file_name).unwrap();
    }
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();
        

    let res = serde_json::to_string(&Arc::new(MutexWrapper(values))).unwrap();
    file.write(res.as_bytes())
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn main() {
    let args = Args::parse();

    lazy_static! {
        static ref PARAMETERS: Mutex<HashMap<&'static str, &'static str>> = {
            Mutex::new(HashMap::new())
        };
    }
    let sim: Result<Simulation<u32>, String> =
    match &args.sim[..] {
        "benir" => Ok(Arc::new(Box::new(SimulationBenir {}))),
        "ignis" => Ok(Arc::new(Box::new(SimulationIgnis {}))),
        el =>  {
            Err(format!("Unexpected simulation name {}. Read --help", el))
        }
    };
    if let Err(err_str) = sim {
        println!("{}", err_str);
        return;
    }
    let mc = MonteCarlo::<u32>::new_with(
        args.n, 
        sim.unwrap(),
        args.collect_values
    );
    PARAMETERS.lock().unwrap().insert("start_modif", string_to_static_str(args.start.to_string()));
    PARAMETERS.lock().unwrap().insert("end_modif", string_to_static_str(args.end.to_string()));

    if args.command == "avg" {
        let avg = {
            if args.n < 1000 {
                mc.get_avg(&PARAMETERS)
            } else {
                mc.get_avg_t(&PARAMETERS)
            }
        };
        println!("Avg is: {}", avg);
    }

    if args.collect_values {
        let file_name = &args.collect_file[..];
        let write_result = write_values(mc.values, file_name);
        
        match write_result {
            Ok(_) => {},
            Err(er) => panic!("{}", er)
        }
    }
    

}
