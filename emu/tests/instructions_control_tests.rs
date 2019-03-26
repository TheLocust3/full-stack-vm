#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::instructions::control;

    #[test]
    fn test_jump_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_pc(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        let register_return = control::jump(cpu.a);

        assert_eq!(register_return.out.value, 10);
    }

    #[test]
    fn test_jump0_b_false() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_pc(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 100 });

        let pc = cpu.pc;
        let a = cpu.a;
        let b = cpu.b;

        let register_return = control::jump0(pc, a, b);

        assert_eq!(register_return.out.value, 1);
    }

    #[test]
    fn test_jump0_b_true() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_pc(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 100 });

        let pc = cpu.pc;
        let a = cpu.a;
        let b = cpu.b;

        let register_return = control::jump0(pc, a, b);

        assert_eq!(register_return.out.value, 100);
    }
}