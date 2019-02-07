use register::register_str_to_code;

pub fn compile_add(destination_reg: String, reg: String) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    // move val of A to stack
    // move dest_reg to A
    compiled.push(0b00001111 + (register_str_to_code(reg) << 4)); // add A {reg}
    // move A to dest_reg
    // set val of A from stack

    compiled
}
