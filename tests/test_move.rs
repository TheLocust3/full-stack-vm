extern crate emu;
extern crate asm;

use asm::run::run as compile;
use emu::test_program::test_program;

#[test]
fn test_move_reg_val() {
    let compiled = compile("MOVE A 10");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_reg_val2() {
    let compiled = compile("MOVE B 10");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_reg_addr() {
    let compiled = compile("SET HL 100\nSET B 10\nWRITE64 B\nMOVE A (100)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_reg_addr2() {
    let compiled = compile("SET HL 100\nSET A 10\nWRITE64 A\nMOVE B (100)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_reg_regaddr() {
    let compiled = compile("SET HL 100\nSET B 10\nWRITE64 B\nSET C 100\nMOVE A (C)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.c.value, 100);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_reg_regaddr2() {
    let compiled = compile("SET HL 100\nSET A 10\nWRITE64 A\nSET C 100\nMOVE B (C)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.c.value, 100);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_addr_reg() {
    let compiled = compile("SET A 10\nMOVE (100) A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_addr_reg2() {
    let compiled = compile("SET B 10\nMOVE (100) B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_addr_addr() {
    let compiled = compile("SET HL 100\nSET A 10\nWRITE64 A\nMOVE (50) (100)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.memory.read_64bit(50), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_addr_regaddr() {
    let compiled = compile("SET HL 100\nSET B 10\nWRITE64 B\nSET A 100\nMOVE (50) (A)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.a.value, 100);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.memory.read_64bit(50), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_addr_regaddr2() {
    let compiled = compile("SET HL 100\nSET A 10\nWRITE64 A\nSET B 100\nMOVE (50) (B)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.b.value, 100);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.memory.read_64bit(50), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_addr_val() {
    let compiled = compile("MOVE (100) 10");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_regaddr_addr() {
    let compiled = compile("SET HL 50\nSET A 10\nWRITE64 A\nSET B 100\nMOVE (B) (50)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 50);
    assert_eq!(out_cpu.b.value, 100);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(50), 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_regaddr_addr2() {
    let compiled = compile("SET HL 50\nSET B 10\nWRITE64 B\nSET A 100\nMOVE (A) (50)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 50);
    assert_eq!(out_cpu.a.value, 100);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(50), 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_regaddr_value() {
    let compiled = compile("SET A 100\nMOVE (A) 10");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.a.value, 100);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_regaddr_value2() {
    let compiled = compile("SET B 100\nMOVE (B) 10");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.b.value, 100);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_regaddr_reg() {
    let compiled = compile("SET A 100\nSET B 10\nMOVE (A) B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.a.value, 100);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move_regaddr_reg2() {
    let compiled = compile("SET C 100\nSET D 10\nMOVE (C) D");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.c.value, 100);
    assert_eq!(out_cpu.d.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}
