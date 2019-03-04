pub fn parse_address(address: String) -> String {
    address.chars().skip(1).take(address.len() - 2).collect::<String>()
}
