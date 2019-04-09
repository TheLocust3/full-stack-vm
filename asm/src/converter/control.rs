use instruction::Instruction;
use recognizers::is_register;
use recognizers::is_label;
use register::get_dest_reg;

pub fn convert_jump(value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if should_compile(&value) {
        compiled.push(Instruction::new("JUMP", &value, ""));
    } else {
        // temporarily use E register. Can't reset the value because of the jump
        compiled.push(Instruction::new("MOVE", "E", &value));

        compiled.push(Instruction::new("JUMP", "E", ""));
    }

    compiled
}

pub fn convert_jump0(value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if should_compile(&value) {
        compiled.push(Instruction::new("JUMP0", &value, ""));
    } else {
        // temporarily use E register. Can't reset the value because of the jump
        compiled.push(Instruction::new("MOVE", "E", &value));

        compiled.push(Instruction::new("JUMP0", "E", ""));
    }

    compiled
}

pub fn should_compile(value: &str) -> bool {
    is_register(&value.to_string())
}

pub fn convert_call(label: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if is_label(&label) {
        
    }

    compiled
}
