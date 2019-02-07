extern crate asm;

use self::asm::instruction;
use self::asm::converter::convert;

#[test]
fn test_convert_jump() {
    let instruction = instruction::Instruction::new("JUMP", "10", "");

    let compiled = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 9);
    assert_eq!(compiled[0], 0b11111111);
    assert_eq!(compiled[1], 0);
    assert_eq!(compiled[2], 0);
    assert_eq!(compiled[3], 0);
    assert_eq!(compiled[4], 0);
    assert_eq!(compiled[5], 0);
    assert_eq!(compiled[6], 0);
    assert_eq!(compiled[7], 0);
    assert_eq!(compiled[8], 10);
}

#[test]
fn test_convert_jump0() {
    let instruction = instruction::Instruction::new("JUMP0", "10", "");

    let compiled = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 9);
    assert_eq!(compiled[0], 0b111111110);
    assert_eq!(compiled[1], 0);
    assert_eq!(compiled[2], 0);
    assert_eq!(compiled[3], 0);
    assert_eq!(compiled[4], 0);
    assert_eq!(compiled[5], 0);
    assert_eq!(compiled[6], 0);
    assert_eq!(compiled[7], 0);
    assert_eq!(compiled[8], 10);
}
