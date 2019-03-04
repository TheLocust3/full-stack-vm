use log::{info, error};

use instruction::Instruction;
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
            register::convert_move(instruction.arg1, instruction.arg2)
        },
        "PUSH" => {
            miscellaneous::convert_push(instruction.arg1)
        },
        "ADD" => {
            if arithmetic::should_reconvert_add(arg1, arg2) {
                convert(arithmetic::convert_add(instruction.arg1, instruction.arg2))
            } else {
                arithmetic::convert_add(instruction.arg1, instruction.arg2)
            }
        },
        "SUB" => {
            if arithmetic::should_reconvert_sub(arg1, arg2) {
                convert(arithmetic::convert_sub(instruction.arg1, instruction.arg2))
            } else {
                arithmetic::convert_sub(instruction.arg1, instruction.arg2)
            }
        },
        "AND" => {
            if bitwise::should_reconvert_and(arg1, arg2) {
                convert(bitwise::convert_and(instruction.arg1, instruction.arg2))
            } else {
                bitwise::convert_and(instruction.arg1, instruction.arg2)
            }
        },
        "OR" => {
            if bitwise::should_reconvert_or(arg1, arg2) {
                convert(bitwise::convert_or(instruction.arg1, instruction.arg2))
            } else {
                bitwise::convert_or(instruction.arg1, instruction.arg2)
            }
        },
        _ => {
            vec!(instruction)
        }
    }
}
