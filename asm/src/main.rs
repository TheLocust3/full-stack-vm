extern crate log;
extern crate env_logger;
extern crate bitwise;

use std::env;
use std::process;
use std::fs;

pub mod data;
pub mod recognizers;
mod register;
mod instruction;
mod subroutines;
mod compiler;
mod parser;
mod converter;

pub mod run;

fn main() {
    env_logger::init();

    let args: Vec<_> = env::args().collect();

    if args.len() != 3 { // main.rs INPUT_FILE OUTPUT_FILE
        println!("Wrong number of arguments!");
        println!("Format: INPUT_FILE OUTPUT_FILE");

        process::abort();
    }
    
    let in_file = &args[1];
    let out_file = &args[2];

    let program = fs::read_to_string(in_file).expect("Failed to read input file");

    let compiled = run::run(&program);

    let mut binary: String = "".into();
    for instr in compiled {
        binary.push_str(&instr.to_string());
        binary.push_str("\n");
    }

    fs::write(out_file, binary).expect("Failed to write to output file");
}
