extern crate asm;

use self::asm::compiler::arithmetic;

#[test]
fn test_compile_add_a() {
    let compiled = arithmetic::compile_add("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00001111);
}

#[test]
fn test_compile_add_b() {
    let compiled = arithmetic::compile_add("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00101111);
}

#[test]
fn test_compile_add_c() {
    let compiled = arithmetic::compile_add("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01001111);
}

#[test]
fn test_compile_add_d() {
    let compiled = arithmetic::compile_add("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01101111);
}

#[test]
fn test_compile_add_e() {
    let compiled = arithmetic::compile_add("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10001111);
}

#[test]
fn test_compile_add_f() {
    let compiled = arithmetic::compile_add("F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10101111);
}

#[test]
fn test_compile_add_hl() {
    let compiled = arithmetic::compile_add("HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11001111);
}

#[test]
fn test_compile_sub_a() {
    let compiled = arithmetic::compile_sub("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00011111);
}

#[test]
fn test_compile_sub_b() {
    let compiled = arithmetic::compile_sub("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00111111);
}

#[test]
fn test_compile_sub_c() {
    let compiled = arithmetic::compile_sub("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01011111);
}

#[test]
fn test_compile_sub_d() {
    let compiled = arithmetic::compile_sub("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01111111);
}

#[test]
fn test_compile_sub_e() {
    let compiled = arithmetic::compile_sub("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10011111);
}

#[test]
fn test_compile_sub_f() {
    let compiled = arithmetic::compile_sub("F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10111111);
}

#[test]
fn test_compile_sub_hl() {
    let compiled = arithmetic::compile_sub("HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11011111);
}
