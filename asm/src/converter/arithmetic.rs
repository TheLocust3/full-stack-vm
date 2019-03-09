use instruction::Instruction;

pub fn convert_add(dest: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest == "A" {
        compiled.push(Instruction::new("ADD", &reg, ""));
    } else if dest != "A" && reg != "A" {
        compiled.push(Instruction::new("PUSH", "A", ""));
        
        compiled.push(Instruction::new("MOVE", "A", &dest));
        compiled.push(Instruction::new("ADD", "A", &reg));
        compiled.push(Instruction::new("MOVE", &dest, "A"));

        compiled.push(Instruction::new("POP", "A", ""));
    } else if dest != "A" && reg == "A" {
        compiled.push(Instruction::new("PUSH", &reg, ""));

        compiled.push(Instruction::new("ADD", &reg, &dest));
        compiled.push(Instruction::new("MOVE", &dest, &reg));

        compiled.push(Instruction::new("POP", &reg, ""));
    }

    compiled
}

pub fn should_compile_add(dest: String, reg: String) -> bool {
    dest == "A"
}

pub fn convert_sub(dest: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest == "A" {
        compiled.push(Instruction::new("SUB", &reg, ""));
    } else if dest != "A" && reg != "A" {
        compiled.push(Instruction::new("PUSH", "A", ""));
        
        compiled.push(Instruction::new("MOVE", "A", &dest));
        compiled.push(Instruction::new("SUB", "A", &reg));
        compiled.push(Instruction::new("MOVE", &dest, "A"));
        
        compiled.push(Instruction::new("POP", "A", ""));
    } else if dest != "A" && reg == "A" {
        compiled.push(Instruction::new("PUSH", &reg, ""));

        compiled.push(Instruction::new("MOVE", &reg, &dest));
        compiled.push(Instruction::new("POP", &dest, ""));
        compiled.push(Instruction::new("PUSH", &dest, ""));
        
        compiled.push(Instruction::new("SUB", &reg, &dest));
        compiled.push(Instruction::new("MOVE", &dest, &reg));

        compiled.push(Instruction::new("POP", &reg, ""));
    }

    compiled
}

pub fn should_compile_sub(dest: String, reg: String) -> bool {
    dest == "A"
}
