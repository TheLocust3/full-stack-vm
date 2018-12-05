mod bitwise;
mod cpu;
mod memory;

use cpu::cpu::CPU;
use memory::Memory;

fn main() {
    let cpu: CPU = CPU::new();
    let memory: Memory = Memory::new();

    println!("Hello, world!");
    println!("Test: {:?}", cpu);
}
