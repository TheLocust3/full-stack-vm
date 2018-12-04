mod cpu;
use cpu::cpu::CPU;

fn main() {
    let cpu: CPU = CPU::new();
    println!("Hello, world!");
    println!("Test: {:?}", cpu);
}
