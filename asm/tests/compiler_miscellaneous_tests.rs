extern crate asm;

use self::asm::compiler::miscellaneous;

#[test]
fn test_compile_nop() {
    let compiled = miscellaneous::compile_nop();

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000000);
}

#[test]
fn test_compile_push_a() {
    let compiled = miscellaneous::compile_push("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11001000);
}

#[test]
fn test_compile_push_b() {
    let compiled = miscellaneous::compile_push("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11001001);
}

#[test]
fn test_compile_push_c() {
    let compiled = miscellaneous::compile_push("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11001010);
}

#[test]
fn test_compile_push_d() {
    let compiled = miscellaneous::compile_push("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11001011);
}

#[test]
fn test_compile_push_e() {
    let compiled = miscellaneous::compile_push("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11001100);
}

#[test]
fn test_compile_push_f() {
    let compiled = miscellaneous::compile_push("F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11001101);
}

#[test]
fn test_compile_push_hl() {
    let compiled = miscellaneous::compile_push("HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11001110);
}

#[test]
fn test_compile_pop_a() {
    let compiled = miscellaneous::compile_pop("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11101000);
}

#[test]
fn test_compile_pop_b() {
    let compiled = miscellaneous::compile_pop("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11101001);
}

#[test]
fn test_compile_pop_c() {
    let compiled = miscellaneous::compile_pop("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11101010);
}

#[test]
fn test_compile_pop_d() {
    let compiled = miscellaneous::compile_pop("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11101011);
}

#[test]
fn test_compile_pop_e() {
    let compiled = miscellaneous::compile_pop("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11101100);
}

#[test]
fn test_compile_pop_f() {
    let compiled = miscellaneous::compile_pop("F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11101101);
}

#[test]
fn test_compile_pop_hl() {
    let compiled = miscellaneous::compile_pop("HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11101110);
}
