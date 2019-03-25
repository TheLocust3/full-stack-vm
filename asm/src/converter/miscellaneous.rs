use instruction::Instruction;
use recognizers::is_register;
use recognizers::is_address;
use data::parse_address;
use converter::subroutines::temp_register;

pub fn convert_push(value: String) -> Vec<Instruction> {
    if is_register(&value) {
        vec!(Instruction::new("PUSH", &value, ""))
    } else if is_address(&value) {
        convert_push_addr(value)
    } else {
        convert_push_value(value)
    }
}

// TODO: Allow use of address registers with push/pop

pub fn convert_push_addr(address: String) -> Vec<Instruction> {
    let address_val = parse_address(address);

    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.append(&mut temp_register::set_temp_register_to("A"));

    compiled.push(Instruction::new("PUSH", "HL", ""));

    compiled.push(Instruction::new("SET", "HL", &address_val));
    compiled.push(Instruction::new("READ64", "A", ""));

    compiled.push(Instruction::new("POP", "HL", ""));

    compiled.push(Instruction::new("PUSH", "A", ""));

    compiled.append(&mut temp_register::set_by_temp_register("A"));

    compiled
}

pub fn convert_push_value(value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.append(&mut temp_register::set_temp_register_to("A"));

    compiled.push(Instruction::new("SET", "A", &value));

    compiled.push(Instruction::new("PUSH", "A", ""));

    compiled.append(&mut temp_register::set_by_temp_register("A"));

    compiled
}

pub fn convert_pop(value: String) -> Vec<Instruction> {
    if is_address(&value) {
        convert_pop_addr(value)
    } else { // default to just try
        vec!(Instruction::new("POP", &value, ""))
    }
}

pub fn convert_pop_addr(address: String) -> Vec<Instruction> {
    let address_val = parse_address(address);

    let mut compiled: Vec<Instruction> = Vec::new();

    compiled.append(&mut temp_register::set_temp_register_to("A"));

    compiled.push(Instruction::new("POP", "A", ""));

    compiled.push(Instruction::new("PUSH", "HL", ""));

    compiled.push(Instruction::new("SET", "HL", &address_val));
    compiled.push(Instruction::new("WRITE64", "A", ""));

    compiled.push(Instruction::new("POP", "HL", ""));

    compiled.append(&mut temp_register::set_by_temp_register("A"));

    compiled
}
