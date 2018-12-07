extern crate log;
extern crate env_logger;

mod bitwise;
mod memory;
mod instructions;
mod cpu;
mod computer;

use computer::Computer;

fn main() {
    env_logger::init();
    
    let computer: Computer = Computer::new();

    computer.cycle();
}
