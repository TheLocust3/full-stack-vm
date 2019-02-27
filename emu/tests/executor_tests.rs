#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor;
    use self::emu::memory::TOTAL_MEMORY;

    // miscellaneous

    #[test]
    fn test_nop() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 0);
    }

    #[test]
    fn test_halt() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01010101);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.stopped, true);
    }

    #[test]
    fn test_push_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11001000);

        cpu = cpu.set_a(Register { value: 10 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11001001);

        cpu = cpu.set_b(Register { value: 10 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11001010);

        cpu = cpu.set_c(Register { value: 10 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11001011);

        cpu = cpu.set_d(Register { value: 10 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11001100);

        cpu = cpu.set_e(Register { value: 10 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11001101);

        cpu = cpu.set_f(Register { value: 10 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11001110);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11101000);

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11101001);

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11101010);

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11101011);

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11101100);

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }
    
    #[test]
    fn test_pop_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11101110);

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    // register

    #[test]
    fn test_set_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001000);

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 9);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_set_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001001);

        cpu = cpu.set_b(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 9);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_set_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001010);

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 9);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_set_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001011);

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 9);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_set_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001100);

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 9);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_set_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001110);

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 9);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }

        #[test]
    fn test_move_a_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00000001);

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_a_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00000010);

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_a_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00000011);

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_a_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00000100);

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_a_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00000101);

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_a_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00000110);

        cpu = cpu.set_a(Register { value: 0 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_move_b_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00010000);

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_b_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00010010);

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_b_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00010011);

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_b_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00010100);

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_b_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00010101);

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_b_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00010110);

        cpu = cpu.set_b(Register { value: 0 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 10);
    }

    #[test]
    fn test_move_c_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00100000);

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_c_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00100001);

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_c_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00100011);

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_c_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00100100);

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_c_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00100101);

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_c_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00100110);

        cpu = cpu.set_c(Register { value: 0 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 10);
    }

    #[test]
    fn test_move_d_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00110000);

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_d_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00110001);

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_d_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00110010);

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_d_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00110100);

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_d_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00110101);

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_d_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00110110);

        cpu = cpu.set_d(Register { value: 0 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 10);
    }

    #[test]
    fn test_move_e_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01000000);

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_move_e_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01000001);

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_move_e_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01000010);

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_move_e_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01000011);

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_move_e_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01000101);

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.e.value, 10);
    }
    
    #[test]
    fn test_move_e_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01000110);

        cpu = cpu.set_e(Register { value: 0 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 10);
    }

    #[test]
    fn test_move_hl_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01100000);

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }

    #[test]
    fn test_move_hl_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01100001);

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }

    #[test]
    fn test_move_hl_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01100010);

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }

    #[test]
    fn test_move_hl_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01100011);

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }

    #[test]
    fn test_move_hl_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01100100);

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 10);
    }
    
    #[test]
    fn test_move_hl_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01100101);

        cpu = cpu.set_hl(Register { value: 0 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 10);
        assert_eq!(cpu.hl.value, 10);
    }

    // arithmatic

    #[test]
    fn test_add_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001111);

        cpu = cpu.set_a(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
    }

    #[test]
    fn test_add_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00101111);

        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_add_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01001111);

        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_c(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_add_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01101111);

        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_d(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_add_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10001111);

        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_e(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_add_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10101111);
        
        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_f(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_add_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11001111);

        cpu = cpu.set_a(Register { value: 10 });
        cpu = cpu.set_hl(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 30);
    }

    #[test]
    fn test_sub_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00011111);

        cpu = cpu.set_a(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 0);
    }

    #[test]
    fn test_sub_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00111111);

        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_b(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_sub_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01011111);

        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_c(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_sub_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01111111);

        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_d(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_sub_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10011111);

        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_e(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_sub_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10111111);

        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_f(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_sub_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11011111);

        cpu = cpu.set_a(Register { value: 20 });
        cpu = cpu.set_hl(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    // bitwise

    #[test]
    fn test_and_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00011000);

        cpu = cpu.set_a(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_and_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00111000);
        
        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_b(Register { value: 345 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_and_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01011000);

        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_c(Register { value: 345 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_and_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01111000);

        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_d(Register { value: 345 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_and_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10011000);

        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_e(Register { value: 345 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_and_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10111000);

        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_f(Register { value: 345 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 345);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_and_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11011000);

        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_hl(Register { value: 345 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 89);
    }

    #[test]
    fn test_or_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00011001);

        cpu = cpu.set_a(Register { value: 10 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
    }

    #[test]
    fn test_or_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00111001);

        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_b(Register { value: 1000 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_or_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01011001);

        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_c(Register { value: 1000 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_or_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01111001);

        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_d(Register { value: 1000 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_or_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10011001);

        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_e(Register { value: 1000 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_or_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10111001);

        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_f(Register { value: 1000 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 1000);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_or_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11011001);

        cpu = cpu.set_a(Register { value: 255 });
        cpu = cpu.set_hl(Register { value: 1000 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1023);
    }

    #[test]
    fn test_not_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00011011);

        cpu = cpu.set_a(Register { value: 0 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, u64::max_value());
    }

    #[test]
    fn test_not_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00111011);

        cpu = cpu.set_b(Register { value: 0 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, u64::max_value());
    }

    #[test]
    fn test_not_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01011011);

        cpu = cpu.set_c(Register { value: 0 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, u64::max_value());
    }

    #[test]
    fn test_not_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01111011);

        cpu = cpu.set_d(Register { value: 0 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, u64::max_value());
    }

    #[test]
    fn test_not_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10011011);

        cpu = cpu.set_e(Register { value: 0 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, u64::max_value());
    }

    #[test]
    fn test_not_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11011011);

        cpu = cpu.set_hl(Register { value: 0 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, u64::max_value());
    }

    #[test]
    fn test_shift_left_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00011101);

        cpu = cpu.set_a(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 2);
    }

    #[test]
    fn test_shift_left_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00111101);

        cpu = cpu.set_b(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 2);
    }

    #[test]
    fn test_shift_left_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01011101);

        cpu = cpu.set_c(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 2);
    }

    #[test]
    fn test_shift_left_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01111101);

        cpu = cpu.set_d(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 2);
    }

    #[test]
    fn test_shift_left_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10011101);

        cpu = cpu.set_e(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 2);
    }

    #[test]
    fn test_shift_left_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11011101);

        cpu = cpu.set_hl(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 2);
    }

    #[test]
    fn test_shift_left_a_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00000111);

        cpu = cpu.set_a(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 2);
    }

    #[test]
    fn test_shift_left_b_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00100111);

        cpu = cpu.set_b(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 2);
    }

    #[test]
    fn test_shift_left_c_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01000111);

        cpu = cpu.set_c(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 2);
    }

    #[test]
    fn test_shift_left_d_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01100111);

        cpu = cpu.set_d(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 2);
    }

    #[test]
    fn test_shift_left_e_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10000111);

        cpu = cpu.set_e(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 2);
    }

    #[test]
    fn test_shift_left_hl_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11000111);

        cpu = cpu.set_hl(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 2);
    }

    #[test]
    fn test_shift_right_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00011110);

        cpu = cpu.set_a(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1);
    }

    #[test]
    fn test_shift_right_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00111110);

        cpu = cpu.set_b(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 1);
    }

    #[test]
    fn test_shift_right_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01011110);

        cpu = cpu.set_c(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 1);
    }

    #[test]
    fn test_shift_right_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01111110);

        cpu = cpu.set_d(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 1);
    }

    #[test]
    fn test_shift_right_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10011110);

        cpu = cpu.set_e(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 1);
    }

    #[test]
    fn test_shift_right_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11011110);

        cpu = cpu.set_hl(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 1);
    }

    #[test]
    fn test_shift_right_a_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00010111);

        cpu = cpu.set_a(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1);
    }

    #[test]
    fn test_shift_right_b_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00110111);

        cpu = cpu.set_b(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 1);
    }

    #[test]
    fn test_shift_right_c_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01010111);

        cpu = cpu.set_c(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 1);
    }

    #[test]
    fn test_shift_right_d_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01110111);

        cpu = cpu.set_d(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 1);
    }

    #[test]
    fn test_shift_right_e_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10010111);

        cpu = cpu.set_e(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 1);
    }

    #[test]
    fn test_shift_right_hl_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11010111);

        cpu = cpu.set_hl(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 1);
    }

    #[test]
    fn test_jump() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11111111);

        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 10);
        assert_eq!(cpu.f.value, 0);
    }

    #[test]
    fn test_jump0_false() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11111110);
        
        cpu = cpu.set_pc(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 10 });

        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
    }

    #[test]
    fn test_jump0_true() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11111110);

        cpu = cpu.set_pc(Register { value: 0 });
        cpu = cpu.set_a(Register { value: 0 });

        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 10);
        assert_eq!(cpu.f.value, 0);
    }

    #[test]
    fn test_read8_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11111000);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_8bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
    }

    #[test]
    fn test_read8_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11111001);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_8bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
    }

    #[test]
    fn test_read8_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11111010);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_8bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
    }

    #[test]
    fn test_read8_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11111011);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_8bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
    }

    #[test]
    fn test_read8_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11111100);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_8bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
    }

    #[test]
    fn test_read16_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11110000);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
    }

    #[test]
    fn test_read16_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11110001);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
    }

    #[test]
    fn test_read16_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11110010);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
    }

    #[test]
    fn test_read16_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11110011);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
    }

    #[test]
    fn test_read16_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11110100);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_16bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
    }

    fn test_read32_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11100000);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
    }

    #[test]
    fn test_read32_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11100001);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
    }

    #[test]
    fn test_read32_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11100010);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
    }

    #[test]
    fn test_read32_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11100011);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
    }

    #[test]
    fn test_read32_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11100100);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_32bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
    }

    fn test_read64_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11000000);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
    }

    #[test]
    fn test_read64_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11000001);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
    }

    #[test]
    fn test_read64_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11000010);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
    }

    #[test]
    fn test_read64_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11000011);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
    }

    #[test]
    fn test_read64_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11000100);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu.memory.write_64bit(10, 20);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
    }

    #[test]
    fn test_write8_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10000000);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write8_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10000001);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write8_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10000010);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_c(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write8_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10000011);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_d(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write8_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10000100);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_e(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write8_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10000101);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_f(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 20);
        assert_eq!(cpu.memory.read_8bit(10), 20);
    }

    #[test]
    fn test_write16_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10001000);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write16_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10001001);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write16_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10001010);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_c(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write16_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10001011);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_d(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write16_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10001100);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_e(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write16_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10001101);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_f(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 20);
        assert_eq!(cpu.memory.read_16bit(10), 20);
    }

    #[test]
    fn test_write32_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10010000);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write32_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10010001);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write32_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10010010);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_c(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write32_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10010011);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_d(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write32_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10010100);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_e(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write32_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10010101);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_f(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 20);
        assert_eq!(cpu.memory.read_32bit(10), 20);
    }

    #[test]
    fn test_write64_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10101000);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_a(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }

    #[test]
    fn test_write64_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10101001);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_b(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }

    #[test]
    fn test_write64_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10101010);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_c(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }

    #[test]
    fn test_write64_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10101011);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_d(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }

    #[test]
    fn test_write64_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10101100);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_e(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }

    #[test]
    fn test_write64_f() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10101101);

        cpu = cpu.set_hl(Register { value: 10 });
        cpu = cpu.set_f(Register { value: 20 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 20);
        assert_eq!(cpu.memory.read_64bit(10), 20);
    }
}