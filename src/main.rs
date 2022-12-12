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
    let sim: Simulation<u32>;
    match &args.sim[..] {
        "benir" => sim = Arc::new(Box::new(SimulationBenir {})),
        "ignis" => sim = Arc::new(Box::new(SimulationIgnis {})),
        el =>  {
            println!("Unexpected simulation name {}. Read --help", el);
            return;
        }
    }
    let mc = MonteCarlo::<u32>::new_with(
        args.n, 
        sim,
        args.collect_values
    );
    PARAMETERS.lock().unwrap().insert("start_modif", string_to_static_str(args.start.to_string()));
    PARAMETERS.lock().unwrap().insert("end_modif", string_to_static_str(args.end.to_string()));
    let avg = mc.get_avg_t(&PARAMETERS);

    if args.collect_values {
        let file_name = &args.collect_file[..];
        let write_result = write_values(mc.values, file_name);
        
        match write_result {
            Ok(_) => println!("avg is {}", avg),
            Err(er) => panic!("{}", er)
        }
    }
    

}
