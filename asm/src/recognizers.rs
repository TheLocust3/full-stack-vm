use data::parse_address;

pub fn is_register(arg: &String) -> bool {
    arg == "A" || arg == "B" || arg == "C" || arg == "D" || arg == "E" || arg == "F" || arg == "HL"
}

pub fn is_address(arg: &String) -> bool {
    if arg.len() < 3 {
        return false
    }
    
    let first = arg.chars().next().unwrap();
    let last = arg.chars().last().unwrap();

    first == '(' && last  == ')'
}

pub fn is_address_value(arg: &String) -> bool {
    if arg.len() < 3 {
        return false
    }

    let val = parse_address(arg.to_string());

    is_address(&arg) && !is_register(&val)
}

pub fn is_address_register(arg: &String) -> bool {
    if arg.len() < 3 {
        return false
    }

    let reg = parse_address(arg.to_string());

    is_address(&arg) && is_register(&reg)
}

pub fn is_label(arg: &String) -> bool {
    if arg.len() < 2 {
        return false
    }

    let first = arg.chars().next().unwrap();

    first == ':' && !arg.contains(" ")
}

pub fn is_value(arg: &String) -> bool {
    !is_register(&arg) && !is_address(&arg) && !is_label(&arg)
}