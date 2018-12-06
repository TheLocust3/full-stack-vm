mod bitwise;
mod memory;
mod instructions;
mod cpu;
mod computer;

use computer::Computer;

fn main() {
    let mut computer: Computer = Computer::new();

    computer.cycle();
}
