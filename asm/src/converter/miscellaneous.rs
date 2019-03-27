use instruction::Instruction;
use recognizers::is_register;
use recognizers::is_address;
use data::parse_address;
use converter::subroutines::temp_register;
use register::get_dest_reg;

pub fn should_compile(value: &String) -> bool {
    is_register(&value)
}

pub fn convert_push(value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if should_compile(&value) {
        compiled.push(Instruction::new("PUSH", &value, ""));
    } else {
        let dest_reg = get_dest_reg(&value);

        compiled.append(&mut temp_register::set_temp_register_to(&dest_reg));

        compiled.push(Instruction::new("MOVE", &dest_reg, &value));

        compiled.push(Instruction::new("PUSH", &dest_reg, ""));

        compiled.append(&mut temp_register::set_by_temp_register(&dest_reg));
    }

    compiled
}

pub fn convert_pop(value: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if should_compile(&value) {
        compiled.push(Instruction::new("POP", &value, ""));
    } else {
        let dest_reg = get_dest_reg(&value);

        compiled.append(&mut temp_register::set_temp_register_to(&dest_reg));

        compiled.push(Instruction::new("POP", &dest_reg, ""));

        compiled.push(Instruction::new("MOVE", &value, &dest_reg));

        compiled.append(&mut temp_register::set_by_temp_register(&dest_reg));
    }

    compiled
}
