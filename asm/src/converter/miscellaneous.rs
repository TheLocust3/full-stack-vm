use instruction::Instruction;

pub fn convert_push_value(value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    // TODO: this is infinitely recursive
    compiled.push(Instruction::new("MOVE", "HL", "(0)"));
    compiled.push(Instruction::new("MOVE", "A", value));
    compiled.push(Instruction::new("WRITE64", "A", ""));

    compiled.push(Instruction::new("MOVE", "A", "4"));
    compiled.push(Instruction::new("ADD", "HL", ""));

    compiled.push(Instruction::new("MOVE", "HL", "0"));
    compiled.push(Instruction::new("WRITE64", A, ""));

    compiled
}

pub fn convert_push_reg(reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    // TODO: this is infinitely recursive
    compiled.push(Instruction::new("MOVE", "HL", "(0)"));
    compiled.push(Instruction::new("WRITE64", reg, ""));

    compiled.push(Instruction::new("MOVE", "A", "4"));
    compiled.push(Instruction::new("ADD", "HL", ""));
    
    compiled.push(Instruction::new("MOVE", "HL", "0"));
    compiled.push(Instruction::new("WRITE64", A, ""));

    compiled
}

pub fn convert_pop(dest: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    // TODO: this is infinitely recursive
    compiled.push(Instruction::new("MOVE", "A", "(0)"))
    compiled.push(Instruction::new("MOVE", "HL", "4"))
    compiled.push(Instruction::new("SUB", "HL", ""));

    compiled.push(Instruction::new("MOVE", "HL", "0"))
    compiled.push(Instruction::new("WRITE64", "A", ""))

    compiled.push(Instruction::new("MOVE", "HL", "A"));
    compiled.push(Instruction::new("READ64", dest, ""));

    compiled
}
