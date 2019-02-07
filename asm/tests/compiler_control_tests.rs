extern crate asm;

use self::asm::compiler::control;

#[test]
fn test_compile_jump() {
    let compiled = control::compile_jump("10".to_string());

    assert_eq!(compiled.len(), 9);
    assert_eq!(compiled[0], 0b11111111);
    assert_eq!(compiled[1], 0);
    assert_eq!(compiled[2], 0);
    assert_eq!(compiled[3], 0);
    assert_eq!(compiled[4], 0);
    assert_eq!(compiled[5], 0);
    assert_eq!(compiled[6], 0);
    assert_eq!(compiled[7], 0);
    assert_eq!(compiled[8], 10);
}
