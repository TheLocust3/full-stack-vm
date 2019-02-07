use log::{info, error};

use instruction::Instruction;
use compiler::miscellaneous;
use compiler::control;

pub fn convert(instructions: Vec<Instruction>) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    for instruction in instructions {
        compiled.append(&mut convert_instruction(instruction));
    }

    compiled
}

pub fn convert_instruction(instruction: Instruction) -> Vec<u8> {
    info!("Instruction: {}", instruction.to_string());

    match instruction.command.as_str() {
        "MOVE" => {
            Vec::new()
        },
        "ADD" => {
            Vec::new()
        },
        "SUB" => {
            Vec::new()
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
            Vec::new()
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
