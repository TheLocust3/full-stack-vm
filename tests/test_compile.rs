extern crate emu;
extern crate asm;

use asm::run::run as compile;
use emu::test_program::test_program;

#[test]
fn test_push() {
    let compiled = compile("SET A 10\nPUSH A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(out_cpu.sp.value), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_pop() {
    let compiled = compile("SET A 10\nPUSH A\nPOP B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_set() {
    let compiled = compile("SET A 10");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_move() {
    let compiled = compile("SET A 10\nMOVE B A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add() {
    let compiled = compile("SET A 10\nSET B 20\nADD A B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 30);
    assert_eq!(out_cpu.b.value, 20);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub() {
    let compiled = compile("SET A 20\nSET B 10\nSUB A B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_and() {
    let compiled = compile("SET A 255\nSET B 345\nAND A B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 89);
    assert_eq!(out_cpu.b.value, 345);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_or() {
    let compiled = compile("SET A 255\nSET B 1000\nOR A B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 1023);
    assert_eq!(out_cpu.b.value, 1000);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_not() {
    let compiled = compile("SET A 0\nNOT A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, u64::max_value());
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_shift_left() {
    let compiled = compile("SET A 1\nSHIFT_LEFT A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 2);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_shift_left_wrap() {
    let compiled = compile("SET A 1\nSHIFT_LEFT_W A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 2);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_shift_right() {
    let compiled = compile("SET A 2\nSHIFT_RIGHT A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 1);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_shift_right_wrap() {
    let compiled = compile("SET A 2\nSHIFT_RIGHT_W A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 1);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_jump() {
    let compiled = compile("SET A 340\nJUMP A\nHALT\nNOP"); // jump over halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 341);
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
    let compiled = compile("SET A 10\nSET B 248\nJUMP0 B\nHALT\nNOP"); // hit first halt instead of end halt
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 348);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_write8() {
    let compiled = compile("SET A 10\nSET HL 100\nWRITE8 A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.memory.read_8bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_write16() {
    let compiled = compile("SET A 10\nSET HL 100\nWRITE16 A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.memory.read_16bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_write32() {
    let compiled = compile("SET A 10\nSET HL 100\nWRITE32 A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.memory.read_32bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_write64() {
    let compiled = compile("SET A 10\nSET HL 100\nWRITE64 A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_read8() {
    let compiled = compile("SET A 10\nSET HL 100\nWRITE8 A\nREAD8 B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_read16() {
    let compiled = compile("SET A 10\nSET HL 100\nWRITE16 A\nREAD16 B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_read32() {
    let compiled = compile("SET A 10\nSET HL 100\nWRITE32 A\nREAD32 B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_read64() {
    let compiled = compile("SET A 10\nSET HL 100\nWRITE64 A\nREAD64 B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_nop() {
    let compiled = compile("NOP");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 330);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_halt() {
    let compiled = compile("HALT");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.pc.value, 329);
    assert_eq!(out_cpu.f.value, 0);
}
