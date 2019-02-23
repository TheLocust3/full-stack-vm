use bitwise::functions::to_bytes_64bit;
use register::register_str_to_code;

pub fn compile_set(reg: String, value_str: String) -> Vec<u8> {
    let value: u64 = value_str.parse::<u64>().unwrap();

    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00001000 + register_str_to_code(reg)); // set {reg} {value}

    for byte in to_bytes_64bit(value).iter() {
        compiled.push(*byte);
    }

    compiled
}

pub fn compile_move(dest: String, reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00000000 + (register_str_to_code(dest) << 4) + register_str_to_code(reg)); // move {dest} {reg}

    compiled
}
