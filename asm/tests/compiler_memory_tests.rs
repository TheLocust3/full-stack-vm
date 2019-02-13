extern crate asm;

use self::asm::compiler::memory;

#[test]
fn test_compile_read8_a() {
    let compiled = memory::compile_read8("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11111000);
}

#[test]
fn test_compile_read8_b() {
    let compiled = memory::compile_read8("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11111001);
}

#[test]
fn test_compile_read8_c() {
    let compiled = memory::compile_read8("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11111010);
}

#[test]
fn test_compile_read8_d() {
    let compiled = memory::compile_read8("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11111011);
}

#[test]
fn test_compile_read8_e() {
    let compiled = memory::compile_read8("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11111100);
}

#[test]
fn test_compile_read16_a() {
    let compiled = memory::compile_read16("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11110000);
}

#[test]
fn test_compile_read16_b() {
    let compiled = memory::compile_read16("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11110001);
}

#[test]
fn test_compile_read16_c() {
    let compiled = memory::compile_read16("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11110010);
}

#[test]
fn test_compile_read16_d() {
    let compiled = memory::compile_read16("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11110011);
}

#[test]
fn test_compile_read16_e() {
    let compiled = memory::compile_read16("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11110100);
}

#[test]
fn test_compile_read32_a() {
    let compiled = memory::compile_read32("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11100000);
}

#[test]
fn test_compile_read32_b() {
    let compiled = memory::compile_read32("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11100001);
}

#[test]
fn test_compile_read32_c() {
    let compiled = memory::compile_read32("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11100010);
}

#[test]
fn test_compile_read32_d() {
    let compiled = memory::compile_read32("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11100011);
}

#[test]
fn test_compile_read32_e() {
    let compiled = memory::compile_read32("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11100100);
}

#[test]
fn test_compile_read64_a() {
    let compiled = memory::compile_read64("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11000000);
}

#[test]
fn test_compile_read64_b() {
    let compiled = memory::compile_read64("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11000001);
}

#[test]
fn test_compile_read64_c() {
    let compiled = memory::compile_read64("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11000010);
}

#[test]
fn test_compile_read64_d() {
    let compiled = memory::compile_read64("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11000011);
}

#[test]
fn test_compile_read64_e() {
    let compiled = memory::compile_read64("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b11000100);
}

#[test]
fn test_compile_write8_a() {
    let compiled = memory::compile_write8("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10000000);
}

#[test]
fn test_compile_write8_b() {
    let compiled = memory::compile_write8("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10000001);
}

#[test]
fn test_compile_write8_c() {
    let compiled = memory::compile_write8("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10000010);
}

#[test]
fn test_compile_write8_d() {
    let compiled = memory::compile_write8("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10000011);
}

#[test]
fn test_compile_write8_e() {
    let compiled = memory::compile_write8("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10000100);
}

#[test]
fn test_compile_write16_a() {
    let compiled = memory::compile_write16("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10001000);
}

#[test]
fn test_compile_write16_b() {
    let compiled = memory::compile_write16("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10001001);
}

#[test]
fn test_compile_write16_c() {
    let compiled = memory::compile_write16("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10001010);
}

#[test]
fn test_compile_write16_d() {
    let compiled = memory::compile_write16("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10001011);
}

#[test]
fn test_compile_write16_e() {
    let compiled = memory::compile_write16("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10001100);
}

#[test]
fn test_compile_write32_a() {
    let compiled = memory::compile_write32("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10010000);
}

#[test]
fn test_compile_write32_b() {
    let compiled = memory::compile_write32("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10010001);
}

#[test]
fn test_compile_write32_c() {
    let compiled = memory::compile_write32("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10010010);
}

#[test]
fn test_compile_write32_d() {
    let compiled = memory::compile_write32("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10010011);
}

#[test]
fn test_compile_write32_e() {
    let compiled = memory::compile_write32("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10010100);
}

#[test]
fn test_compile_write64_a() {
    let compiled = memory::compile_write64("A".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10101000);
}

#[test]
fn test_compile_write64_b() {
    let compiled = memory::compile_write64("B".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10101001);
}

#[test]
fn test_compile_write64_c() {
    let compiled = memory::compile_write64("C".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10101010);
}

#[test]
fn test_compile_write64_d() {
    let compiled = memory::compile_write64("D".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10101011);
}

#[test]
fn test_compile_write64_e() {
    let compiled = memory::compile_write64("E".to_string());

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b10101100);
}
