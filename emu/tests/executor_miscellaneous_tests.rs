#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor_functions::execute_push;
    use self::emu::cpu::executor_functions::execute_pop;
    use self::emu::memory::TOTAL_MEMORY;

    #[test]
    fn test_push_a() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_a(Register { value: 10 });
        cpu = execute_push(cpu, 0b000);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_b() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_b(Register { value: 10 });
        cpu = execute_push(cpu, 0b001);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_c() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_c(Register { value: 10 });
        cpu = execute_push(cpu, 0b010);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_d() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_d(Register { value: 10 });
        cpu = execute_push(cpu, 0b011);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_e() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_e(Register { value: 10 });
        cpu = execute_push(cpu, 0b100);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_f() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_f(Register { value: 10 });
        cpu = execute_push(cpu, 0b101);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_hl() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = execute_push(cpu, 0b110);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_a() {
        let mut cpu: CPU = CPU::new();

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = execute_pop(cpu, 0b000);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_b() {
        let mut cpu: CPU = CPU::new();

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = execute_pop(cpu, 0b001);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_c() {
        let mut cpu: CPU = CPU::new();

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = execute_pop(cpu, 0b010);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_d() {
        let mut cpu: CPU = CPU::new();

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = execute_pop(cpu, 0b011);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_e() {
        let mut cpu: CPU = CPU::new();

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = execute_pop(cpu, 0b100);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_hl() {
        let mut cpu: CPU = CPU::new();

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = execute_pop(cpu, 0b110);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }
}