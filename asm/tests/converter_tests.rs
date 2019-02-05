extern crate asm;

use self::asm::instruction::Instruction;
use self::asm::converter::convert;

#[test]
fn test_convert_nop() {
    let mut instructions: Vec<Instruction> = Vec::new();
    instructions.push(Instruction::new("NOP", "", ""));

    let compiled = convert::convert(instructions);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000000);
}
