#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::memory::Memory;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::cpu::CPU;

    #[test]
    fn test_new() {
        let cpu: CPU = CPU::new();

        assert_eq!(cpu.a.value, 0);
    }

    #[test]
    fn test_set_a() {
        let cpu: CPU = CPU::new();

        assert_eq!(cpu.set_a(Register { value: 10 }).a.value, 10);
    }

    #[test]
    fn test_set_b() {
        let cpu: CPU = CPU::new();

        assert_eq!(cpu.set_b(Register { value: 10 }).b.value, 10);
    }

    #[test]
    fn test_set_c() {
        let cpu: CPU = CPU::new();

        assert_eq!(cpu.set_c(Register { value: 10 }).c.value, 10);
    }

    #[test]
    fn test_set_d() {
        let cpu: CPU = CPU::new();

        assert_eq!(cpu.set_d(Register { value: 10 }).d.value, 10);
    }

    #[test]
    fn test_set_e() {
        let cpu: CPU = CPU::new();

        assert_eq!(cpu.set_e(Register { value: 10 }).e.value, 10);
    }

    #[test]
    fn test_set_f() {
        let cpu: CPU = CPU::new();

        assert_eq!(cpu.set_f(Register { value: 10 }).f.value, 10);
    }

    #[test]
    fn test_set_hl() {
        let cpu: CPU = CPU::new();

        assert_eq!(cpu.set_hl(Register { value: 10 }).hl.value, 10);
    }

    #[test]
    fn test_set_pc() {
        let cpu: CPU = CPU::new();

        assert_eq!(cpu.set_pc(Register { value: 10 }).pc.value, 10);
    }
}
