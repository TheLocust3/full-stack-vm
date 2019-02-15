use log::{info, error};

use instruction::Instruction;
use converter::arithmetic;
use converter::bitwise;
use converter::register;

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
            Vec::new()
        },
        "POP" => {
            Vec::new()
        },
        "ADD" => {
            if instruction.arg1 == "A" || instruction.arg2.is_empty() {
                arithmetic::convert_add(instruction.arg1, instruction.arg2)
            } else {
                convert(arithmetic::convert_add(instruction.arg1, instruction.arg2))
            }
        },
        "SUB" => {
            if instruction.arg1 == "A" || instruction.arg2.is_empty() {
                arithmetic::convert_sub(instruction.arg1, instruction.arg2)
            } else {
                convert(arithmetic::convert_sub(instruction.arg1, instruction.arg2))
            }
        },
        "AND" => {
            if instruction.arg1 == "A" || instruction.arg2.is_empty() {
                bitwise::convert_and(instruction.arg1, instruction.arg2)
            } else {
                convert(bitwise::convert_and(instruction.arg1, instruction.arg2))
            }
        },
        "OR" => {
            if instruction.arg1 == "A" || instruction.arg2.is_empty() {
                bitwise::convert_or(instruction.arg1, instruction.arg2)
            } else {
                convert(bitwise::convert_or(instruction.arg1, instruction.arg2))
            }
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
        "SHIFT_RIGHT" => {
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
        "NOP" => {
            vec!(instruction)
        },
        _ => {
            error!("Instruction not handled!");
            Vec::new()
        }
    }
}
