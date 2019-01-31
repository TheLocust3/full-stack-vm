#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::instructions::bitwise;

    #[test]
    fn test_and() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_b(Register { value: 345 });

        let a = cpu.a;
        let b = cpu.b;

        let register_return = bitwise::and(a, b);

        assert_eq!(register_return.out.value, 89);
    }

    #[test]
    fn test_or() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_b(Register { value: 1000 });

        let a = cpu.a;
        let b = cpu.b;

        let register_return = bitwise::or(a, b);

        assert_eq!(register_return.out.value, 1023);
    }

    #[test]
    fn test_not() {
        let cpu: CPU = CPU::new();

        let a = cpu.a;

        let register_return = bitwise::not(a);

        assert_eq!(register_return.out.value, u64::max_value());
    }

    #[test]
    fn test_shift_left() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 1 });

        let a = cpu.a;

        let register_return = bitwise::shift_left(a, false);

        assert_eq!(register_return.out.value, 2);
        assert_eq!(register_return.overflow, false);
    }
    
    #[test]
    fn test_shift_left_max() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: u64::max_value() });

        let a = cpu.a;

        let register_return = bitwise::shift_left(a, false);

        assert_eq!(register_return.out.value, 18446744073709551614);
        assert_eq!(register_return.overflow, true);
    }

    #[test]
    fn test_shift_left_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 1 });

        let a = cpu.a;

        let register_return = bitwise::shift_left(a, true);

        assert_eq!(register_return.out.value, 2);
        assert_eq!(register_return.overflow, false);
    }

    #[test]
    fn test_shift_left_wrap_max() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: u64::max_value() });

        let a = cpu.a;

        let register_return = bitwise::shift_left(a, true);

        assert_eq!(register_return.out.value, u64::max_value());
        assert_eq!(register_return.overflow, false);
    }

    #[test]
    fn test_shift_right() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 2 });

        let a = cpu.a;

        let register_return = bitwise::shift_right(a, false);

        assert_eq!(register_return.out.value, 1);
        assert_eq!(register_return.overflow, false);
    }
    
    #[test]
    fn test_shift_right_max() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: u64::max_value() });

        let a = cpu.a;

        let register_return = bitwise::shift_right(a, false);

        assert_eq!(register_return.out.value, 9223372036854775807);
        assert_eq!(register_return.overflow, true);
    }

    #[test]
    fn test_shift_right_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 2 });

        let a = cpu.a;

        let register_return = bitwise::shift_right(a, true);

        assert_eq!(register_return.out.value, 1);
        assert_eq!(register_return.overflow, false);
    }

    #[test]
    fn test_shift_right_wrap_max() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: u64::max_value() });

        let a = cpu.a;

        let register_return = bitwise::shift_right(a, true);

        assert_eq!(register_return.out.value, u64::max_value());
        assert_eq!(register_return.overflow, false);
    }
}