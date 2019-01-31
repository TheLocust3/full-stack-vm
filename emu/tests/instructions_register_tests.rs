#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::instructions::register;

    #[test]
    fn test_set() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });

        let a = cpu.a;

        let register_return = register::set(a, 11);

        assert_eq!(register_return.out.value, 11);
    }

    #[test]
    fn test_move_reg() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        let a = cpu.a;
        let b = cpu.b;

        let register_return = register::move_reg(a, b);

        assert_eq!(register_return.out.value, 20);
    }
}