use instruction::Instruction;
use recognizers::is_register;
use data::parse_address;

pub fn convert_add(dest: String, input: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest == "A" && is_register(&input) {
        compiled.push(Instruction::new("ADD", &input, ""));
    } else {
        compiled.push(Instruction::new("PUSH", "A", ""));
        compiled.push(Instruction::new("MOVE", "A", &dest));

        compiled.push(Instruction::new("ADD", "A", &input));

        compiled.push(Instruction::new("MOVE", &dest, "A"));
        compiled.push(Instruction::new("POP", "A", ""));
    }

    compiled
}

pub fn should_compile_add(dest: &str, input: &str) -> bool {
    dest == "A"
}

pub fn convert_sub(dest: String, input: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest == "A" && is_register(&input) {
        compiled.push(Instruction::new("SUB", &input, ""));
    } else {
        compiled.push(Instruction::new("PUSH", "A", ""));
        compiled.push(Instruction::new("MOVE", "A", &dest));

        compiled.push(Instruction::new("SUB", "A", &input));

        compiled.push(Instruction::new("MOVE", &dest, "A"));
        compiled.push(Instruction::new("POP", "A", ""));
    }

    compiled
}

pub fn should_compile_sub(dest: &str, input: &str) -> bool {
    dest == "A"
}
