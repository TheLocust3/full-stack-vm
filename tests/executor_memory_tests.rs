#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor::execute_read8;
    use self::emu::cpu::executor::execute_read16;
    use self::emu::cpu::executor::execute_read32;
    use self::emu::cpu::executor::execute_read64;

    #[test]
    fn test_read8_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_8bit(10, 20);

        cpu = execute_read8(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
    }

    #[test]
    fn test_read8_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_8bit(10, 20);

        cpu = execute_read8(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
    }

    #[test]
    fn test_read8_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_8bit(10, 20);

        cpu = execute_read8(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
    }

    #[test]
    fn test_read8_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_8bit(10, 20);

        cpu = execute_read8(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
    }

    #[test]
    fn test_read8_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_8bit(10, 20);

        cpu = execute_read8(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
    }

    #[test]
    fn test_read16_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        cpu = execute_read16(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
    }

    #[test]
    fn test_read16_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        cpu = execute_read16(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
    }

    #[test]
    fn test_read16_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        cpu = execute_read16(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
    }

    #[test]
    fn test_read16_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        cpu = execute_read16(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
    }

    #[test]
    fn test_read16_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        cpu = execute_read16(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
    }

    fn test_read32_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        cpu = execute_read32(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
    }

    #[test]
    fn test_read32_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        cpu = execute_read32(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
    }

    #[test]
    fn test_read32_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        cpu = execute_read32(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
    }

    #[test]
    fn test_read32_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        cpu = execute_read32(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
    }

    #[test]
    fn test_read32_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        cpu = execute_read32(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
    }

    fn test_read64_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        cpu = execute_read64(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
    }

    #[test]
    fn test_read64_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        cpu = execute_read64(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
    }

    #[test]
    fn test_read64_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        cpu = execute_read64(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
    }

    #[test]
    fn test_read64_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        cpu = execute_read64(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
    }

    #[test]
    fn test_read64_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        cpu = execute_read64(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
    }
}