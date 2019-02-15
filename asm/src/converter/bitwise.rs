use instruction::Instruction;

pub fn convert_and(dest: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest != "A" {
        compiled.push(Instruction::new("PUSH", "A", ""));
        compiled.push(Instruction::new("MOVE", &dest, "A"));
        compiled.push(Instruction::new("AND", &reg, ""));
        compiled.push(Instruction::new("MOVE", "A", &dest));
        compiled.push(Instruction::new("POP", "A", ""));
    } else {
        compiled.push(Instruction::new("AND", &reg, ""));
    }

    compiled
}

pub fn should_reconvert_and(dest: String, reg: String) -> bool {
    dest != "A" && reg.is_empty()
}

pub fn convert_or(dest: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest != "A" {
        compiled.push(Instruction::new("PUSH", "A", ""));
        compiled.push(Instruction::new("MOVE", &dest, "A"));
        compiled.push(Instruction::new("OR", &reg, ""));
        compiled.push(Instruction::new("MOVE", "A", &dest));
        compiled.push(Instruction::new("POP", "A", ""));
    } else {
        compiled.push(Instruction::new("OR", &reg, ""));
    }

    compiled
}

pub fn should_reconvert_or(dest: String, reg: String) -> bool {
    dest != "A" && reg.is_empty()
}
