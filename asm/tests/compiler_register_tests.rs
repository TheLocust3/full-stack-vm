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

#[test]
fn test_compile_move_a_b() {
    let compiled = register::compile_move("A".to_string(), "B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000001);
}

#[test]
fn test_compile_move_a_c() {
    let compiled = register::compile_move("A".to_string(), "C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000010);
}

#[test]
fn test_compile_move_a_d() {
    let compiled = register::compile_move("A".to_string(), "D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000011);
}

#[test]
fn test_compile_move_a_e() {
    let compiled = register::compile_move("A".to_string(), "E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000100);
}

#[test]
fn test_compile_move_a_f() {
    let compiled = register::compile_move("A".to_string(), "F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000101);
}

#[test]
fn test_compile_move_a_hl() {
    let compiled = register::compile_move("A".to_string(), "HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000110);
}

#[test]
fn test_compile_move_b_a() {
    let compiled = register::compile_move("B".to_string(), "A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00010000);
}

#[test]
fn test_compile_move_b_c() {
    let compiled = register::compile_move("B".to_string(), "C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00010010);
}

#[test]
fn test_compile_move_b_d() {
    let compiled = register::compile_move("B".to_string(), "D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00010011);
}

#[test]
fn test_compile_move_b_e() {
    let compiled = register::compile_move("B".to_string(), "E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00010100);
}

#[test]
fn test_compile_move_b_f() {
    let compiled = register::compile_move("B".to_string(), "F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00010101);
}

#[test]
fn test_compile_move_b_hl() {
    let compiled = register::compile_move("B".to_string(), "HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00010110);
}

#[test]
fn test_compile_move_c_a() {
    let compiled = register::compile_move("C".to_string(), "A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00100000);
}

#[test]
fn test_compile_move_c_b() {
    let compiled = register::compile_move("C".to_string(), "B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00100001);
}

#[test]
fn test_compile_move_c_d() {
    let compiled = register::compile_move("C".to_string(), "D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00100011);
}

#[test]
fn test_compile_move_c_e() {
    let compiled = register::compile_move("C".to_string(), "E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00100100);
}

#[test]
fn test_compile_move_c_f() {
    let compiled = register::compile_move("C".to_string(), "F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00100101);
}

#[test]
fn test_compile_move_c_hl() {
    let compiled = register::compile_move("C".to_string(), "HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00100110);
}

#[test]
fn test_compile_move_d_a() {
    let compiled = register::compile_move("D".to_string(), "A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00110000);
}

#[test]
fn test_compile_move_d_b() {
    let compiled = register::compile_move("D".to_string(), "B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00110001);
}

#[test]
fn test_compile_move_d_c() {
    let compiled = register::compile_move("D".to_string(), "C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00110010);
}

#[test]
fn test_compile_move_d_e() {
    let compiled = register::compile_move("D".to_string(), "E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00110100);
}

#[test]
fn test_compile_move_d_f() {
    let compiled = register::compile_move("D".to_string(), "F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00110101);
}

#[test]
fn test_compile_move_d_hl() {
    let compiled = register::compile_move("D".to_string(), "HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00110110);
}

#[test]
fn test_compile_move_e_a() {
    let compiled = register::compile_move("E".to_string(), "A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01000000);
}

#[test]
fn test_compile_move_e_b() {
    let compiled = register::compile_move("E".to_string(), "B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01000001);
}

#[test]
fn test_compile_move_e_c() {
    let compiled = register::compile_move("E".to_string(), "C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01000010);
}

#[test]
fn test_compile_move_e_d() {
    let compiled = register::compile_move("E".to_string(), "D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01000011);
}

#[test]
fn test_compile_move_e_f() {
    let compiled = register::compile_move("E".to_string(), "F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01000101);
}

#[test]
fn test_compile_move_e_hl() {
    let compiled = register::compile_move("E".to_string(), "HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01000110);
}

#[test]
fn test_compile_move_hl_a() {
    let compiled = register::compile_move("HL".to_string(), "A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01100000);
}

#[test]
fn test_compile_move_hl_b() {
    let compiled = register::compile_move("HL".to_string(), "B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01100001);
}

#[test]
fn test_compile_move_hl_c() {
    let compiled = register::compile_move("HL".to_string(), "C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01100010);
}

#[test]
fn test_compile_move_hl_d() {
    let compiled = register::compile_move("HL".to_string(), "D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01100011);
}

#[test]
fn test_compile_move_hl_e() {
    let compiled = register::compile_move("HL".to_string(), "E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01100100);
}

#[test]
fn test_compile_move_hl_f() {
    let compiled = register::compile_move("HL".to_string(), "F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01100101);
}
