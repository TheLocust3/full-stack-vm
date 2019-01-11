#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::instructions::arithmetic;

    #[test]
    fn test_add() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        let a = cpu.a;
        let b = cpu.b;

        let registerReturn = arithmetic::add(a, b);

        assert_eq!(registerReturn.out.value, 30);
        assert_eq!(registerReturn.overflow, false);
        assert_eq!(registerReturn.negative, false);
    }

    #[test]
    fn test_add_overflow() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: u64::max_value() });
        cpu = cpu.set_b(Register { value: 20 });

        let a = cpu.a;
        let b = cpu.b;

        let registerReturn = arithmetic::add(a, b);

        assert_eq!(registerReturn.out.value, u64::max_value());
        assert_eq!(registerReturn.overflow, true);
        assert_eq!(registerReturn.negative, false);
    }

    #[test]
    fn test_add_overflow_edge() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: u64::max_value() - 1 });
        cpu = cpu.set_b(Register { value: 1 });

        let a = cpu.a;
        let b = cpu.b;

        let registerReturn = arithmetic::add(a, b);

        assert_eq!(registerReturn.out.value, u64::max_value());
        assert_eq!(registerReturn.overflow, false);
        assert_eq!(registerReturn.negative, false);
    }

    #[test]
    fn test_sub() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_b(Register { value: 10 });

        let a = cpu.a;
        let b = cpu.b;

        let registerReturn = arithmetic::sub(a, b);

        assert_eq!(registerReturn.out.value, 10);
        assert_eq!(registerReturn.overflow, false);
        assert_eq!(registerReturn.negative, false);
    }

    #[test]
    fn test_negative() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        let a = cpu.a;
        let b = cpu.b;

        let registerReturn = arithmetic::sub(a, b);

        assert_eq!(registerReturn.out.value, 0);
        assert_eq!(registerReturn.overflow, false);
        assert_eq!(registerReturn.negative, true);
    }

    #[test]
    fn test_negative_edge() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 2 });
        cpu = cpu.set_b(Register { value: 2 });

        let a = cpu.a;
        let b = cpu.b;

        let registerReturn = arithmetic::sub(a, b);

        assert_eq!(registerReturn.out.value, 0);
        assert_eq!(registerReturn.overflow, false);
        assert_eq!(registerReturn.negative, false);
    }
}