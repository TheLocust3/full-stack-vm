pub fn compile_nop() -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    compiled.push(0);

    compiled
}

pub fn compile_push() -> Vec<u8>
