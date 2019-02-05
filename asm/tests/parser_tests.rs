extern crate asm;

use self::asm::instruction::Instruction;
use self::asm::parser::parse;

#[test]
fn test_parse_line() {
    let line = "COMMAND ARG1 ARG2";

    let instruction = parse::parse_line(line);

    assert_eq!(instruction.command, "COMMAND");
    assert_eq!(instruction.arg1, "ARG1");
    assert_eq!(instruction.arg2, "ARG2");
}

#[test]
fn test_parse() {
    let program = "COMMAND ARG1 ARG2\nNOP";

    let instructions: Vec<Instruction> = parse::parse(program);

    assert_eq!(instructions.len(), 2);
    assert_eq!(instructions[0].command, "COMMAND");
    assert_eq!(instructions[0].arg1, "ARG1");
    assert_eq!(instructions[0].arg2, "ARG2");
    assert_eq!(instructions[1].command, "NOP");
    assert_eq!(instructions[1].arg1, "");
    assert_eq!(instructions[1].arg2, "");
}
