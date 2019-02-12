extern crate asm;

use self::asm::instruction::Instruction;
use self::asm::converter::convert;

#[test]
fn test_convert_add() {
    let instruction = Instruction::new("ADD", "B", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 5);
}

#[test]
fn test_convert_sub() {
    let instruction = Instruction::new("SUB", "B", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 5);
}
