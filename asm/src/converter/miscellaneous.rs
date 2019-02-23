use instruction::Instruction;
use recognizers::is_register;

pub fn convert_push(value: String) -> Vec<Instruction> {
    if is_register(&value) {
        vec!(Instruction::new("PUSH", &value, ""))
    } else {
        convert_push_value(value)
    }
}

pub fn convert_push_value(value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("PUSH", "A", ""));
    compiled.push(Instruction::new("MOVE", "A", &value));

    compiled.push(Instruction::new("PUSH", "A", ""));

    compiled.push(Instruction::new("POP", "A", ""));

    compiled
}
