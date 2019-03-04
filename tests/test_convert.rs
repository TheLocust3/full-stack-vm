extern crate emu;
extern crate asm;

use asm::run::run as compile;
use emu::test_program::test_program;

#[test]
fn test_push_addr() {
    let compiled = compile("SET HL 100\nSET A 10\nWRITE64 A\nPUSH (100)");
    let out_cpu = test_program(compiled);

    assert_eq!(out_cpu.hl.value, 100);
    assert_eq!(out_cpu.a.value, 10);
    assert_eq!(out_cpu.memory.read_64bit(100), 10);
    assert_eq!(out_cpu.memory.read_64bit(out_cpu.sp.value), 10);
    assert_eq!(out_cpu.f.value, 0);
}

// TODO: Test other moves

// TODO: Test other adds

// TODO: Test other subs

// TODO: Test other ands

// TODO: Test other ors
