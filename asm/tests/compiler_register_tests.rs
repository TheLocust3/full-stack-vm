extern crate asm;

use self::asm::compiler::register;

#[test]
fn test_compile_set_a() {
    let compiled = register::compile_set("A".to_string(), "10".to_string());

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
fn test_compile_set_b() {
    let compiled = register::compile_set("B".to_string(), "10".to_string());

    assert_eq!(compiled.len(), 9);
    assert_eq!(compiled[0], 0b00001001);
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
fn test_compile_set_c() {
    let compiled = register::compile_set("C".to_string(), "10".to_string());

    assert_eq!(compiled.len(), 9);
    assert_eq!(compiled[0], 0b00001010);
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
fn test_compile_set_d() {
    let compiled = register::compile_set("D".to_string(), "10".to_string());

    assert_eq!(compiled.len(), 9);
    assert_eq!(compiled[0], 0b00001011);
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
fn test_compile_set_e() {
    let compiled = register::compile_set("E".to_string(), "10".to_string());

    assert_eq!(compiled.len(), 9);
    assert_eq!(compiled[0], 0b00001100);
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
fn test_compile_set_hl() {
    let compiled = register::compile_set("HL".to_string(), "10".to_string());

    assert_eq!(compiled.len(), 9);
    assert_eq!(compiled[0], 0b00001110);
    assert_eq!(compiled[1], 0);
    assert_eq!(compiled[2], 0);
    assert_eq!(compiled[3], 0);
    assert_eq!(compiled[4], 0);
    assert_eq!(compiled[5], 0);
    assert_eq!(compiled[6], 0);
    assert_eq!(compiled[7], 0);
    assert_eq!(compiled[8], 10);
}
