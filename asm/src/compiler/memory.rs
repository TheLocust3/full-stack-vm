use register::register_str_to_code;

pub fn compile_read8(dest: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b11111000 + register_str_to_code(dest)); // read8 {dest}

    compiled
}

pub fn compile_read16(dest: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b11110000 + register_str_to_code(dest)); // read16 {dest}

    compiled
}

pub fn compile_read32(dest: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b11100000 + register_str_to_code(dest)); // read32 {dest}

    compiled
}

pub fn compile_read64(dest: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b11000000 + register_str_to_code(dest)); // read64 {dest}

    compiled
}

pub fn compile_write8(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b10000000 + register_str_to_code(reg)); // write8 {reg}

    compiled
}

pub fn compile_write16(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b10001000 + register_str_to_code(reg)); // write16 {reg}

    compiled
}

pub fn compile_write32(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b10010000 + register_str_to_code(reg)); // write32 {reg}

    compiled
}

pub fn compile_write64(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b10101000 + register_str_to_code(reg)); // write64 {reg}

    compiled
}
