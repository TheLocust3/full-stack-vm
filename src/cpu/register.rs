#[derive(Debug)]
pub struct Register {
    pub value: u64
}

impl Register {
    fn set_value(&self, value: u64) -> Register {
        Register { value: value }
    }
}
