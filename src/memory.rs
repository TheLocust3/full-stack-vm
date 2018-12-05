use bitwise;

const TOTAL_MEMORY: usize = 1073741824;

pub struct Memory {
    pub memory: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory: vec![0; TOTAL_MEMORY] }
    }

    pub fn read_8bit(&self, address: usize) -> u8 {
        if address <= 0 || address >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        self.memory[address]
    }

    pub fn read_16bit(&self, address: usize) -> u16 {
        if address <= 0 || (address + 1) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        bitwise::to_16bit(self.memory[address], self.memory[address + 1])
    }

    pub fn read_32bit(&self, address: usize) -> u32 {
        if address <= 0 || (address + 3) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        bitwise::to_32bit(self.memory[address], self.memory[address] + 1,
                          self.memory[address] + 2, self.memory[address] + 3)
    }

    pub fn read_64bit(&self, address: usize) -> u64 {
        if address <= 0 || (address + 7) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        bitwise::to_64bit(self.memory[address], self.memory[address] + 1,
                          self.memory[address] + 2, self.memory[address] + 3,
                          self.memory[address] + 4, self.memory[address] + 5,
                          self.memory[address] + 6, self.memory[address] + 7)
    }

    pub fn write_8bit(&self, address: usize, data: u8) -> bool {
        true
    }

    pub fn write_16bit(&self, address: usize, data: u16) -> bool {
        true
    }

    pub fn write_32bit(&self, address: usize, data: u32) -> bool {
        true
    }

    pub fn write_64bit(&self, address: usize, data: u64) -> bool {
        true
    }
}
