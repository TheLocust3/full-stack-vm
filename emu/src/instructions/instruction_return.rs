use cpu::register::Register;

pub struct RegisterReturn {
    pub out: Register,
    pub overflow: bool,
    pub negative: bool
}

pub struct MemoryReturn {
    pub value: u64,
    pub address: u64,
    pub overflow: bool,
    pub negative: bool
}
