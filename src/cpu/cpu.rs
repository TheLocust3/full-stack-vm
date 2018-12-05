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
            a: Register::new(),
            b: Register::new(),
            c: Register::new(),
            d: Register::new(),
            e: Register::new(),
            f: Register::new(),
            hl: Register::new()
        }
    }
}
