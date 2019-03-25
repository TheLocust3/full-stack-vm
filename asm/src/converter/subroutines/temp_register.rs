use instruction::Instruction;
use data::parse_address;

const TEMPORARY_REGISTER: u64 = 320;

pub fn set_temp_register_to(register: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    instructions.push(Instruction::new("PUSH", "HL", ""));

    instructions.push(Instruction::new("SET", "HL", &TEMPORARY_REGISTER.to_string()));
    instructions.push(Instruction::new("WRITE64", &register, ""));

    instructions.push(Instruction::new("POP", "HL", ""));

    instructions
}

pub fn set_by_temp_register(register: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    instructions.push(Instruction::new("PUSH", "HL", ""));

    instructions.push(Instruction::new("SET", "HL", &TEMPORARY_REGISTER.to_string()));
    instructions.push(Instruction::new("READ64", &register, ""));

    instructions.push(Instruction::new("POP", "HL", ""));

    instructions
}
