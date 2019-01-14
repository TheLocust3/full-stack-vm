use cpu::register::Register;
use memory::Memory;
use cpu::executor;
use instructions::instruction_return::RegisterReturn;

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
    pub fn new() -> CPU {
        CPU {
            a: Register::new(),
            b: Register::new(),
            c: Register::new(),
            d: Register::new(),
            e: Register::new(),
            f: Register::new(),
            hl: Register::new(),
            pc: Register::new(),
            memory: Memory::new()
        }
    }

    pub fn set_a(self, a: Register) -> CPU {
        CPU {
            a: a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
            hl: self.hl,
            pc: self.pc,
            memory: self.memory
        }
    }

    pub fn set_b(self, b: Register) -> CPU {
        CPU {
            a: self.a,
            b: b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
            hl: self.hl,
            pc: self.pc,
            memory: self.memory
        }
    }

    pub fn set_c(self, c: Register) -> CPU {
        CPU {
            a: self.a,
            b: self.b,
            c: c,
            d: self.d,
            e: self.e,
            f: self.f,
            hl: self.hl,
            pc: self.pc,
            memory: self.memory
        }
    }

    pub fn set_d(self, d: Register) -> CPU {
        CPU {
            a: self.a,
            b: self.b,
            c: self.c,
            d: d,
            e: self.e,
            f: self.f,
            hl: self.hl,
            pc: self.pc,
            memory: self.memory
        }
    }

    pub fn set_e(self, e: Register) -> CPU {
        CPU {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: e,
            f: self.f,
            hl: self.hl,
            pc: self.pc,
            memory: self.memory
        }
    }

    pub fn set_f(self, f: Register) -> CPU {
        CPU {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: f,
            hl: self.hl,
            pc: self.pc,
            memory: self.memory
        }
    }

    pub fn set_hl(self, hl: Register) -> CPU {
        CPU {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
            hl: hl,
            pc: self.pc,
            memory: self.memory
        }
    }
    
    pub fn set_pc(self, pc: Register) -> CPU {
        CPU {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
            hl: self.hl,
            pc: pc,
            memory: self.memory
        }
    }

    pub fn set_memory(self, memory: Memory) -> CPU {
        CPU {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
            hl: self.hl,
            pc: self.pc,
            memory: memory
        }
    }

    pub fn set_f_from_register_return(self, register_return: RegisterReturn) -> CPU {
        let mut flag = 0;

        if register_return.overflow {
            flag += 1;
        }

        if register_return.negative {
            flag += 2;
        }

        self.set_f(Register { value: flag })
    }

    pub fn cycle(self) -> CPU {
        executor::execute(self)
    }
}
