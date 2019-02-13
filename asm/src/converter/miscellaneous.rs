use instruction::Instruction;

pub fn convert_push_reg(reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("", "", ""));

    compiled
}

pub fn convert_push_value(value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("", "", ""));

    compiled
}

pub fn convert_pop(dest: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("", "", ""));

    compiled
}
