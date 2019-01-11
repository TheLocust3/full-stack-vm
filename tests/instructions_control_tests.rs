#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::instructions::control;

    #[test]
    fn test_jump() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_pc(Register { value: 0 });

        let pc = cpu.pc;

        let register_return = control::jump(pc, 100);

        assert_eq!(register_return.out.value, 100);
    }

    #[test]
    fn test_jump0_false() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_pc(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        let pc = cpu.pc;
        let a = cpu.a;

        let register_return = control::jump0(pc, a, 100);

        assert_eq!(register_return.out.value, 0);
    }

    #[test]
    fn test_jump0_true() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_pc(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 0 });

        let pc = cpu.pc;
        let a = cpu.a;

        let register_return = control::jump0(pc, a, 100);

        assert_eq!(register_return.out.value, 100);
    }
}