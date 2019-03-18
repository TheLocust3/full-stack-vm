use instruction::Instruction;

pub fn compile(instructions: Vec<Instruction>) -> String {
    let compiled: String = "".to_string();

    for instruction in instructions {
        format!("{}{}\n", compiled, compile_instruction(instruction));
    }

    compiled
}

pub fn compile_instruction(instruction: Instruction) -> String {
    let compiled: String = "".to_string();

    // compile to assembly

    compiled
}
