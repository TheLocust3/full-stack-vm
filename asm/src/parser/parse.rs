use log::{info, error};

use instruction::Instruction;

pub fn parse(program: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    instructions
}

pub fn parse_line(line: String) -> Instruction {
    Instruction::new("", "", "")
}
