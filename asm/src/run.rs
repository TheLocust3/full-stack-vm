use parser;
use instruction;
use converter;
use compiler;
use subroutines;

pub fn run(program: &str) -> Vec<u8> {
    let mut instructions: Vec<instruction::Instruction> = Vec::new();

    instructions.append(&mut subroutines::beginning_subroutine());
    instructions.append(&mut parser::parse::parse(program));
    instructions.append(&mut subroutines::ending_subroutine());

    let converted = converter::convert::convert(instructions);
    compiler::compile::compile(converted)
}
