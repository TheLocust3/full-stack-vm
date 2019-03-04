use parser;
use instruction;
use converter;
use compiler;

pub fn run(program: String) -> Vec<u8> {
    let mut instructions = parser::parse::parse(&program);
    instructions.push(instruction::Instruction::new("HALT", "", "")); // stop cpu after program is done

    let converted = converter::convert::convert(instructions);
    compiler::compile::compile(converted)
}
