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
fn test_convert_add_a() {
    let instruction = Instruction::new("ADD", "A", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 1);
}

#[test]
fn test_convert_sub() {
    let instruction = Instruction::new("SUB", "B", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 5);
}

#[test]
fn test_convert_sub_a() {
    let instruction = Instruction::new("SUB", "A", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 1);
}

#[test]
fn test_convert_not() {
    let instruction = Instruction::new("NOT", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "NOT");
    assert_eq!(compiled[0].arg1, "A");
    assert_eq!(compiled[0].arg2, "");
}

#[test]
fn test_convert_shift_left() {
    let instruction = Instruction::new("SHIFT_LEFT", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "SHIFT_LEFT");
    assert_eq!(compiled[0].arg1, "A");
    assert_eq!(compiled[0].arg2, "");
}

#[test]
fn test_convert_shift_left_wrap() {
    let instruction = Instruction::new("SHIFT_LEFT_W", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "SHIFT_LEFT_W");
    assert_eq!(compiled[0].arg1, "A");
    assert_eq!(compiled[0].arg2, "");
}

#[test]
fn test_convert_shift_right() {
    let instruction = Instruction::new("SHIFT_RIGHT", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "SHIFT_RIGHT");
    assert_eq!(compiled[0].arg1, "A");
    assert_eq!(compiled[0].arg2, "");
}

#[test]
fn test_convert_shift_right_wrap() {
    let instruction = Instruction::new("SHIFT_RIGHT_W", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "SHIFT_RIGHT_W");
    assert_eq!(compiled[0].arg1, "A");
    assert_eq!(compiled[0].arg2, "");
}

#[test]
fn test_convert_jump() {
    let instruction = Instruction::new("JUMP", "10", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "JUMP");
    assert_eq!(compiled[0].arg1, "10");
    assert_eq!(compiled[0].arg2, "");
}

#[test]
fn test_convert_jump0() {
    let instruction = Instruction::new("JUMP0", "10", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "JUMP0");
    assert_eq!(compiled[0].arg1, "10");
    assert_eq!(compiled[0].arg2, "");
}

#[test]
fn test_convert_nop() {
    let instruction = Instruction::new("NOP", "", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "NOP");
    assert_eq!(compiled[0].arg1, "");
    assert_eq!(compiled[0].arg2, "");
}
