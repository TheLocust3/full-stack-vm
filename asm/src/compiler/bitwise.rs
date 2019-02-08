use register::register_str_to_code;

pub fn compile_and(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00011000 + (register_str_to_code(reg) << 5)); // and A {reg}

    compiled
}

pub fn compile_or(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00011001 + (register_str_to_code(reg) << 5)); // or A {reg}

    compiled
}

pub fn compile_not(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00011011 + (register_str_to_code(reg) << 5)); // not {reg}

    compiled
}

pub fn compile_shift_left_wrap(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00000111 + (register_str_to_code(reg) << 5)); // shift-left {reg}, wrap=true

    compiled
}

pub fn compile_shift_left(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00011101 + (register_str_to_code(reg) << 5)); // shift-left {reg}, wrap=false

    compiled
}

pub fn compile_shift_right_wrap(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00010111 + (register_str_to_code(reg) << 5)); // shift-right {reg}, wrap=true

    compiled
}

pub fn compile_shift_right(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00011110 + (register_str_to_code(reg) << 5)); // shift-right {reg}, wrap=false

    compiled
}
