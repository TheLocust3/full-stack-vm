use instruction::Instruction;
use recognizers::is_register;
use recognizers::is_address_value;
use recognizers::is_address_register;
use recognizers::is_value;
use data::parse_address;
use converter::subroutines::address;

pub fn convert_move(arg1: String, arg2: String) -> Vec<Instruction> {
    if is_register(&arg1) && is_address_value(&arg2) { // MOVE A (10)
        convert_move_reg_address(arg1, arg2)
    } else if is_register(&arg1) && is_register(&arg2) { // MOVE A B
        convert_move_reg_reg(arg1, arg2)
    } else if is_register(&arg1) && is_value(&arg2) { // MOVE A 1
        convert_move_reg_value(arg1, arg2)
    } else if is_register(&arg1) && is_address_register(&arg2) { // MOVE A (B)
        convert_move_reg_addr_reg(arg1, arg2)
    } else if is_address_value(&arg1) && is_register(&arg2) { // MOVE (10) A
        convert_move_address_reg(arg1, arg2)
    } else if is_address_value(&arg1) && is_address_value(&arg2) { // MOVE (10) (11)
        convert_move_address_address(arg1, arg2)
    } else if is_address_value(&arg1) && is_address_register(&arg2) { // MOVE (10) (A)
        convert_move_address_addr_reg(arg1, arg2)
    } else if is_address_value(&arg1) && is_value(&arg2) { // MOVE (10) 1
        convert_move_address_value(arg1, arg2)
    } else if is_address_register(&arg1) && is_address_value(&arg2) { // MOVE (A) (10)
        convert_move_addr_reg_address(arg1, arg2)
    } else if is_address_register(&arg1) && is_value(&arg2) { // MOVE (A) 1
        convert_move_addr_reg_value(arg1, arg2)
    } else if is_address_register(&arg1) && is_register(&arg2) { // MOVE (A) B
        convert_move_addr_reg_reg(arg1, arg2)
    } else {
        Vec::new()
    }
}

fn convert_move_reg_address(dest_reg: String, address: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.append(&mut address::push_address(address));

    compiled.push(Instruction::new("READ64", &dest_reg, ""));

    compiled.append(&mut address::teardown_address());

    compiled
}

fn convert_move_reg_addr_reg(dest_reg: String, addr_reg: String) -> Vec<Instruction> {
    let addr_reg_val = parse_address(addr_reg);

    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("PUSH", "HL", ""));
    compiled.push(Instruction::new("MOVE", "HL", &addr_reg_val));
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

    compiled.append(&mut address::push_address(dest_address));

    compiled.push(Instruction::new("WRITE64", &reg, ""));

    compiled.append(&mut address::teardown_address());

    compiled
}

fn convert_move_addr_reg_reg(dest_addr_reg: String, reg: String) -> Vec<Instruction> {
    let addr_reg_val = parse_address(dest_addr_reg);

    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("PUSH", "HL", ""));
    compiled.push(Instruction::new("MOVE", "HL", &addr_reg_val));
    compiled.push(Instruction::new("WRITE64", &reg, ""));
    compiled.push(Instruction::new("POP", "HL", ""));

    compiled
}

fn convert_move_address_address(dest_address: String, address: String) -> Vec<Instruction> {
    let dest_address_val = parse_address(dest_address);

    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.append(&mut address::push_address(address));

    compiled.push(Instruction::new("PUSH", "A", ""));
    compiled.push(Instruction::new("READ64", "A", ""));
    // A has value to be placed in dest_address

    compiled.push(Instruction::new("SET", "HL", &dest_address_val));
    compiled.push(Instruction::new("WRITE64", "A", ""));
    // value written to dest_address

    compiled.push(Instruction::new("POP", "A", ""));

    compiled.append(&mut address::teardown_address());

    compiled
}

fn convert_move_addr_reg_address(dest_addr_reg: String, address: String) -> Vec<Instruction> {
    let addr_reg_val = parse_address(dest_addr_reg);

    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.append(&mut address::push_address(address));

    if addr_reg_val == "A" {
        compiled.push(Instruction::new("PUSH", "B", ""));
        compiled.push(Instruction::new("READ64", "B", ""));
        // A has value to be placed at dest_addr_reg

        compiled.push(Instruction::new("MOVE", "HL", &addr_reg_val));
        compiled.push(Instruction::new("WRITE64", "B", ""));
        // value written to dest_addr_reg

        compiled.push(Instruction::new("POP", "B", ""));
    } else {
        compiled.push(Instruction::new("PUSH", "A", ""));
        compiled.push(Instruction::new("READ64", "A", ""));
        // A has value to be placed at dest_addr_reg

        compiled.push(Instruction::new("MOVE", "HL", &addr_reg_val));
        compiled.push(Instruction::new("WRITE64", "A", ""));
        // value written to dest_addr_reg

        compiled.push(Instruction::new("POP", "A", ""));
    }

    compiled.append(&mut address::teardown_address());

    compiled
}

fn convert_move_address_addr_reg(dest_address: String, addr_reg: String) -> Vec<Instruction> {
    let dest_address_val = parse_address(dest_address);
    let addr_reg_val = parse_address(addr_reg);

    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("PUSH", "HL", ""));
    compiled.push(Instruction::new("MOVE", "HL", &addr_reg_val));
    // HL set to address in addr_reg

    if addr_reg_val == "A" {
        compiled.push(Instruction::new("PUSH", "B", ""));
        compiled.push(Instruction::new("READ64", "B", ""));
        // A has value to be placed in dest_address

        compiled.push(Instruction::new("SET", "HL", &dest_address_val));
        compiled.push(Instruction::new("WRITE64", "B", ""));
        // value written to dest_address

        compiled.push(Instruction::new("POP", "B", ""));
    } else {
        compiled.push(Instruction::new("PUSH", "A", ""));
        compiled.push(Instruction::new("READ64", "A", ""));
        // A has value to be placed in dest_address

        compiled.push(Instruction::new("SET", "HL", &dest_address_val));
        compiled.push(Instruction::new("WRITE64", "A", ""));
        // value written to dest_address

        compiled.push(Instruction::new("POP", "A", ""));
    }

    compiled.push(Instruction::new("POP", "HL", ""));

    compiled
}

fn convert_move_address_value(dest_address: String, value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.append(&mut address::push_address(dest_address));

    compiled.push(Instruction::new("PUSH", "A", ""));
    compiled.push(Instruction::new("SET", "A", &value));

    compiled.push(Instruction::new("WRITE64", "A", ""));

    compiled.push(Instruction::new("POP", "A", ""));

    compiled.append(&mut address::teardown_address());

    compiled
}

fn convert_move_addr_reg_value(dest_addr_reg: String, value: String) -> Vec<Instruction> {
    let addr_reg_val = parse_address(dest_addr_reg);

    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.push(Instruction::new("PUSH", "HL", ""));
    compiled.push(Instruction::new("MOVE", "HL", &addr_reg_val));

    println!("TEST! {}", addr_reg_val);
    if addr_reg_val == "A" {
        compiled.push(Instruction::new("PUSH", "B", ""));
        compiled.push(Instruction::new("SET", "B", &value));

        compiled.push(Instruction::new("WRITE64", "B", ""));

        compiled.push(Instruction::new("POP", "B", ""));
    } else {
        compiled.push(Instruction::new("PUSH", "A", ""));
        compiled.push(Instruction::new("SET", "A", &value));

        compiled.push(Instruction::new("WRITE64", "A", ""));

        compiled.push(Instruction::new("POP", "A", ""));
    }

    compiled.push(Instruction::new("POP", "HL", ""));

    compiled
}
