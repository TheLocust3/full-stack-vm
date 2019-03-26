#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor_functions::execute_jump;
    use self::emu::cpu::executor_functions::execute_jump0;

    #[test]
    fn test_jump_a() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_a(Register { value: 10 });

        cpu = execute_jump(cpu, 0b000);

        assert_eq!(cpu.pc.value, 10);
        assert_eq!(cpu.a.value, 10);
        assert_eq!(cpu.f.value, 0);
    }

    #[test]
    fn test_jump0_b_false() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_pc(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 100 });

        let pc = cpu.pc;
        let a = cpu.a;

        cpu = execute_jump0(cpu, pc, 0b001);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.a.value, 10);
        assert_eq!(cpu.b.value, 100);
        assert_eq!(cpu.f.value, 0);
    }

    #[test]
    fn test_jump0_b_true() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_pc(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 100 });

        let pc = cpu.pc;
        let a = cpu.a;

        cpu = execute_jump0(cpu, pc, 0b001);

        assert_eq!(cpu.pc.value, 100);
        assert_eq!(cpu.a.value, 0);
        assert_eq!(cpu.b.value, 100);
        assert_eq!(cpu.f.value, 0);
    }
}