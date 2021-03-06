extern crate asm;
use std::collections::HashMap;

use self::asm::instruction::Instruction;
use self::asm::converter::convert;

#[test]
fn test_convert_move_reg_addr() {
    let instruction = Instruction::new("MOVE", "A", "(10)");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 4);
}

#[test]
fn test_convert_move_reg_reg() {
    let instruction = Instruction::new("MOVE", "A", "B");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 1);
}

#[test]
fn test_convert_move_reg_value() {
    let instruction = Instruction::new("MOVE", "A", "10");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 1);
}

#[test]
fn test_convert_move_reg_addr_reg() {
    let instruction = Instruction::new("MOVE", "A", "(B)");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 4);
}

#[test]
fn test_convert_move_addr_reg() {
    let instruction = Instruction::new("MOVE", "(10)", "A");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 4);
}

#[test]
fn test_convert_move_addr_addr() {
    let instruction = Instruction::new("MOVE", "(10)", "(11)");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 8);
}

#[test]
fn test_convert_move_addr_addr_reg() {
    let instruction = Instruction::new("MOVE", "(10)", "(A)");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 8);
}

#[test]
fn test_convert_move_addr_value() {
    let instruction = Instruction::new("MOVE", "(10)", "10");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 7);
}

#[test]
fn test_convert_move_addr_reg_addr() {
    let instruction = Instruction::new("MOVE", "(A)", "(10)");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 8);
}

#[test]
fn test_convert_move_addr_reg_value() {
    let instruction = Instruction::new("MOVE", "(A)", "1");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 7);
}

#[test]
fn test_convert_move_addr_reg_reg() {
    let instruction = Instruction::new("MOVE", "(A)", "B");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 4);
}

#[test]
fn test_convert_and() {
    let instruction = Instruction::new("AND", "B", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 11);
}

#[test]
fn test_convert_and_a() {
    let instruction = Instruction::new("AND", "A", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 4);
}

#[test]
fn test_convert_or() {
    let instruction = Instruction::new("OR", "B", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 11);
}

#[test]
fn test_convert_or_a() {
    let instruction = Instruction::new("OR", "A", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 4);
}

#[test]
fn test_convert_add() {
    let instruction = Instruction::new("ADD", "B", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 11);
}

#[test]
fn test_convert_add_a() {
    let instruction = Instruction::new("ADD", "A", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 4);
}

#[test]
fn test_convert_sub() {
    let instruction = Instruction::new("SUB", "B", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 11);
}

#[test]
fn test_convert_sub_a() {
    let instruction = Instruction::new("SUB", "A", "C");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 4);
}

#[test]
fn test_convert_not() {
    let instruction = Instruction::new("NOT", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 5);
}

#[test]
fn test_convert_shift_left() {
    let instruction = Instruction::new("SHIFT_LEFT", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 5);
}

#[test]
fn test_convert_shift_left_wrap() {
    let instruction = Instruction::new("SHIFT_LEFT_W", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 5);
}

#[test]
fn test_convert_shift_right() {
    let instruction = Instruction::new("SHIFT_RIGHT", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 5);
}

#[test]
fn test_convert_shift_right_wrap() {
    let instruction = Instruction::new("SHIFT_RIGHT_W", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 5);
}

#[test]
fn test_convert_jump() {
    let instruction = Instruction::new("JUMP", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 1);
}

#[test]
fn test_convert_jump0() {
    let instruction = Instruction::new("JUMP0", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 1);
}

#[test]
fn test_convert_nop() {
    let instruction = Instruction::new("NOP", "", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "NOP");
    assert_eq!(compiled[0].arg1, "");
    assert_eq!(compiled[0].arg2, "");
}

#[test]
fn test_convert_halt() {
    let instruction = Instruction::new("HALT", "", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "HALT");
    assert_eq!(compiled[0].arg1, "");
    assert_eq!(compiled[0].arg2, "");
}

#[test]
fn test_convert_push_reg() {
    let instruction = Instruction::new("PUSH", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "PUSH");
    assert_eq!(compiled[0].arg1, "A");
    assert_eq!(compiled[0].arg2, "");
}

#[test]
fn test_convert_push_addr() {
    let instruction = Instruction::new("PUSH", "(10)", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 13);
}

#[test]
fn test_convert_push_value() {
    let instruction = Instruction::new("PUSH", "10", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 10);
}

#[test]
fn test_convert_pop() {
    let instruction = Instruction::new("POP", "A", "");

    let compiled: Vec<Instruction> = convert::convert_instruction(instruction, &HashMap::new());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0].command, "POP");
    assert_eq!(compiled[0].arg1, "A");
    assert_eq!(compiled[0].arg2, "");
}
