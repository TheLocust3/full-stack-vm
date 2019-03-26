use log::{info, error};

use instruction::Instruction;
use register::get_unused_reg;
use register::get_dest_reg;
use converter::arithmetic;
use converter::bitwise;
use converter::register;
use converter::miscellaneous;
use converter::control;

// converts instructions in complex assembly to base assembly

pub fn convert(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    for instruction in instructions {
        compiled.append(&mut convert_instruction(instruction));
    }

    compiled
}

pub fn convert_instruction(instruction: Instruction) -> Vec<Instruction> {
    info!("Instruction: {}", instruction.to_string());

    match instruction.command.as_str() {
        "MOVE" => {
            register::convert_move(instruction.arg1, instruction.arg2)
        },
        "PUSH" => {
            miscellaneous::convert_push(instruction.arg1)
        },
        "POP" => {
            miscellaneous::convert_pop(instruction.arg1)
        },
        "NOT" => {
            convert_one_arg_instruction(instruction)
        },
        "SHIFT_LEFT" => {
            convert_one_arg_instruction(instruction)
        },
        "SHIFT_LEFT_W" => {
            convert_one_arg_instruction(instruction)
        },
        "SHIFT_RIGHT" => {
            convert_one_arg_instruction(instruction)
        },
        "SHIFT_RIGHT_W" => {
            convert_one_arg_instruction(instruction)
        },
        "JUMP" => {
            if control::should_compile(&instruction.arg1) {
                control::convert_jump(instruction.arg1)
            } else {
                convert(control::convert_jump(instruction.arg1))
            }
        },
        "JUMP0" => {
            if control::should_compile(&instruction.arg1) {
                control::convert_jump0(instruction.arg1)
            } else {
                convert(control::convert_jump0(instruction.arg1))
            }
        },
        "ADD" => {
            convert_two_arg_instruction(instruction)
        },
        "SUB" => {
            convert_two_arg_instruction(instruction)
        },
        "AND" => {
            convert_two_arg_instruction(instruction)
        },
        "OR" => {
            convert_two_arg_instruction(instruction)
        },
        _ => {
            vec!(instruction)
        }
    }
}

fn convert_one_arg_instruction(instruction: Instruction) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    let dest_reg = get_dest_reg(&instruction.arg1);

    compiled.push(Instruction::new("PUSH", &dest_reg, ""));
    compiled.append(&mut convert_instruction(Instruction::new("MOVE", &dest_reg, &instruction.arg1)));

    compiled.push(match instruction.command.as_str() {
        "NOT" => {
            Instruction::new("NOT", &dest_reg, "")
        },
        "SHIFT_LEFT" => {
            Instruction::new("SHIFT_LEFT", &dest_reg, "")
        },
        "SHIFT_LEFT_W" => {
            Instruction::new("SHIFT_LEFT_W", &dest_reg, "")
        },
        "SHIFT_RIGHT" => {
            Instruction::new("SHIFT_RIGHT", &dest_reg, "")
        },
        "SHIFT_RIGHT_W" => {
            Instruction::new("SHIFT_RIGHT_W", &dest_reg, "")
        }
        _ => {
            instruction.clone()
        }
    });

    compiled.append(&mut convert_instruction(Instruction::new("MOVE", &instruction.arg1, &dest_reg)));
    compiled.push(Instruction::new("POP", &dest_reg, ""));

    compiled
}

fn convert_two_arg_instruction(instruction: Instruction) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    let input_reg = get_unused_reg("A", &instruction.arg1, &instruction.arg2);

    compiled.push(Instruction::new("PUSH", &input_reg, ""));
    compiled.append(&mut convert_instruction(Instruction::new("MOVE", &input_reg, &instruction.arg2)));
    // input_reg now has whatever value/address/register/address_register that was passed in
    
    compiled.append(&mut match instruction.command.as_str() {
        "ADD" => {
            if arithmetic::should_compile_add(&instruction.arg1, &input_reg) {
                arithmetic::convert_add(instruction.arg1, input_reg.clone())
            } else {
                convert(arithmetic::convert_add(instruction.arg1, input_reg.clone()))
            }
        },
        "SUB" => {
            if arithmetic::should_compile_sub(&instruction.arg1, &input_reg) {
                arithmetic::convert_sub(instruction.arg1, input_reg.clone())
            } else {
                convert(arithmetic::convert_sub(instruction.arg1, input_reg.clone()))
            }
        },
        "AND" => {
            if bitwise::should_compile_and(&instruction.arg1, &input_reg) {
                bitwise::convert_and(instruction.arg1, input_reg.clone())
            } else {
                convert(bitwise::convert_and(instruction.arg1, input_reg.clone()))
            }
        },
        "OR" => {
            if bitwise::should_compile_or(&instruction.arg1, &input_reg) {
                bitwise::convert_or(instruction.arg1, input_reg.clone())
            } else {
                convert(bitwise::convert_or(instruction.arg1, input_reg.clone()))
            }
        },
        _ => {
            vec!(instruction)
        }
    });

    compiled.push(Instruction::new("POP", &input_reg, ""));

    compiled
}
