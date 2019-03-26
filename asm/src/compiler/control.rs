use register::register_str_to_code;

pub fn compile_jump(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b00101000 + register_str_to_code(reg));

    compiled
}

pub fn compile_jump0(reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0b01101000 + register_str_to_code(reg));

    compiled
}
