#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor::execute_and;
    use self::emu::cpu::executor::execute_or;
    use self::emu::cpu::executor::execute_not;

    #[test]
    fn test_and_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });

        cpu = execute_and(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_and_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_b(Register { value: 345 });

        cpu = execute_and(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_and_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_c(Register { value: 345 });

        cpu = execute_and(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_and_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_d(Register { value: 345 });

        cpu = execute_and(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_and_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_e(Register { value: 345 });

        cpu = execute_and(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_and_f() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_f(Register { value: 345 });

        cpu = execute_and(cpu, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 345);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_and_hl() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_hl(Register { value: 345 });

        cpu = execute_and(cpu, 0b110);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_or_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });

        cpu = execute_or(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_or_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_b(Register { value: 1000 });

        cpu = execute_or(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_or_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_c(Register { value: 1000 });

        cpu = execute_or(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_or_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_d(Register { value: 1000 });

        cpu = execute_or(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_or_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_e(Register { value: 1000 });

        cpu = execute_or(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_or_f() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_f(Register { value: 1000 });

        cpu = execute_or(cpu, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 1000);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_or_hl() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_hl(Register { value: 1000 });

        cpu = execute_or(cpu, 0b110);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_not_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 0 });

        cpu = execute_not(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, u64::max_value());
    }

    #[test]
    fn test_not_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_b(Register { value: 0 });

        cpu = execute_not(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, u64::max_value());
    }

    #[test]
    fn test_not_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_c(Register { value: 0 });

        cpu = execute_not(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, u64::max_value());
    }

    #[test]
    fn test_not_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_d(Register { value: 0 });

        cpu = execute_not(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, u64::max_value());
    }

    #[test]
    fn test_not_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_e(Register { value: 0 });

        cpu = execute_not(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, u64::max_value());
    }

    #[test]
    fn test_not_hl() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_hl(Register { value: 0 });

        cpu = execute_not(cpu, 0b110);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, u64::max_value());
    }
}
