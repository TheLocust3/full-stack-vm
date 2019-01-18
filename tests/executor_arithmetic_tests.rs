#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor::execute_add;
    use self::emu::cpu::executor::execute_sub;

    #[test]
    fn test_add_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });

        cpu = execute_add(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
    }

    #[test]
    fn test_add_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        cpu = execute_add(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_add_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_c(Register { value: 20 });

        cpu = execute_add(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_add_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_d(Register { value: 20 });

        cpu = execute_add(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_add_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_e(Register { value: 20 });

        cpu = execute_add(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_add_f() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_f(Register { value: 20 });

        cpu = execute_add(cpu, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_add_hl() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_hl(Register { value: 20 });

        cpu = execute_add(cpu, 0b110);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_sub_a() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 10 });

        cpu = execute_sub(cpu, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 0);
    }

    #[test]
    fn test_sub_b() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = execute_sub(cpu, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_sub_c() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = execute_sub(cpu, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_sub_d() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = execute_sub(cpu, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_sub_e() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = execute_sub(cpu, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_sub_f() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = execute_sub(cpu, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_sub_hl() {
        let mut cpu: CPU = CPU::new();
        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = execute_sub(cpu, 0b110);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }
}