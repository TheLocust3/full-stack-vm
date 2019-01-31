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

// splits 16bit data into array of bytes
// 0th element: high order, 1st element: low order
pub fn to_bytes_16bit(data: u16) -> [u8; 2] {
    [((data & 0xFF00) >> 8) as u8, (data & 0x00FF) as u8]
}

// splits 32bit data into array of bytes
// 0th element: highest order, last element: lowest order
pub fn to_bytes_32bit(data: u32) -> [u8; 4] {
    [((data & 0xFF000000) >> 24) as u8, ((data & 0x00FF0000) >> 16) as u8, ((data & 0x0000FF00) >> 8) as u8,
     (data & 0x000000FF) as u8]
}

// splits 64bit data into array of bytes
// 0th element: highest order, last element: lowest order
pub fn to_bytes_64bit(data: u64) -> [u8; 8] {
    [((data & 0xFF00000000000000) >> 56) as u8, ((data & 0x00FF000000000000) >> 48) as u8, ((data & 0x0000FF0000000000) >> 40) as u8,
     ((data & 0x000000FF00000000) >> 32) as u8,
     ((data & 0x00000000FF000000) >> 24) as u8, ((data & 0x0000000000FF0000) >> 16) as u8, ((data & 0x000000000000FF00) >> 8) as u8,
     (data & 0x00000000000000FF) as u8]
}
