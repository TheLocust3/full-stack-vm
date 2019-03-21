use instruction::Instruction;
use recognizers::is_register;
use data::parse_address;

pub fn get_unused_reg(used_reg1: &str, used_reg2: &str, used_reg3: &str) -> String {
    let reg1 = parse_address(used_reg1.to_string());
    let reg2 = parse_address(used_reg2.to_string());
    let reg3 = parse_address(used_reg3.to_string());

    if reg1 != "A" && reg2 != "A" && reg3 != "A" {
        "A".to_string()
    } else if reg1 != "B" && reg2 != "B" && reg3 != "B" {
        "B".to_string()
    } else if reg1 != "C" && reg2 != "C" && reg3 != "C" {
        "C".to_string()
    } else {
        "D".to_string()
    }
}

pub fn convert_add(dest: String, input: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest == "A" && is_register(&input) {
        compiled.push(Instruction::new("ADD", &input, ""));
    } else {
        let input_reg = get_unused_reg("A", &dest, &input);

        compiled.push(Instruction::new("PUSH", &input_reg, ""));
        compiled.push(Instruction::new("MOVE", &input_reg, &input));

        if dest == "A" {
            compiled.push(Instruction::new("ADD", "A", &input_reg));
        } else {
            compiled.push(Instruction::new("PUSH", "A", ""));
            compiled.push(Instruction::new("MOVE", "A", &dest));

            compiled.push(Instruction::new("ADD", "A", &input_reg));

            compiled.push(Instruction::new("MOVE", &dest, "A"));
            compiled.push(Instruction::new("POP", "A", ""));
        }

        compiled.push(Instruction::new("POP", &input_reg, ""));
    }

    compiled
}

pub fn should_compile_add(dest: String, input: String) -> bool {
    dest == "A" && is_register(&input)
}

pub fn convert_sub(dest: String, input: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    if dest == "A" && is_register(&input) {
        compiled.push(Instruction::new("SUB", &input, ""));
    } else {
        let input_reg = get_unused_reg("A", &dest, &input);

        compiled.push(Instruction::new("PUSH", &input_reg, ""));
        compiled.push(Instruction::new("MOVE", &input_reg, &input));

        if dest == "A" {
            compiled.push(Instruction::new("SUB", "A", &input_reg));
        } else {
            compiled.push(Instruction::new("PUSH", "A", ""));
            compiled.push(Instruction::new("MOVE", "A", &dest));

            compiled.push(Instruction::new("SUB", "A", &input_reg));

            compiled.push(Instruction::new("MOVE", &dest, "A"));
            compiled.push(Instruction::new("POP", "A", ""));
        }

        compiled.push(Instruction::new("POP", &input_reg, ""));
    }

    compiled
}

pub fn should_compile_sub(dest: String, input: String) -> bool {
    dest == "A" && is_register(&input)
}
