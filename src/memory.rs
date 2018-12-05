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

        bitwise::to_16bit(self.read_8bit(address),
                          self.read_8bit(address + 1))
    }

    pub fn read_32bit(&self, address: usize) -> u32 {
        if address <= 0 || (address + 3) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        bitwise::to_32bit(self.read_8bit(address),
                          self.read_8bit(address + 1),
                          self.read_8bit(address + 2),
                          self.read_8bit(address + 3))
    }

    pub fn read_64bit(&self, address: usize) -> u64 {
        if address <= 0 || (address + 7) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        bitwise::to_64bit(self.read_8bit(address),
                          self.read_8bit(address + 1),
                          self.read_8bit(address + 2),
                          self.read_8bit(address + 3),
                          self.read_8bit(address + 4),
                          self.read_8bit(address + 5),
                          self.read_8bit(address + 6),
                          self.read_8bit(address + 7))
    }

    pub fn write_8bit(&mut self, address: usize, data: u8) -> bool {
        if address <= 0 || address >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        self.memory[address] = data;

        true
    }

    pub fn write_16bit(&mut self, address: usize, data: u16) -> bool {
        if address <= 0 || (address + 1) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        let bytes: [u8; 2] = bitwise::to_bytes_16bit(data);
        self.write_8bit(address, bytes[0]);
        self.write_8bit(address + 1, bytes[1]);

        true
    }

    pub fn write_32bit(&mut self, address: usize, data: u32) -> bool {
        if address <= 0 || (address + 3) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        let bytes: [u8; 4] = bitwise::to_bytes_32bit(data);
        self.write_8bit(address, bytes[0]);
        self.write_8bit(address + 1, bytes[1]);
        self.write_8bit(address + 2, bytes[2]);
        self.write_8bit(address + 3, bytes[3]);

        true
    }

    pub fn write_64bit(&mut self, address: usize, data: u64) -> bool {
        if address <= 0 || (address + 7) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        let bytes: [u8; 8] = bitwise::to_bytes_64bit(data);
        self.write_8bit(address, bytes[0]);
        self.write_8bit(address + 1, bytes[1]);
        self.write_8bit(address + 2, bytes[2]);
        self.write_8bit(address + 3, bytes[3]);
        self.write_8bit(address + 4, bytes[4]);
        self.write_8bit(address + 5, bytes[5]);
        self.write_8bit(address + 6, bytes[6]);
        self.write_8bit(address + 7, bytes[7]);

        true
    }
}
