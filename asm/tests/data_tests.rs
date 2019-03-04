extern crate asm;

use self::asm::data;

#[test]
fn test_address_to_u64() {
    assert_eq!(data::parse_address("(100)".to_string()), "100");
    assert_eq!(data::parse_address("(10)".to_string()), "10");
    assert_eq!(data::parse_address("(1)".to_string()), "1");
}
