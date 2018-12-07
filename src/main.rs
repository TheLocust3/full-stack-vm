#[macro_use] extern crate log;
extern crate env_logger;
use log::Level;

mod bitwise;
mod memory;
mod instructions;
mod cpu;
mod computer;

use computer::Computer;

fn main() {
    env_logger::init();
    
    let mut computer: Computer = Computer::new();

    computer.cycle();
}
