use log::{error};

use instruction::Instruction;
use compiler::miscellaneous;
use compiler::control;
use compiler::arithmetic;
use compiler::bitwise;

// compiles instructions in base assembly straight to binary

pub fn compile(instructions: Vec<Instruction>) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    for instruction in instructions {
        compiled.append(&mut compile_instruction(instruction));
    }

    compiled
}

pub fn compile_instruction(instruction: Instruction) -> Vec<u8> {
    match instruction.command.as_str() {
        "SET" => {
            Vec::new()
        },
        "MOVE" => {
            Vec::new()
        },
        "ADD" => {
            arithmetic::compile_add(instruction.arg1)
        },
        "SUB" => {
            arithmetic::compile_sub(instruction.arg1)
        },
        "AND" => {
            bitwise::compile_and(instruction.arg1)
        },
        "OR" => {
            bitwise::compile_or(instruction.arg1)
        },
        "NOT" => {
            bitwise::compile_not(instruction.arg1)
        },
        "SHIFT_LEFT" => {
            bitwise::compile_shift_left(instruction.arg1)
        },
        "SHIFT_LEFT_W" => {
            bitwise::compile_shift_left_wrap(instruction.arg1)
        },
        "SHIFT_RIGHT" => {
            bitwise::compile_shift_right(instruction.arg1)
        },
        "SHIFT_RIGHT_W" => {
            bitwise::compile_shift_right_wrap(instruction.arg1)
        },
        "JUMP" => {
            control::compile_jump(instruction.arg1)
        },
        "JUMP0" => {
            control::compile_jump0(instruction.arg1)
        },
        "NOP" => {
            miscellaneous::compile_nop()
        },
        _ => {
            error!("Instruction not handled!");
            Vec::new()
        }
    }
}
