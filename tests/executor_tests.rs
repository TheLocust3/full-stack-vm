#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor;

    #[test]
    fn test_nop() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 0);
    }

    #[test]
    fn test_set_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001000);

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_set_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001001);

        cpu = cpu.set_b(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_set_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001010);

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_set_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001011);

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_set_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001100);

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_set_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001110);

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }
}