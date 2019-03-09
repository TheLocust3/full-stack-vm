use instruction::Instruction;

pub fn convert_and(dest: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest == "A" {
        compiled.push(Instruction::new("AND", &reg, ""));
    } else if dest != "A" && reg != "A" {
        println!("TEST");
        compiled.push(Instruction::new("PUSH", "A", ""));

        compiled.push(Instruction::new("MOVE", "A", &dest));
        compiled.push(Instruction::new("AND", "A", &reg));
        compiled.push(Instruction::new("MOVE", &dest, "A"));

        compiled.push(Instruction::new("POP", "A", ""));
    } else if dest != "A" && reg == "A" {
        compiled.push(Instruction::new("PUSH", &reg, ""));

        compiled.push(Instruction::new("AND", &reg, &dest));
        compiled.push(Instruction::new("MOVE", &dest, &reg));

        compiled.push(Instruction::new("POP", &reg, ""));
    } 

    compiled
}

pub fn should_compile_and(dest: String, reg: String) -> bool {
    dest == "A"
}

pub fn convert_or(dest: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest == "A" {
        compiled.push(Instruction::new("OR", &reg, ""));
    } else if dest != "A" && reg != "A" {
        compiled.push(Instruction::new("PUSH", "A", ""));
        
        compiled.push(Instruction::new("MOVE", "A", &dest));
        compiled.push(Instruction::new("OR", "A", &reg));
        compiled.push(Instruction::new("MOVE", &dest, "A"));

        compiled.push(Instruction::new("POP", "A", ""));
    } else if dest != "A" && reg == "A" {
        compiled.push(Instruction::new("PUSH", &reg, ""));

        compiled.push(Instruction::new("OR", &reg, &dest));
        compiled.push(Instruction::new("MOVE", &dest, &reg));

        compiled.push(Instruction::new("POP", &reg, ""));
    } 

    compiled
}

pub fn should_compile_or(dest: String, reg: String) -> bool {
    dest == "A"
}
