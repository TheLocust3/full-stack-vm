extern crate asm;

use self::asm::instruction;
use self::asm::converter::convert;

#[test]
fn test_convert_nop() {
    let instruction = instruction::Instruction::new("NOP", "", "");

    let compiled = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000000);
}
