use log::{info, error};

use instruction::Instruction;
use register::get_unused_reg;
use converter::arithmetic;
use converter::bitwise;
use converter::register;
use converter::miscellaneous;

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

    let arg1 = instruction.arg1.clone();
    let arg2 = instruction.arg2.clone();

    match instruction.command.as_str() {
        "MOVE" => {
            // TODO: Convert MOVE into convert_two_arg_instruction
            register::convert_move(instruction.arg1, instruction.arg2)
        },
        // TODO: Allow anything to be passed into these
        "PUSH" => {
            miscellaneous::convert_push(instruction.arg1)
        },
        "POP" => {
            vec!(instruction)
        },
        "NOT" => {
            vec!(instruction)
        },
        "SHIFT_LEFT" => {
            vec!(instruction)
        },
        "SHIFT_LEFT_W" => {
            vec!(instruction)
        },
        "SHIFT_RIGHT_W" => {
            vec!(instruction)
        },
        "SHIFT_RIGHT_W" => {
            vec!(instruction)
        },
        "JUMP" => {
            vec!(instruction)
        },
        "JUMP0" => {
            vec!(instruction)
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

pub fn convert_two_arg_instruction(instruction: Instruction) -> Vec<Instruction> {
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
