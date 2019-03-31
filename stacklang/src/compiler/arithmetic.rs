use compiler::helpers::add_instruction;

pub fn compile_add() -> String {
    let mut compiled: String = "".to_string();

    compiled = add_instruction(compiled, "POP A");
    compiled = add_instruction(compiled, "POP B");
    compiled = add_instruction(compiled, "ADD A B");
    compiled = add_instruction(compiled, "PUSH A");

    compiled
}

pub fn compile_sub() -> String {
    let mut compiled: String = "".to_string();

    compiled
}
