use cpu::register::Register;
use memory::Memory;
use cpu::executor;

pub struct CPU {
    pub a: Register,
    pub b: Register,
    pub c: Register,
    pub d: Register,
    pub e: Register,
    pub f: Register,
    pub hl: Register,
    pub pc: Register,
    pub memory: Memory
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        CPU {
            a: Register::new(),
            b: Register::new(),
            c: Register::new(),
            d: Register::new(),
            e: Register::new(),
            f: Register::new(),
            hl: Register::new(),
            pc: Register::new(),
            memory: memory
        }
    }

    pub fn cycle(&mut self) {
        executor::execute(self);
    }
}
