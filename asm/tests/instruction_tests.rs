extern crate asm;

use self::asm::instruction::Instruction;

#[test]
fn test_new_instruction() {
    let instruction = Instruction::new("COMMAND", "ARG1", "ARG2");

    assert_eq!(instruction.command, "COMMAND");
    assert_eq!(instruction.arg1, "ARG1");
    assert_eq!(instruction.arg2, "ARG2");
}

#[test]
fn test_instruction_to_string() {
    let instruction = Instruction::new("COMMAND", "ARG1", "ARG2");

    assert_eq!(instruction.to_string(), "COMMAND ARG1 ARG2");
}
