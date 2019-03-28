extern crate asm;

use self::asm::compiler::control;

#[test]
fn test_compile_jump() {
    let compiled = control::compile_jump("A".to_string());

    assert_eq!(compiled.len(), 1);
}

#[test]
fn test_compile_jump0() {
    let compiled = control::compile_jump0("10".to_string());

    assert_eq!(compiled.len(), 1);
}
