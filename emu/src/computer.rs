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

    pub fn read_program(&mut self, program: String) {
        let mut i = 0;
        for byte in program.split("\n") {
            self.cpu.memory.write_8bit(i, byte.parse::<u8>().unwrap_or(0));

            i += 1;
        }
    }
}
