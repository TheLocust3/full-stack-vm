use std::process;
use log::{info, error};

use instruction::InstructionTree;
use instruction::Instruction;

pub fn parse(program: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    // TODO: Eventually this scrubbing might be an issue with string literals
    for line in program.replace("  ", " ").replace("   ", " ").split('\n') {
        if !line.is_empty() {
            instructions.append(&mut parse_line(line.to_string()));
        }
    }

    instructions
}

pub fn parse_line(line: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    // create tree structure and parse instructions
    for instruction in line.split('(') {
        // create new instruction
        // check if at '('
            // re-run parse
            // else first is instruction name
        // check if at ')'
            // second is first instruction arg
            // else stop
        // check if at ')'
            // third is second instruction arg
            // else stop
    }

    instructions
}
