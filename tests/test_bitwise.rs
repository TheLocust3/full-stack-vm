extern crate emu;
extern crate asm;

use asm::run::run as compile;
use emu::test_program::test_program;

#[test]
fn test_and() {
    let compiled = compile("SET B 255\nSET C 345\nAND B C");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 0);
    assert_eq!(out_cpu.b.value, 89);
    assert_eq!(out_cpu.c.value, 345);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_and2() {
    let compiled = compile("SET C 255\nSET B 345\nAND C B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 0);
    assert_eq!(out_cpu.c.value, 89);
    assert_eq!(out_cpu.b.value, 345);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_and3() {
    let compiled = compile("SET A 255\nAND A A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 255);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_and4() {
    let compiled = compile("SET B 255\nAND B B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.b.value, 255);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_and5() {
    let compiled = compile("SET A 255\nSET B 345\nAND A B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 89);
    assert_eq!(out_cpu.b.value, 345);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_and6() {
    let compiled = compile("SET A 255\nSET B 345\nAND B A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 255);
    assert_eq!(out_cpu.b.value, 89);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_or() {
    let compiled = compile("SET B 255\nSET C 1000\nOR B C");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.b.value, 1023);
    assert_eq!(out_cpu.c.value, 1000);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_or2() {
    let compiled = compile("SET C 255\nSET B 1000\nOR C B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.c.value, 1023);
    assert_eq!(out_cpu.b.value, 1000);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_or3() {
    let compiled = compile("SET A 255\nOR A A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 255);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_or4() {
    let compiled = compile("SET B 255\nOR B B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.b.value, 255);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_or5() {
    let compiled = compile("SET A 255\nSET B 1000\nOR A B");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.a.value, 1023);
    assert_eq!(out_cpu.b.value, 1000);
    assert_eq!(out_cpu.f.value, 0);
}

#[test]
fn test_or6() {
    let compiled = compile("SET A 255\nSET B 1000\nOR B A");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.b.value, 1023);
    assert_eq!(out_cpu.a.value, 255);
    assert_eq!(out_cpu.f.value, 0);
}
