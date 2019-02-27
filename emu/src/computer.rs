use cpu::cpu::CPU;
use memory::Memory;

pub struct Computer {
    cpu: CPU
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            cpu: CPU::new()
        }
    }

    pub fn cycle(self) -> Computer {
        Computer {
            cpu: self.cpu.cycle()
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.cpu.stopped
    }

    pub fn read_program(self, program: String) -> Computer {
        Computer {
            cpu: self.cpu.read_program(program)
        }
    }
}
