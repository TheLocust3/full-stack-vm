use instruction::Instruction;
use recognizers::is_register;
use recognizers::is_address;
use recognizers::is_value;

pub fn convert_move(arg1: String, arg2: String) -> Vec<Instruction> {
    if is_register(&arg1) && is_address(&arg2) { // MOVE A (10)
        convert_move_reg_address(arg1, arg2)
    } else if is_register(&arg1) && is_register(&arg2) { // MOVE A B
        convert_move_reg_reg(arg1, arg2)
    } else if is_register(&arg1) && is_value(&arg2) { // MOVE A 1
        convert_move_reg_value(arg1, arg2)
    } else if is_address(&arg1) && is_register(&arg2) { // MOVE (10) A
        convert_move_address_reg(arg1, arg2)
    } else if is_address(&arg1) && is_address(&arg2) { // MOVE (10) (11)
        convert_move_address_address(arg1, arg2)
    } else if is_address(&arg1) && is_value(&arg2) { // MOVE (10) 1
        convert_move_address_value(arg1, arg2)
    } else {
        Vec::new()
    }
    // TODO: Add using registers as addresses
}

fn convert_move_reg_address(dest_reg: String, address: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("PUSH", "HL", ""));
    compiled.push(Instruction::new("SET", "HL", &address));
    compiled.push(Instruction::new("READ64", &dest_reg, ""));
    compiled.push(Instruction::new("POP", "HL", ""));

    compiled
}

fn convert_move_reg_reg(dest_reg: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("MOVE", &dest_reg, &reg));

    compiled
}

fn convert_move_reg_value(dest_reg: String, value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("SET", &dest_reg, &value));

    compiled
}

fn convert_move_address_reg(dest_address: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("PUSH", "HL", ""));
    compiled.push(Instruction::new("SET", "HL", &dest_address));
    compiled.push(Instruction::new("WRITE64", &reg, ""));
    compiled.push(Instruction::new("POP", "HL", ""));

    compiled
}

fn convert_move_address_address(dest_address: String, address: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("PUSH", "HL", ""));
    compiled.push(Instruction::new("SET", "HL", &address));
    // HL set to address

    compiled.push(Instruction::new("PUSH", "A", ""));
    compiled.push(Instruction::new("READ64", "A", ""));
    // A has value to be placed in dest_address

    compiled.push(Instruction::new("SET", "HL", &dest_address));
    compiled.push(Instruction::new("WRITE64", "A", ""));
    // value written to dest_address

    compiled.push(Instruction::new("POP", "A", ""));
    compiled.push(Instruction::new("POP", "HL", ""));

    compiled
}

fn convert_move_address_value(dest_address: String, value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("PUSH", "HL", ""));
    compiled.push(Instruction::new("SET", "HL", &dest_address));

    compiled.push(Instruction::new("PUSH", "A", ""));
    compiled.push(Instruction::new("SET", "A", &value));

    compiled.push(Instruction::new("WRITE64", "A", ""));

    compiled.push(Instruction::new("POP", "A", ""));
    compiled.push(Instruction::new("POP", "HL", ""));

    compiled
}
