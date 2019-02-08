extern crate asm;

use self::asm::compiler::bitwise;

#[test]
fn test_compile_and_a() {
    let compiled = bitwise::compile_and("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b000011000);
}

#[test]
fn test_compile_and_b() {
    let compiled = bitwise::compile_and("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00111000);
}

#[test]
fn test_compile_and_c() {
    let compiled = bitwise::compile_and("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01011000);
}

#[test]
fn test_compile_and_d() {
    let compiled = bitwise::compile_and("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01111000);
}

#[test]
fn test_compile_and_e() {
    let compiled = bitwise::compile_and("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10011000);
}

#[test]
fn test_compile_and_f() {
    let compiled = bitwise::compile_and("F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10111000);
}

#[test]
fn test_compile_and_hl() {
    let compiled = bitwise::compile_and("HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11011000);
}

#[test]
fn test_compile_or_a() {
    let compiled = bitwise::compile_or("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b000011001);
}

#[test]
fn test_compile_or_b() {
    let compiled = bitwise::compile_or("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00111001);
}

#[test]
fn test_compile_or_c() {
    let compiled = bitwise::compile_or("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01011001);
}

#[test]
fn test_compile_or_d() {
    let compiled = bitwise::compile_or("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01111001);
}

#[test]
fn test_compile_or_e() {
    let compiled = bitwise::compile_or("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10011001);
}

#[test]
fn test_compile_or_f() {
    let compiled = bitwise::compile_or("F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10111001);
}

#[test]
fn test_compile_or_hl() {
    let compiled = bitwise::compile_or("HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11011001);
}

#[test]
fn test_compile_not_a() {
    let compiled = bitwise::compile_not("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b000011011);
}

#[test]
fn test_compile_not_b() {
    let compiled = bitwise::compile_not("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00111011);
}

#[test]
fn test_compile_not_c() {
    let compiled = bitwise::compile_not("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01011011);
}

#[test]
fn test_compile_not_d() {
    let compiled = bitwise::compile_not("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01111011);
}

#[test]
fn test_compile_not_e() {
    let compiled = bitwise::compile_not("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10011011);
}

#[test]
fn test_compile_not_f() {
    let compiled = bitwise::compile_not("F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10111011);
}

#[test]
fn test_compile_not_hl() {
    let compiled = bitwise::compile_not("HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11011011);
}

#[test]
fn test_compile_shift_left_a() {
    let compiled = bitwise::compile_shift_left("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00011101);
}

#[test]
fn test_compile_shift_left_b() {
    let compiled = bitwise::compile_shift_left("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00111101);
}

#[test]
fn test_compile_shift_left_c() {
    let compiled = bitwise::compile_shift_left("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01011101);
}

#[test]
fn test_compile_shift_left_d() {
    let compiled = bitwise::compile_shift_left("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01111101);
}

#[test]
fn test_compile_shift_left_e() {
    let compiled = bitwise::compile_shift_left("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10011101);
}

#[test]
fn test_compile_shift_left_f() {
    let compiled = bitwise::compile_shift_left("F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10111101);
}

#[test]
fn test_compile_shift_left_hl() {
    let compiled = bitwise::compile_shift_left("HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11011101);
}

#[test]
fn test_compile_shift_left_wrap_a() {
    let compiled = bitwise::compile_shift_left_wrap("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000111);
}

#[test]
fn test_compile_shift_left_wrap_b() {
    let compiled = bitwise::compile_shift_left_wrap("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00100111);
}

#[test]
fn test_compile_shift_left_wrap_c() {
    let compiled = bitwise::compile_shift_left_wrap("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01000111);
}

#[test]
fn test_compile_shift_left_wrap_d() {
    let compiled = bitwise::compile_shift_left_wrap("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01100111);
}

#[test]
fn test_compile_shift_left_wrap_e() {
    let compiled = bitwise::compile_shift_left_wrap("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10000111);
}

#[test]
fn test_compile_shift_left_wrap_f() {
    let compiled = bitwise::compile_shift_left_wrap("F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10100111);
}

#[test]
fn test_compile_shift_left_wrap_hl() {
    let compiled = bitwise::compile_shift_left_wrap("HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11000111);
}

#[test]
fn test_compile_shift_right_a() {
    let compiled = bitwise::compile_shift_right("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00011110);
}

#[test]
fn test_compile_shift_right_b() {
    let compiled = bitwise::compile_shift_right("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00111110);
}

#[test]
fn test_compile_shift_right_c() {
    let compiled = bitwise::compile_shift_right("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01011110);
}

#[test]
fn test_compile_shift_right_d() {
    let compiled = bitwise::compile_shift_right("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01111110);
}

#[test]
fn test_compile_shift_right_e() {
    let compiled = bitwise::compile_shift_right("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10011110);
}

#[test]
fn test_compile_shift_right_f() {
    let compiled = bitwise::compile_shift_right("F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10111110);
}

#[test]
fn test_compile_shift_right_hl() {
    let compiled = bitwise::compile_shift_right("HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11011110);
}

#[test]
fn test_compile_shift_right_wrap_a() {
    let compiled = bitwise::compile_shift_right_wrap("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00010111);
}

#[test]
fn test_compile_shift_right_wrap_b() {
    let compiled = bitwise::compile_shift_right_wrap("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00110111);
}

#[test]
fn test_compile_shift_right_wrap_c() {
    let compiled = bitwise::compile_shift_right_wrap("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01010111);
}

#[test]
fn test_compile_shift_right_wrap_d() {
    let compiled = bitwise::compile_shift_right_wrap("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b01110111);
}

#[test]
fn test_compile_shift_right_wrap_e() {
    let compiled = bitwise::compile_shift_right_wrap("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10010111);
}

#[test]
fn test_compile_shift_right_wrap_f() {
    let compiled = bitwise::compile_shift_right_wrap("F".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10110111);
}

#[test]
fn test_compile_shift_right_wrap_hl() {
    let compiled = bitwise::compile_shift_right_wrap("HL".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11010111);
}
