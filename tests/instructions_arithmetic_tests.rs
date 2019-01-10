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
    }
}