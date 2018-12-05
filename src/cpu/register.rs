#[derive(Debug)]
pub struct Register {
    pub value: u64
}

impl Register {
    pub fn new() -> Register {
        Register { value: 0 }
    }

    pub fn set_value(&self, value: u64) -> Register {
        Register { value: value }
    }
}
