use recognizers::is_address;

pub fn parse_address(address: String) -> String {
    if is_address(&address) {
        address.chars().skip(1).take(address.len() - 2).collect::<String>()
    } else {
        address
    }
}
