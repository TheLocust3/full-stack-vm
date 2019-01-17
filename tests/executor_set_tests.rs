#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor::execute_set;

    #[test]
    fn test_set_a() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b000);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_set_b() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_b(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b001);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_set_c() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_c(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b010);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_set_d() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_d(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b011);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_set_e() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_e(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b100);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_set_hl() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_hl(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b110);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }
}