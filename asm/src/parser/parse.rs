use std::process;
use log::{info, error};

use instruction::Instruction;

pub fn parse(program: &str) -> Vec<Instruction> {
    // TODO: Clean up file, remove trailing newlines, extra spaces, etc
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in program.split('\n') {
        if !line.is_empty() {
            instructions.push(parse_line(line));
        }
    }

    instructions
}

pub fn parse_line(line: &str) -> Instruction {
    let tokens: Vec<&str> = line.split(' ').collect();

    if tokens.len() > 3 {
        error!("Too many arguments in line");
        process::exit(1);
    } else if tokens.len() <= 0 {
        error!("Too few arguments in line");
        process::exit(1);
    }
    
    if tokens.len() == 1 {
        Instruction::new(tokens[0], "", "")
    } else if tokens.len() == 2 {
        Instruction::new(tokens[0], tokens[1], "")
    } else {
        Instruction::new(tokens[0], tokens[1], tokens[2])
    }
}
