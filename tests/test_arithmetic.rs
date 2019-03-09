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

    println!("A: {}, B: {}", out_cpu.a.value, out_cpu.b.value);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.b.value, 20);
    assert_eq!(out_cpu.f.value, 0);
}
