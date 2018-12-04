#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::Test;

    #[test]
    fn test_add() {
        assert_eq!(Test::add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(Test::bad_add(1, 2), 3);
    }
}