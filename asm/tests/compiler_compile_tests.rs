extern crate asm;

use self::asm::instruction;
use self::asm::compiler::compile;

#[test]
fn test_compile_add() {
    let instruction = instruction::Instruction::new("ADD", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00001111);
}

#[test]
fn test_compile_sub() {
    let instruction = instruction::Instruction::new("SUB", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00011111);
}

#[test]
fn test_compile_and() {
    let instruction = instruction::Instruction::new("AND", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00011000);
}

#[test]
fn test_compile_or() {
    let instruction = instruction::Instruction::new("OR", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00011001);
}

#[test]
fn test_compile_not() {
    let instruction = instruction::Instruction::new("NOT", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00011011);
}

#[test]
fn test_compile_shift_left() {
    let instruction = instruction::Instruction::new("SHIFT_LEFT", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00011101);
}

#[test]
fn test_compile_shift_left_wrap() {
    let instruction = instruction::Instruction::new("SHIFT_LEFT_W", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000111);
}

#[test]
fn test_compile_shift_right() {
    let instruction = instruction::Instruction::new("SHIFT_RIGHT", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00011110);
}

#[test]
fn test_compile_shift_right_wrap() {
    let instruction = instruction::Instruction::new("SHIFT_RIGHT_W", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00010111);
}

#[test]
fn test_compile_jump() {
    let instruction = instruction::Instruction::new("JUMP", "10", "");

    let compiled = compile::compile_instruction(instruction);

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
fn test_compile_jump0() {
    let instruction = instruction::Instruction::new("JUMP0", "10", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 9);
    assert_eq!(compiled[0], 0b11111110);
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
fn test_compile_set() {
    let instruction = instruction::Instruction::new("SET", "A", "10");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 9);
    assert_eq!(compiled[0], 0b00001000);
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
fn test_compile_move_a_b() {
    let instruction = instruction::Instruction::new("MOVE", "A", "B");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000001);
}

#[test]
fn test_compile_read8() {
    let instruction = instruction::Instruction::new("READ8", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11111000);
}

#[test]
fn test_compile_read16() {
    let instruction = instruction::Instruction::new("READ16", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11110000);
}

#[test]
fn test_compile_read32() {
    let instruction = instruction::Instruction::new("READ32", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11100000);
}

#[test]
fn test_compile_read64() {
    let instruction = instruction::Instruction::new("READ64", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11000000);
}

#[test]
fn test_compile_write8() {
    let instruction = instruction::Instruction::new("WRITE8", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10000000);
}

#[test]
fn test_compile_write16() {
    let instruction = instruction::Instruction::new("WRITE16", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10001000);
}

#[test]
fn test_compile_write32() {
    let instruction = instruction::Instruction::new("WRITE32", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10010000);
}

#[test]
fn test_compile_write64() {
    let instruction = instruction::Instruction::new("WRITE64", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10101000);
}

#[test]
fn test_compile_nop() {
    let instruction = instruction::Instruction::new("NOP", "", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000000);
}

#[test]
fn test_compile_push() {
    let instruction = instruction::Instruction::new("PUSH", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11001000);
}

#[test]
fn test_compile_pop() {
    let instruction = instruction::Instruction::new("POP", "A", "");

    let compiled = compile::compile_instruction(instruction);

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11101000);
}
