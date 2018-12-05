#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::bitwise;

    // to_16bit

    #[test]
    fn test_to_16bit() {
        assert_eq!(bitwise::to_16bit(123u8, 10u8), 0b0111101100001010);
    }

    #[test]
    fn test_to_16bit_0() {
        assert_eq!(bitwise::to_16bit(0u8, 0u8), 0u16);
    }

    #[test]
    fn test_to_16bit_255() {
        assert_eq!(bitwise::to_16bit(255u8, 255u8), 0xFFFF);
    }

    // to_32bit

    #[test]
    fn test_to_32bit() {
        assert_eq!(bitwise::to_32bit(123u8, 10u8, 143u8, 32u8),
                   0b01111011000010101000111100100000);
    }

    #[test]
    fn test_to_32bit_0() {
        assert_eq!(bitwise::to_32bit(0u8, 0u8, 0u8, 0u8), 0u32);
    }

    #[test]
    fn test_to_32bit_255() {
        assert_eq!(bitwise::to_32bit(255u8, 255u8, 255u8, 255u8), 0xFFFFFFFF);
    }

    // to_64bit

    #[test]
    fn test_to_64bit() {
        assert_eq!(bitwise::to_64bit(123u8, 10u8, 143u8, 32u8, 36u8, 86u8, 201u8, 1u8),
                   0b0111101100001010100011110010000000100100010101101100100100000001);
    }

    #[test]
    fn test_to_64bit_0() {
        assert_eq!(bitwise::to_64bit(0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8), 0u64);
    }

    #[test]
    fn test_to_64bit_255() {
        assert_eq!(bitwise::to_64bit(255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8),
                   0xFFFFFFFFFFFFFFFF);
    }

    // to_bytes_16bit

    #[test]
    fn test_to_bytes_16bit() {
        assert_eq!(bitwise::to_bytes_16bit(0xaf03u16), [175u8, 3u8]);
    }

    #[test]
    fn test_to_bytes_16bit_0() {
        assert_eq!(bitwise::to_bytes_16bit(0u16), [0u8, 0u8]);
    }

    #[test]
    fn test_to_bytes_16bit_max() {
        assert_eq!(bitwise::to_bytes_16bit(65535u16), [255u8, 255u8]);
    }

    // to_bytes_32bit

    #[test]
    fn test_to_bytes_32bit() {
        assert_eq!(bitwise::to_bytes_32bit(0xd65af062u32), [214u8, 90u8, 240u8, 98u8]);
    }

    #[test]
    fn test_to_bytes_32bit_0() {
        assert_eq!(bitwise::to_bytes_32bit(0u32), [0u8, 0u8, 0u8, 0u8]);
    }

    #[test]
    fn test_to_bytes_32bit_max() {
        assert_eq!(bitwise::to_bytes_32bit(4294967295u32), [255u8, 255u8, 255u8, 255u8]);
    }

    // to_bytes_64bit

    #[test]
    fn test_to_bytes_64bit() {
        assert_eq!(bitwise::to_bytes_64bit(0x722fff7faf641fafu64), [114u8, 47u8, 255u8, 127u8, 175u8, 100u8, 31u8, 175u8]);
    }

    #[test]
    fn test_to_bytes_64bit_0() {
        assert_eq!(bitwise::to_bytes_64bit(0u64), [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
    }

    #[test]
    fn test_to_bytes_64bit_max() {
        assert_eq!(bitwise::to_bytes_64bit(18446744073709551615u64), [255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8]);
    }
}