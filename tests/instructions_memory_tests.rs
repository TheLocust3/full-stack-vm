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

        let hl = cpu.hl;

        let memoryReturn = memory::read8(cpu);

        assert_eq!(memoryReturn.value, 20);
        assert_eq!(memoryReturn.address, 0);
        assert_eq!(memoryReturn.overflow, false);
        assert_eq!(memoryReturn.negative, false);
    }

    #[test]
    fn test_read16() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        let hl = cpu.hl;

        let memoryReturn = memory::read16(cpu);

        assert_eq!(memoryReturn.value, 20);
        assert_eq!(memoryReturn.address, 0);
        assert_eq!(memoryReturn.overflow, false);
        assert_eq!(memoryReturn.negative, false);
    }

    #[test]
    fn test_read32() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        let hl = cpu.hl;

        let memoryReturn = memory::read32(cpu);

        assert_eq!(memoryReturn.value, 20);
        assert_eq!(memoryReturn.address, 0);
        assert_eq!(memoryReturn.overflow, false);
        assert_eq!(memoryReturn.negative, false);
    }

    #[test]
    fn test_read64() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        let hl = cpu.hl;

        let memoryReturn = memory::read64(cpu);

        assert_eq!(memoryReturn.value, 20);
        assert_eq!(memoryReturn.address, 0);
        assert_eq!(memoryReturn.overflow, false);
        assert_eq!(memoryReturn.negative, false);
    }
}