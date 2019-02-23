use bitwise::functions::to_bytes_64bit;

pub fn compile_jump(address_str: String) -> Vec<u8> {
    let address: u64 = address_str.parse::<u64>().unwrap();
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b11111111);
    for byte in to_bytes_64bit(address).iter() {
        compiled.push(*byte);
    }

    compiled
}

pub fn compile_jump0(address_str: String) -> Vec<u8> {
    let address: u64 = address_str.parse::<u64>().unwrap();
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b11111110);
    for byte in to_bytes_64bit(address).iter() {
        compiled.push(*byte);
    }

    compiled
}
