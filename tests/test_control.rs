extern crate emu;
extern crate asm;

use asm::run::run as compile;
use emu::test_program::test_program;

#[test]
fn test_jump_a() {
    let compiled = compile("SET A 340\nJUMP A\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 341);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump_b() {
    let compiled = compile("SET B 340\nJUMP B\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 341);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump_value() {
    let compiled = compile("JUMP 340\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 341);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump_addr() {
    let compiled = compile("MOVE (100) 365\nJUMP (100)\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 367);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump_regaddr() {
    let compiled = compile("MOVE (100) 366\nMOVE A 100\nJUMP (A)\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 368);
    assert_eq!(out_cpu.f.value, 0);
}


#[test]
fn test_jump0_true() {
    let compiled = compile("SET A 0\nSET B 348\nJUMP0 B\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 350);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump0_false() {
    let compiled = compile("SET A 10\nSET B 348\nJUMP0 B\nHALT\nNOP"); // hit first halt instead of end halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 348);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump0_true_value() {
    let compiled = compile("MOVE A 0\nJUMP0 341\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 346);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump0_true_addr() {
    let compiled = compile("MOVE A 0\nMOVE (100) 374\nJUMP0 (100)\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 376);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump0_true_regaddr() {
    let compiled = compile("MOVE A 0\nMOVE (100) 375\nMOVE B 100\nJUMP0 (B)\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 377);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump0_false_value() {
    let compiled = compile("MOVE A 10\nJUMP0 341\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 348);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump0_false_addr() {
    let compiled = compile("MOVE A 10\nMOVE (100) 374\nJUMP0 (100)\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 374);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump0_false_regaddr() {
    let compiled = compile("MOVE A 10\nMOVE (100) 375\nMOVE B 100\nJUMP0 (B)\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 375);
    assert_eq!(out_cpu.f.value, 0);
}
