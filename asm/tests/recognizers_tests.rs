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
    assert_eq!(recognizers::is_register(&"".to_string()), false);
    assert_eq!(recognizers::is_register(&"()".to_string()), false);
}

#[test]
fn test_is_address() {
    assert_eq!(recognizers::is_address(&"(10)".to_string()), true);
    assert_eq!(recognizers::is_address(&"(1)".to_string()), true);
    assert_eq!(recognizers::is_address(&"(100)".to_string()), true);
    assert_eq!(recognizers::is_address(&"(A)".to_string()), true);
    assert_eq!(recognizers::is_address(&"(Z)".to_string()), true);
    assert_eq!(recognizers::is_address(&"A".to_string()), false);
    assert_eq!(recognizers::is_address(&"10".to_string()), false);
    assert_eq!(recognizers::is_address(&"".to_string()), false);
    assert_eq!(recognizers::is_address(&"()".to_string()), false);
}

#[test]
fn test_is_address_value() {
    assert_eq!(recognizers::is_address_value(&"(10)".to_string()), true);
    assert_eq!(recognizers::is_address_value(&"(1)".to_string()), true);
    assert_eq!(recognizers::is_address_value(&"(100)".to_string()), true);
    assert_eq!(recognizers::is_address_value(&"(A)".to_string()), false);
    assert_eq!(recognizers::is_address_value(&"(B)".to_string()), false);
    assert_eq!(recognizers::is_address_value(&"A".to_string()), false);
    assert_eq!(recognizers::is_address_value(&"10".to_string()), false);
    assert_eq!(recognizers::is_address_value(&"".to_string()), false);
    assert_eq!(recognizers::is_address_value(&"()".to_string()), false);
}

#[test]
fn test_is_address_register() {
    assert_eq!(recognizers::is_address_register(&"(A)".to_string()), true);
    assert_eq!(recognizers::is_address_register(&"(B)".to_string()), true);
    assert_eq!(recognizers::is_address_register(&"(10)".to_string()), false);
    assert_eq!(recognizers::is_address_register(&"(1)".to_string()), false);
    assert_eq!(recognizers::is_address_register(&"(100)".to_string()), false);
    assert_eq!(recognizers::is_address_register(&"A".to_string()), false);
    assert_eq!(recognizers::is_address_register(&"10".to_string()), false);
    assert_eq!(recognizers::is_address_register(&"".to_string()), false);
    assert_eq!(recognizers::is_address_register(&"()".to_string()), false);
}

#[test]
fn test_is_label() {
    assert_eq!(recognizers::is_label(&":test".to_string()), true);
    assert_eq!(recognizers::is_label(&":test-function".to_string()), true);
    assert_eq!(recognizers::is_label(&"(10)".to_string()), false);
    assert_eq!(recognizers::is_label(&"(A)".to_string()), false);
    assert_eq!(recognizers::is_label(&"10".to_string()), false);
    assert_eq!(recognizers::is_label(&"".to_string()), false);
    assert_eq!(recognizers::is_label(&"()".to_string()), false);
}

#[test]
fn test_is_value() {
    assert_eq!(recognizers::is_value(&"10".to_string()), true);
    assert_eq!(recognizers::is_value(&"1".to_string()), true);
    assert_eq!(recognizers::is_value(&"100".to_string()), true);
    assert_eq!(recognizers::is_value(&"A".to_string()), false);
    assert_eq!(recognizers::is_value(&"(10)".to_string()), false);
    assert_eq!(recognizers::is_value(&":test".to_string()), false);
}
