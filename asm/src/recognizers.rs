pub fn is_register(arg: &String) -> bool {
    arg == "A" || arg == "B" || arg == "C" || arg == "D" || arg == "E"
}

pub fn is_address(arg: &String) -> bool {
    let first = arg.chars().next().unwrap();
    let last = arg.chars().last().unwrap();

    first == '(' && last  == ')'
}

pub fn is_value(arg: &String) -> bool {
    !is_register(&arg) && !is_address(&arg)
}