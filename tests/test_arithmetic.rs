extern crate emu;
extern crate asm;

use asm::run::run as compile;
use emu::test_program::test_program;

#[test]
fn test_add() {
    let compiled = compile("SET B 10\nSET C 20\nADD B C");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 0);
    assert_eq!(out_cpu.b.value, 30);
    assert_eq!(out_cpu.c.value, 20);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add2() {
    let compiled = compile("SET B 10\nSET C 20\nADD C B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 0);
    assert_eq!(out_cpu.c.value, 30);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add3() {
    let compiled = compile("SET A 10\nADD A A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 20);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add4() {
    let compiled = compile("SET A 10\nSET B 20\nADD A B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 30);
    assert_eq!(out_cpu.b.value, 20);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add5() {
    let compiled = compile("SET A 10\nSET B 20\nADD B A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.b.value, 30);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add6() {
    let compiled = compile("SET B 10\nADD B B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.b.value, 20);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add_reg_value() {
    let compiled = compile("SET B 10\nADD B 20");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 0);
    assert_eq!(out_cpu.b.value, 30);
    assert_eq!(out_cpu.c.value, 0);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add_reg_value2() {
    let compiled = compile("SET A 10\nADD A 20");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 30);
    assert_eq!(out_cpu.b.value, 0);
    assert_eq!(out_cpu.c.value, 0);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add_reg_addr() {
    let compiled = compile("SET HL 100\nSET B 10\nWRITE64 B\nSET A 20\nADD A (100)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.a.value, 30);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.c.value, 00);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add_reg_addr2() {
    let compiled = compile("SET HL 100\nSET B 10\nWRITE64 B\nSET C 20\nADD C (100)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.a.value, 0);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.c.value, 30);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add_addr_reg() {
    let compiled = compile("SET HL 100\nSET B 10\nWRITE64 B\nSET A 20\nADD (100) A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.a.value, 20);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 30);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_add_regaddr_addr() {
    let compiled = compile("SET HL 50\nSET A 10\nWRITE64 A\nSET HL 100\nSET C 20\nWRITE64 C\nSET B 100\nADD (B) (50)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.b.value, 100);
    assert_eq!(out_cpu.c.value, 20);
    assert_eq!(out_cpu.memory.read_64bit(50), 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 30);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub() {
    let compiled = compile("SET B 30\nSET C 10\nSUB B C");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 0);
    assert_eq!(out_cpu.b.value, 20);
    assert_eq!(out_cpu.c.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub2() {
    let compiled = compile("SET B 10\nSET C 30\nSUB C B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 0);
    assert_eq!(out_cpu.c.value, 20);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub3() {
    let compiled = compile("SET A 20\nSUB A A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 0);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub4() {
    let compiled = compile("SET A 30\nSET B 10\nSUB A B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 20);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub5() {
    let compiled = compile("SET A 10\nSET B 30\nSUB B A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.b.value, 20);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub6() {
    let compiled = compile("SET B 20\nSUB B B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.b.value, 0);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub_reg_value() {
    let compiled = compile("SET B 20\nSUB B 10");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 0);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.c.value, 0);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub_reg_value2() {
    let compiled = compile("SET A 20\nSUB A 10");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.b.value, 0);
    assert_eq!(out_cpu.c.value, 0);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub_reg_addr() {
    let compiled = compile("SET HL 100\nSET B 10\nWRITE64 B\nSET A 30\nSUB A (100)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.a.value, 20);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.c.value, 00);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub_reg_addr2() {
    let compiled = compile("SET HL 100\nSET B 10\nWRITE64 B\nSET C 30\nSUB C (100)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.a.value, 0);
    assert_eq!(out_cpu.b.value, 10);
    assert_eq!(out_cpu.c.value, 20);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub_addr_reg() {
    let compiled = compile("SET HL 100\nSET B 30\nWRITE64 B\nSET A 10\nSUB (100) A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.b.value, 30);
    assert_eq!(out_cpu.memory.read_64bit(100), 20);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_sub_regaddr_addr() {
    let compiled = compile("SET HL 50\nSET A 10\nWRITE64 A\nSET HL 100\nSET C 30\nWRITE64 C\nSET B 100\nSUB (B) (50)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.b.value, 100);
    assert_eq!(out_cpu.c.value, 30);
    assert_eq!(out_cpu.memory.read_64bit(50), 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 20);
    assert_eq!(out_cpu.f.value, 0);
}
