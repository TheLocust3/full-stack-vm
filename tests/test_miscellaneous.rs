extern crate emu;
extern crate asm;

use asm::run::run as compile;
use emu::test_program::test_program;

#[test]
fn test_push_reg() {
    let compiled = compile("SET A 10\nPUSH A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(out_cpu.sp.value), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_push_reg2() {
    let compiled = compile("SET B 10\nPUSH B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(out_cpu.sp.value), 10);
    assert_eq!(out_cpu.f.value, 0);
}

// TODO: Test registers to make sure op wasn't destructive

#[test]
fn test_push_addr() {
    let compiled = compile("SET HL 100\nSET A 10\nWRITE64 A\nPUSH (100)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.memory.read_64bit(out_cpu.sp.value), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_push_addr2() {
    let compiled = compile("SET HL 100\nSET B 11\nWRITE64 B\nPUSH (100)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.memory.read_64bit(out_cpu.sp.value), 11);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_push_value() {
    let compiled = compile("PUSH 10");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.memory.read_64bit(out_cpu.sp.value), 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_push_value2() {
    let compiled = compile("PUSH 11");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.memory.read_64bit(out_cpu.sp.value), 11);
    assert_eq!(out_cpu.f.value, 0);
}
