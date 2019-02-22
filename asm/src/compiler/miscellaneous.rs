use register::register_str_to_code;

pub fn compile_nop() -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0);

    compiled
}

pub fn compile_push(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b11001000 + register_str_to_code(reg)); // PUSH {reg}

    compiled
}

pub fn compile_pop(dest: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b11101000 + register_str_to_code(dest)); // POP {dest}

    compiled
}
