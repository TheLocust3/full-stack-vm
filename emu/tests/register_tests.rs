extern crate emu;
use self::emu::register::Register;

#[test]
fn test_new() {
    let register = Register::new();

    assert_eq!(register.value, 0);
}

#[test]
fn test_set_value() {
    let register = Register::new();

    assert_eq!(register.set_value(10).value, 10);
}