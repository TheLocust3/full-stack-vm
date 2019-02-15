use instruction::Instruction;

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
}

fn convert_move_reg_address(dest_reg: String, address: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled
}

fn convert_move_reg_reg(dest_reg: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled
}

fn convert_move_reg_value(dest_reg: String, value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled
}

fn convert_move_address_reg(dest_address: String, reg: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled
}

fn convert_move_address_address(dest_address: String, address: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled
}

fn convert_move_address_value(dest_address: String, value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled
}

fn is_register(arg: &String) -> bool {
    arg == "A" || arg == "B" || arg == "C" || arg == "D" || arg == "E"
}

fn is_address(arg: &String) -> bool {
    let first = arg.chars().next().unwrap();
    let last = arg.chars().last().unwrap();

    first == '(' && last  == ')'
}

fn is_value(arg: &String) -> bool {
    !is_register(&arg) && !is_address(&arg)
}
