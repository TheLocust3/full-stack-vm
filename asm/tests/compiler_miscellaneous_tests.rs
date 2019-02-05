extern crate asm;

use self::asm::compiler::miscellaneous;

#[test]
fn test_compile_nop() {
    let compiled = miscellaneous::compile_nop();

    assert_eq!(compiled.len(), 1);
    assert_eq!(compiled[0], 0b00000000);
}
