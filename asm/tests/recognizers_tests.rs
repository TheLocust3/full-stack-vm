extern crate asm;

use self::asm::recognizers;

#[test]
fn test_is_register() {
    assert_eq!(recognizers::is_register(&"A".to_string()), true);
    assert_eq!(recognizers::is_register(&"B".to_string()), true);
    assert_eq!(recognizers::is_register(&"C".to_string()), true);
    assert_eq!(recognizers::is_register(&"D".to_string()), true);
    assert_eq!(recognizers::is_register(&"E".to_string()), true);
    assert_eq!(recognizers::is_register(&"F".to_string()), true);
    assert_eq!(recognizers::is_register(&"HL".to_string()), true);
    assert_eq!(recognizers::is_register(&"10".to_string()), false);
    assert_eq!(recognizers::is_register(&"A10".to_string()), false);
    assert_eq!(recognizers::is_register(&"(10)".to_string()), false);
    assert_eq!(recognizers::is_register(&"A(10)".to_string()), false);
}

#[test]
fn test_is_address() {
    assert_eq!(recognizers::is_address(&"(10)".to_string()), true);
    assert_eq!(recognizers::is_address(&"(1)".to_string()), true);
    assert_eq!(recognizers::is_address(&"(100)".to_string()), true);
    assert_eq!(recognizers::is_address(&"A".to_string()), false);
    assert_eq!(recognizers::is_address(&"10".to_string()), false);
}

#[test]
fn test_is_value() {
    assert_eq!(recognizers::is_value(&"10".to_string()), true);
    assert_eq!(recognizers::is_value(&"1".to_string()), true);
    assert_eq!(recognizers::is_value(&"100".to_string()), true);
    assert_eq!(recognizers::is_value(&"A".to_string()), false);
    assert_eq!(recognizers::is_value(&"(10)".to_string()), false);
}
