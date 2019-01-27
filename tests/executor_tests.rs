#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::cpu::executor;

    #[test]
    fn test_nop() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 0);
    }

    // register

    #[test]
    fn test_set_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00001000);

        cpu = cpu.set_a(Register { value: 0 });
        cpu.memory.write_64bit(1, 10);

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 8);
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

        assert_eq!(cpu.pc.value, 8);
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

        assert_eq!(cpu.pc.value, 8);
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

        assert_eq!(cpu.pc.value, 8);
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

        assert_eq!(cpu.pc.value, 8);
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

        assert_eq!(cpu.pc.value, 8);
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
        cpu.memory.write_8bit(0, 0b00011001);

        cpu = cpu.set_a(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 2);
    }

    #[test]
    fn test_shift_left_b_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00111001);

        cpu = cpu.set_b(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 2);
    }

    #[test]
    fn test_shift_left_c_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01011001);

        cpu = cpu.set_c(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 2);
    }

    #[test]
    fn test_shift_left_d_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01111001);

        cpu = cpu.set_d(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 2);
    }

    #[test]
    fn test_shift_left_e_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10011001);

        cpu = cpu.set_e(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 2);
    }

    #[test]
    fn test_shift_left_hl_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11011001);

        cpu = cpu.set_hl(Register { value: 1 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 2);
    }

    #[test]
    fn test_shift_right_a() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00011111);

        cpu = cpu.set_a(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1);
    }

    #[test]
    fn test_shift_right_b() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00111111);

        cpu = cpu.set_b(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 1);
    }

    #[test]
    fn test_shift_right_c() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01011111);

        cpu = cpu.set_c(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 1);
    }

    #[test]
    fn test_shift_right_d() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01111111);

        cpu = cpu.set_d(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 1);
    }

    #[test]
    fn test_shift_right_e() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10011111);

        cpu = cpu.set_e(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 1);
    }

    #[test]
    fn test_shift_right_hl() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11011111);

        cpu = cpu.set_hl(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 1);
    }

    #[test]
    fn test_shift_right_a_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00011011);

        cpu = cpu.set_a(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 1);
    }

    #[test]
    fn test_shift_right_b_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b00111011);

        cpu = cpu.set_b(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.b.value, 1);
    }

    #[test]
    fn test_shift_right_c_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01011011);

        cpu = cpu.set_c(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.c.value, 1);
    }

    #[test]
    fn test_shift_right_d_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b01111011);

        cpu = cpu.set_d(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.d.value, 1);
    }

    #[test]
    fn test_shift_right_e_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b10011011);

        cpu = cpu.set_e(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.e.value, 1);
    }

    #[test]
    fn test_shift_right_hl_wrap() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.write_8bit(0, 0b11011011);

        cpu = cpu.set_hl(Register { value: 2 });

        cpu = executor::execute(cpu);

        assert_eq!(cpu.pc.value, 1);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.hl.value, 1);
    }
}