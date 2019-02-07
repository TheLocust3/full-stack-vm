use log::{error};

use instruction::Instruction;
use compiler::miscellaneous;
use compiler::control;
use compiler::arithmetic;

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
            Vec::new()
        },
        "OR" => {
            Vec::new()
        },
        "NOT" => {
            Vec::new()
        },
        "SHIFT_LEFT" => {
            Vec::new()
        },
        "SHIFT_LEFT_W" => {
            Vec::new()
        },
        "SHIFT_RIGHT" => {
            Vec::new()
        },
        "SHIFT_RIGHT_W" => {
            Vec::new()
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
