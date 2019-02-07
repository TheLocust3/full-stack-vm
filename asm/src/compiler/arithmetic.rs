use register::register_str_to_code;

pub fn compile_add(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00001111 + (register_str_to_code(reg) << 5)); // add A {reg}

    compiled
}

pub fn compile_sub(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00011111 + (register_str_to_code(reg) << 5)); // sub A {reg}

    compiled
}
