#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor_functions::execute_jump;
    use self::emu::cpu::executor_functions::execute_jump0;

    #[test]
    fn test_jump() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu.memory.write_64bit(1, 10);

        cpu = execute_jump(cpu, pc);

        assert_eq!(cpu.pc.value, 10);
        assert_eq!(cpu.f.value, 0);
    }

    #[test]
    fn test_jump0_false() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_pc(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu.memory.write_64bit(1, 10);

        let pc = cpu.pc;
        let a = cpu.a;

        cpu = execute_jump0(cpu, pc);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
    }

    #[test]
    fn test_jump0_true() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_pc(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 0 });

        cpu.memory.write_64bit(1, 10);

        let pc = cpu.pc;
        let a = cpu.a;

        cpu = execute_jump0(cpu, pc);

        assert_eq!(cpu.pc.value, 10);
        assert_eq!(cpu.f.value, 0);
    }
}