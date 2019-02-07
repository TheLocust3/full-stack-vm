use log::{error};

pub fn register_str_to_code(register: String) -> u8 {
    if register == "A" {
        return 0b000;
    } else if register == "B" {
        return 0b001;
    } else if register == "C" {
        return 0b010;
    } else if register == "D" {
        return 0b011;
    } else if register == "E" {
        return 0b100;
    } else if register == "F" {
        return 0b101;
    } else if register == "HL" {
        return 0b110;
    } else {
        error!("Invalid register name!");
        return 0b000; // default and say what's up
    }
}