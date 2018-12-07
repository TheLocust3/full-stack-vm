use cpu::cpu::CPU;
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

    pub fn cycle(self) -> Computer {
        Computer {
            cpu: self.cpu.cycle()
        }
    }
}
