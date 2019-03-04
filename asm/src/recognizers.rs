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

    is_address(&arg) && !is_address_register(&arg)
}

pub fn is_address_register(arg: &String) -> bool {
    if arg.len() < 3 {
        return false
    }

    let reg: String = arg.chars().skip(1).take(arg.len() - 2).collect::<String>();
    println!("TEST");
    println!("{}", arg);
    println!("{}", reg);

    is_address(&arg) && is_register(&reg)
}

pub fn is_value(arg: &String) -> bool {
    !is_register(&arg) && !is_address(&arg)
}