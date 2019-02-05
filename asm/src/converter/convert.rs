use log::{info, error};

use instruction::Instruction;

pub fn convert(instructions: Vec<Instruction>) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    for instruction in instructions {
        compiled.append(&mut convert_instruction(instruction));
    }

    compiled
}

pub fn convert_instruction(instruction: Instruction) -> Vec<u8> {
    let mut compiled: Vec<u8> = Vec::new();

    info!("Instruction: {}", instruction.to_string());

    match instruction.command.as_str() {
        "MOVE" => {

        },
        "ADD" => {

        },
        "SUB" => {

        },
        "AND" => {

        },
        "OR" => {

        },
        "NOT" => {

        },
        "SHIFT_LEFT" => {

        },
        "SHIFT_LEFT_W" => {

        },
        "SHIFT_RIGHT" => {

        },
        "SHIFT_RIGHT_W" => {

        },
        "JUMP" => {

        },
        "JUMP0" => {

        },
        "NOP" => {
            compiled.push(0b00000000);
        },
        _ => {
            error!("Instruction not handled!");
        }
    }

    compiled
}
