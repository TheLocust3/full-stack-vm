extern crate log;
extern crate env_logger;
extern crate bitwise;

use std::env;
use std::process;
use std::fs;

mod memory;
mod instructions;
mod cpu;
mod computer;

use computer::Computer;

fn main() {
    env_logger::init();

    let args: Vec<_> = env::args().collect();
    
    if args.len() != 2 { // main.rs INPUT_FILE
        println!("Wrong number of arguments!");
        println!("Format: INPUT_FILE");

        process::abort();
    }

    let in_file = &args[1];

    let program = fs::read_to_string(in_file).expect("Failed to read input file");

    let mut computer: Computer = Computer::new();
    computer = computer.read_program(program);

    while !computer.is_stopped() {
        computer = computer.cycle();
    }
}
