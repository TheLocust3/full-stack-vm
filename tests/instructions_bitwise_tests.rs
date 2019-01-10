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

        let registerReturn = bitwise::and(a, b);

        assert_eq!(registerReturn.out.value, 89);
    }

    #[test]
    fn test_or() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_b(Register { value: 1000 });

        let a = cpu.a;
        let b = cpu.b;

        let registerReturn = bitwise::or(a, b);

        assert_eq!(registerReturn.out.value, 1023);
    }

    #[test]
    fn test_not() {
        let cpu: CPU = CPU::new();

        let a = cpu.a;

        let registerReturn = bitwise::not(a);

        assert_eq!(registerReturn.out.value, u64::max_value());
    }
}