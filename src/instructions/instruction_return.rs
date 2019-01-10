use cpu::register::Register;

pub struct RegisterReturn {
    pub out: Register,
    pub overflow: bool,
    pub negative: bool
}

// TODO: Memory return
