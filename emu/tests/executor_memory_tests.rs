#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor_functions::execute_read8;
    use self::emu::cpu::executor_functions::execute_read16;
    use self::emu::cpu::executor_functions::execute_read32;
    use self::emu::cpu::executor_functions::execute_read64;
    use self::emu::cpu::executor_functions::execute_write8;
    use self::emu::cpu::executor_functions::execute_write16;
    use self::emu::cpu::executor_functions::execute_write32;
    use self::emu::cpu::executor_functions::execute_write64;

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

    #[test]
    fn test_write8_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        cpu = execute_write8(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write8_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        cpu = execute_write8(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write8_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_c(Register { value: 20 });

        cpu = execute_write8(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write8_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_d(Register { value: 20 });

        cpu = execute_write8(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write8_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_e(Register { value: 20 });

        cpu = execute_write8(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write8_f() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_f(Register { value: 20 });

        cpu = execute_write8(cpu, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write16_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        cpu = execute_write16(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write16_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        cpu = execute_write16(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write16_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_c(Register { value: 20 });

        cpu = execute_write16(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write16_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_d(Register { value: 20 });

        cpu = execute_write16(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write16_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_e(Register { value: 20 });

        cpu = execute_write16(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write16_f() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_f(Register { value: 20 });

        cpu = execute_write16(cpu, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write32_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        cpu = execute_write32(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write32_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        cpu = execute_write32(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write32_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_c(Register { value: 20 });

        cpu = execute_write32(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write32_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_d(Register { value: 20 });

        cpu = execute_write32(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write32_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_e(Register { value: 20 });

        cpu = execute_write32(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write32_f() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_f(Register { value: 20 });

        cpu = execute_write32(cpu, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write64_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        cpu = execute_write64(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }

    #[test]
    fn test_write64_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        cpu = execute_write64(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }

    #[test]
    fn test_write64_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_c(Register { value: 20 });

        cpu = execute_write64(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }

    #[test]
    fn test_write64_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_d(Register { value: 20 });

        cpu = execute_write64(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }

    #[test]
    fn test_write64_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_e(Register { value: 20 });

        cpu = execute_write64(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }

    #[test]
    fn test_write64_f() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_f(Register { value: 20 });

        cpu = execute_write64(cpu, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }
}