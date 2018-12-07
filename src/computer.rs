use cpu::cpu::CPU;
use memory::Memory;

pub struct Computer {
    cpu: CPU
}

impl Computer {
    pub fn new() -> Computer {
        let mut memory: Memory = Memory::new();

        Computer {
            cpu: CPU::new()
        }
    }

    pub fn cycle(self) -> Computer {
        Computer {
            cpu: self.cpu.cycle()
        }
    }
}
