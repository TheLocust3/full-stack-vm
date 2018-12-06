use cpu::cpu::CPU;
use cpu::register::Register;
use memory::Memory;

pub struct Computer {
    cpu: CPU
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            cpu: CPU::new(Memory::new())
        }
    }

    pub fn cycle(&mut self) {
        println!("Cycling computer");
        
        self.cpu.cycle();
    }
}