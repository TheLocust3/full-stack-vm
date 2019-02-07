use bitwise::functions;

const TOTAL_MEMORY: u64 = 1073741824;

pub struct Memory {
    pub memory: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory: vec![0; TOTAL_MEMORY as usize] }
    }

    pub fn read_8bit(&self, address: u64) -> u8 {
        if address >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        self.memory[address as usize]
    }

    pub fn read_16bit(&self, address: u64) -> u16 {
        if (address + 1) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        functions::to_16bit(self.read_8bit(address),
                          self.read_8bit(address + 1))
    }

    pub fn read_32bit(&self, address: u64) -> u32 {
        if (address + 3) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        functions::to_32bit(self.read_8bit(address),
                          self.read_8bit(address + 1),
                          self.read_8bit(address + 2),
                          self.read_8bit(address + 3))
    }

    pub fn read_64bit(&self, address: u64) -> u64 {
        if (address + 7) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        functions::to_64bit(self.read_8bit(address),
                          self.read_8bit(address + 1),
                          self.read_8bit(address + 2),
                          self.read_8bit(address + 3),
                          self.read_8bit(address + 4),
                          self.read_8bit(address + 5),
                          self.read_8bit(address + 6),
                          self.read_8bit(address + 7))
    }

    pub fn write_8bit(&mut self, address: u64, data: u8) -> bool {
        if address >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        self.memory[address as usize] = data;

        true
    }

    pub fn write_16bit(&mut self, address: u64, data: u16) -> bool {
        if (address + 1) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        let bytes: [u8; 2] = functions::to_bytes_16bit(data);
        self.write_8bit(address, bytes[0]);
        self.write_8bit(address + 1, bytes[1]);

        true
    }

    pub fn write_32bit(&mut self, address: u64, data: u32) -> bool {
        if (address + 3) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        let bytes: [u8; 4] = functions::to_bytes_32bit(data);
        self.write_8bit(address, bytes[0]);
        self.write_8bit(address + 1, bytes[1]);
        self.write_8bit(address + 2, bytes[2]);
        self.write_8bit(address + 3, bytes[3]);

        true
    }

    pub fn write_64bit(&mut self, address: u64, data: u64) -> bool {
        if (address + 7) >= TOTAL_MEMORY {
            panic!("Address: {}, out of bounds", address);
        }

        let bytes: [u8; 8] = functions::to_bytes_64bit(data);
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
