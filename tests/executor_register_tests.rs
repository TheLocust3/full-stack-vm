#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor_functions::execute_set;
    use self::emu::cpu::executor_functions::execute_move;

    #[test]
    fn test_set_a() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b000);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_set_b() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_b(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b001);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_set_c() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_c(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b010);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_set_d() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_d(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b011);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_set_e() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_e(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b100);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_set_hl() {
        let mut cpu: CPU = CPU::new();
        let pc = cpu.pc;

        cpu = cpu.set_hl(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = execute_set(cpu, pc, 0b110);

        assert_eq!(cpu.pc.value, 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }

    #[test]
    fn test_move_a_b() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = execute_move(cpu, 0b000, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_a_c() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = execute_move(cpu, 0b000, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_a_d() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = execute_move(cpu, 0b000, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_a_e() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = execute_move(cpu, 0b000, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_a_f() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = execute_move(cpu, 0b000, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_a_hl() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = execute_move(cpu, 0b000, 0b110);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_b_a() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu = execute_move(cpu, 0b001, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_b_c() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = execute_move(cpu, 0b001, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_b_d() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = execute_move(cpu, 0b001, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_b_e() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = execute_move(cpu, 0b001, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_b_f() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = execute_move(cpu, 0b001, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_b_hl() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = execute_move(cpu, 0b001, 0b110);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_c_a() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu = execute_move(cpu, 0b010, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_c_b() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = execute_move(cpu, 0b010, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_c_d() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = execute_move(cpu, 0b010, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_c_e() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = execute_move(cpu, 0b010, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_c_f() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = execute_move(cpu, 0b010, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_c_hl() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = execute_move(cpu, 0b010, 0b110);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_d_a() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu = execute_move(cpu, 0b011, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_d_b() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = execute_move(cpu, 0b011, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_d_c() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = execute_move(cpu, 0b011, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_d_e() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = execute_move(cpu, 0b011, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_d_f() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = execute_move(cpu, 0b011, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_d_hl() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = execute_move(cpu, 0b011, 0b110);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_e_a() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu = execute_move(cpu, 0b100, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_move_e_b() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = execute_move(cpu, 0b100, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_move_e_c() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = execute_move(cpu, 0b100, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_move_e_d() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = execute_move(cpu, 0b100, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_move_e_f() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = execute_move(cpu, 0b100, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.e.value, 10);
    }
    
    #[test]
    fn test_move_e_hl() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = execute_move(cpu, 0b100, 0b110);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_move_hl_a() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu = execute_move(cpu, 0b110, 0b000);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }

    #[test]
    fn test_move_hl_b() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = execute_move(cpu, 0b110, 0b001);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }

    #[test]
    fn test_move_hl_c() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = execute_move(cpu, 0b110, 0b010);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }

    #[test]
    fn test_move_hl_d() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = execute_move(cpu, 0b110, 0b011);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }

    #[test]
    fn test_move_hl_e() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = execute_move(cpu, 0b110, 0b100);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }
    
    #[test]
    fn test_move_hl_f() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = execute_move(cpu, 0b110, 0b101);

        assert_eq!(cpu.pc.value, 0);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.hl.value, 10);
    }
}