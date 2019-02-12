use instruction::Instruction;

pub fn convert_add(dest: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest != "A" {
        compiled.push(Instruction::new("PUSH", "A", ""));
        compiled.push(Instruction::new("MOVE", &dest, "A"));
        compiled.push(Instruction::new("ADD", &reg, ""));
        compiled.push(Instruction::new("MOVE", "A", &dest));
        compiled.push(Instruction::new("POP", "A", ""));
    } else {
        compiled.push(Instruction::new("ADD", &reg, ""));
    }

    compiled
}

pub fn convert_sub(dest: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest != "A" {
        compiled.push(Instruction::new("PUSH", "A", ""));
        compiled.push(Instruction::new("MOVE", &dest, "A"));
        compiled.push(Instruction::new("SUB", &reg, ""));
        compiled.push(Instruction::new("MOVE", "A", &dest));
        compiled.push(Instruction::new("POP", "A", ""));
    } else {
        compiled.push(Instruction::new("SUB", &reg, ""));
    }

    compiled
}
