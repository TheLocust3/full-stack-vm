#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::instructions::register;

    #[test]
    fn test_set() {
        let mut cpu: CPU = CPU::new();

        let a = cpu.a;

        assert_eq!(cpu.set_a(register::set(a, 10)).a.value, 10);
    }
}