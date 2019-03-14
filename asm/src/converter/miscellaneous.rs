use instruction::Instruction;
use recognizers::is_register;
use recognizers::is_address;
use data::parse_address;
use subroutines::TEMPORARY_REGISTER;

pub fn convert_push(value: String) -> Vec<Instruction> {
    if is_register(&value) {
        vec!(Instruction::new("PUSH", &value, ""))
    } else if is_address(&value) {
        convert_push_addr(value)
    } else {
        convert_push_value(value)
    }
}

// TODO: Currently value in HL register is destroyed by the following two functions

pub fn convert_push_addr(address: String) -> Vec<Instruction> {
    let address_val = parse_address(address);

    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("SET", "HL", &TEMPORARY_REGISTER.to_string()));
    compiled.push(Instruction::new("WRITE64", "A", ""));

    compiled.push(Instruction::new("SET", "HL", &address_val));
    compiled.push(Instruction::new("READ64", "A", ""));

    compiled.push(Instruction::new("PUSH", "A", ""));

    compiled.push(Instruction::new("SET", "HL", &TEMPORARY_REGISTER.to_string()));
    compiled.push(Instruction::new("READ64", "A", ""));

    compiled
}

pub fn convert_push_value(value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("SET", "HL", &TEMPORARY_REGISTER.to_string()));
    compiled.push(Instruction::new("WRITE64", "A", ""));

    compiled.push(Instruction::new("SET", "A", &value));

    compiled.push(Instruction::new("PUSH", "A", ""));

    compiled.push(Instruction::new("SET", "HL", &TEMPORARY_REGISTER.to_string()));
    compiled.push(Instruction::new("READ64", "A", ""));

    compiled
}
