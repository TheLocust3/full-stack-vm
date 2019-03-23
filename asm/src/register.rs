use log::{error};

use data::parse_address;

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

pub fn get_unused_reg(used_reg1: &str, used_reg2: &str, used_reg3: &str) -> String {
    let reg1 = parse_address(used_reg1.to_string());
    let reg2 = parse_address(used_reg2.to_string());
    let reg3 = parse_address(used_reg3.to_string());

    if reg1 != "A" && reg2 != "A" && reg3 != "A" {
        "A".to_string()
    } else if reg1 != "B" && reg2 != "B" && reg3 != "B" {
        "B".to_string()
    } else if reg1 != "C" && reg2 != "C" && reg3 != "C" {
        "C".to_string()
    } else {
        "D".to_string()
    }
}

pub fn get_dest_reg(used_reg1: &str) -> String {
    get_unused_reg(used_reg1, used_reg1.clone(), used_reg1.clone())
}
