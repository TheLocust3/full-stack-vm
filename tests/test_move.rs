extern crate emu;
extern crate asm;

use asm::run::run as compile;
use emu::test_program::test_program;

#[test]
fn test_set() {
    let compiled = compile("SET A 10");
    // println!("{}", compiled[0]);
    // println!("{}", compiled.len());
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 0);
    assert_eq!(out_cpu.a.value, 10);
}
