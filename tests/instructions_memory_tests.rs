#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::instructions::memory;

    #[test]
    fn test_read8() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_8bit(10, 20);

        let memory_return = memory::read8(&cpu);

        assert_eq!(memory_return.value, 20);
        assert_eq!(memory_return.address, 0);
        assert_eq!(memory_return.overflow, false);
        assert_eq!(memory_return.negative, false);
    }

    #[test]
    fn test_read16() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        let memory_return = memory::read16(&cpu);

        assert_eq!(memory_return.value, 20);
        assert_eq!(memory_return.address, 0);
        assert_eq!(memory_return.overflow, false);
        assert_eq!(memory_return.negative, false);
    }

    #[test]
    fn test_read32() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        let memory_return = memory::read32(&cpu);

        assert_eq!(memory_return.value, 20);
        assert_eq!(memory_return.address, 0);
        assert_eq!(memory_return.overflow, false);
        assert_eq!(memory_return.negative, false);
    }

    #[test]
    fn test_read64() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        let memory_return = memory::read64(&cpu);

        assert_eq!(memory_return.value, 20);
        assert_eq!(memory_return.address, 0);
        assert_eq!(memory_return.overflow, false);
        assert_eq!(memory_return.negative, false);
    }

    #[test]
    fn test_write8() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        let hl = cpu.hl;
        let a = cpu.a;

        let memory_return = memory::write8(hl, a);

        assert_eq!(memory_return.value, 20);
        assert_eq!(memory_return.address, 10);
        assert_eq!(memory_return.overflow, false);
        assert_eq!(memory_return.negative, false);
    }

    #[test]
    fn test_write16() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        let hl = cpu.hl;
        let a = cpu.a;

        let memory_return = memory::write16(hl, a);

        assert_eq!(memory_return.value, 20);
        assert_eq!(memory_return.address, 10);
        assert_eq!(memory_return.overflow, false);
        assert_eq!(memory_return.negative, false);
    }

    #[test]
    fn test_write32() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        let hl = cpu.hl;
        let a = cpu.a;

        let memory_return = memory::write32(hl, a);

        assert_eq!(memory_return.value, 20);
        assert_eq!(memory_return.address, 10);
        assert_eq!(memory_return.overflow, false);
        assert_eq!(memory_return.negative, false);
    }

    #[test]
    fn test_write64() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        let hl = cpu.hl;
        let a = cpu.a;

        let memory_return = memory::write64(hl, a);

        assert_eq!(memory_return.value, 20);
        assert_eq!(memory_return.address, 10);
        assert_eq!(memory_return.overflow, false);
        assert_eq!(memory_return.negative, false);
    }
}