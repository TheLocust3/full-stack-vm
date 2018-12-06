use cpu::register::Register;
use memory::Memory;

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
        self.a = Register { value: 10 };
        self.a = Register { value: 20 };

        self.memory.write_8bit(0, 10);
        self.memory.write_8bit(0, 20);

        println!("Hello, world!");
        println!("{:?}", self.a);
        println!("{}", self.memory.read_8bit(0));
    }
}
