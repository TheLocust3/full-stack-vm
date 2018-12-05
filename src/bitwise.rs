// combines two bytes into one 16bit number
// byte0: high order, byte1: low order
pub fn to_16bit(byte0: u8, byte1: u8) -> u16 {
    ((byte0 as u16) << 8) + (byte1 as u16)
}

// combines four bytes into one 32bit number
// byte0: highest order, byte3: lowest order
pub fn to_32bit(byte0: u8, byte1: u8, byte2: u8, byte3: u8) -> u32 {
    ((byte0 as u32) << 24) + ((byte1 as u32) << 16) + ((byte2 as u32) << 8) + (byte3 as u32)
}

// combines two bytes into one 16bit number
// byte0: highest order, byte7: low order
pub fn to_64bit(byte0: u8, byte1: u8, byte2: u8, byte3: u8, byte4: u8, byte5: u8, byte6: u8, byte7: u8) -> u64 {
    ((byte0 as u64) << 56) + ((byte1 as u64) << 48) + ((byte2 as u64) << 40) + ((byte3 as u64) << 32) + ((byte4 as u64) << 24) + ((byte5 as u64) << 16) + ((byte6 as u64) << 8) + (byte7 as u64)
}
