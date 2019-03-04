use instruction::Instruction;
use recognizers::is_register;
use recognizers::is_address;

pub fn convert_push(value: String) -> Vec<Instruction> {
    if is_register(&value) {
        vec!(Instruction::new("PUSH", &value, ""))
    } else if is_address(&value) {
        convert_push_addr(value)
    } else {
        convert_push_value(value)
    }
}

pub fn convert_push_addr(address: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("PUSH", "HL", ""));
    compiled.push(Instruction::new("MOVE", "HL", &address));

    compiled.push(Instruction::new("PUSH", "A", ""));
    compiled.push(Instruction::new("READ64", "A", ""));

    compiled.push(Instruction::new("PUSH", "A", ""));

    compiled.push(Instruction::new("POP", "A", ""));
    compiled.push(Instruction::new("POP", "HL", ""));

    compiled
}

pub fn convert_push_value(value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("PUSH", "A", ""));
    compiled.push(Instruction::new("SET", "A", &value));

    compiled.push(Instruction::new("PUSH", "A", ""));

    compiled.push(Instruction::new("POP", "A", ""));

    compiled
}
