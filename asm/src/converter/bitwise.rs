use instruction::Instruction;
use recognizers::is_register;

pub fn convert_and(dest: String, input: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest == "A" && is_register(&input) {
        compiled.push(Instruction::new("AND", &input, ""));
    } else {
        compiled.push(Instruction::new("PUSH", "A", ""));
        compiled.push(Instruction::new("MOVE", "A", &dest));

        compiled.push(Instruction::new("AND", "A", &input));

        compiled.push(Instruction::new("MOVE", &dest, "A"));
        compiled.push(Instruction::new("POP", "A", ""));
    }

    compiled
}

pub fn should_compile_and(dest: &str, input: &str) -> bool {
    dest == "A"
}

pub fn convert_or(dest: String, input: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest == "A" && is_register(&input) {
        compiled.push(Instruction::new("OR", &input, ""));
    } else {
        compiled.push(Instruction::new("PUSH", "A", ""));
        compiled.push(Instruction::new("MOVE", "A", &dest));

        compiled.push(Instruction::new("OR", "A", &input));

        compiled.push(Instruction::new("MOVE", &dest, "A"));
        compiled.push(Instruction::new("POP", "A", ""));
    }

    compiled
}

pub fn should_compile_or(dest: &str, input: &str) -> bool {
    dest == "A"
}
