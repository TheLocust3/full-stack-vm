use log::{error};

use instruction::Instruction;
use compiler::miscellaneous;
use compiler::control;
use compiler::arithmetic;
use compiler::bitwise;
use compiler::register;
use compiler::memory;

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
            register::compile_set(instruction.arg1, instruction.arg2)
        },
        "MOVE" => {
            register::compile_move(instruction.arg1, instruction.arg2)
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
        "READ8" => {
            memory::compile_read8(instruction.arg1)
        },
        "READ16" => {
            memory::compile_read16(instruction.arg1)
        },
        "READ32" => {
            memory::compile_read32(instruction.arg1)
        },
        "READ64" => {
            memory::compile_read64(instruction.arg1)
        },
        "WRITE8" => {
            memory::compile_write8(instruction.arg1)
        },
        "WRITE16" => {
            memory::compile_write16(instruction.arg1)
        },
        "WRITE32" => {
            memory::compile_write32(instruction.arg1)
        },
        "WRITE64" => {
            memory::compile_write64(instruction.arg1)
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
