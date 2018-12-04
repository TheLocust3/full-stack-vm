use cpu::register::Register;

#[derive(Debug)]
pub struct CPU {
    pub a: Register,
    pub b: Register,
    pub c: Register,
    pub d: Register,
    pub e: Register,
    pub f: Register,
    pub hl: Register
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a: Register { value: 0 },
            b: Register { value: 0 },
            c: Register { value: 0 },
            d: Register { value: 0 },
            e: Register { value: 0 },
            f: Register { value: 0 },
            hl: Register { value: 0 }
        }
    }
}
