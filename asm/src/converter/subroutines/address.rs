use instruction::Instruction;
use data::parse_address;

pub fn push_address(address: String) -> Vec<Instruction> {
    let address_val = parse_address(address);

    let mut instructions: Vec<Instruction> = Vec::new();

    instructions.push(Instruction::new("PUSH", "HL", ""));
    instructions.push(Instruction::new("SET", "HL", &address_val));

    instructions
}

pub fn teardown_address() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    instructions.push(Instruction::new("POP", "HL", ""));

    instructions
}
