use std::process;
use log::{error};

use cpu::register::Register;
use memory::Memory;
use memory::TOTAL_MEMORY;
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
    pub sp: Register,
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
            sp: Register::new().set_value(TOTAL_MEMORY),
            pc: Register::new(),
            memory: Memory::new()
        }
    }

    pub fn get_from_code(&self, code: u8) -> Register {
        if code == 0b000 { // a
            self.a
        } else if code == 0b001 { // b
            self.b
        } else if code == 0b010 { // c
            self.c
        } else if code == 0b011 { // d
            self.d
        } else if code == 0b100 { // e
            self.e
        } else if code == 0b101 { // f
            self.f
        } else if code == 0b110 { // hl
            self.hl
        } else {
            error!("Register code: {} not handled", code);
            process::exit(1);
        }
    }

    pub fn set_from_code(self, code: u8, value: Register) -> CPU {
        if code == 0b000 { // a
            self.set_a(value)
        } else if code == 0b001 { // b
            self.set_b(value)
        } else if code == 0b010 { // c
            self.set_c(value)
        } else if code == 0b011 { // d
            self.set_d(value)
        } else if code == 0b100 { // e
            self.set_e(value)
        } else if code == 0b101 { // f
            self.set_f(value)
        } else if code == 0b110 { // hl
            self.set_hl(value)
        } else {
            error!("Register code: {} not handled", code);
            process::exit(1);
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
            sp: self.sp,
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
            sp: self.sp,
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
            sp: self.sp,
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
            sp: self.sp,
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
            sp: self.sp,
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
            sp: self.sp,
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
            sp: self.sp,
            pc: self.pc,
            memory: self.memory
        }
    }

    pub fn set_sp(self, sp: Register) -> CPU {
        CPU {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
            hl: self.hl,
            sp: sp,
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
            sp: self.sp,
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
            sp: self.sp,
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
