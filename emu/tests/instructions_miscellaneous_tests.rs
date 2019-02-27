#[cfg(test)]
mod tests {
    extern crate emu;
    use self::emu::cpu::cpu::CPU;
    use self::emu::cpu::register::Register;
    use self::emu::instructions::miscellaneous;
    use self::emu::memory::TOTAL_MEMORY;
    use self::emu::memory::STACK_END;

    #[test]
    fn test_halt() {
        let mut cpu: CPU = CPU::new();

        cpu = miscellaneous::halt(cpu);

        assert_eq!(cpu.stopped, true);
    }

    #[test]
    fn test_push() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_a(Register { value: 10 });
        cpu = miscellaneous::push(cpu.a, cpu);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY - 8);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_push_underflow() {
        let mut cpu: CPU = CPU::new();

        cpu = cpu.set_sp(Register { value: STACK_END });
        cpu = cpu.set_a(Register { value: 10 });
        cpu = miscellaneous::push(cpu.a, cpu);

        assert_eq!(cpu.sp.value, STACK_END);
        assert_eq!(cpu.f.value, 2);
        assert_eq!(cpu.a.value, 10);
        assert_eq!(cpu.memory.read_64bit(0), 0);
    }

    #[test]
    fn test_pop() {
        let mut cpu: CPU = CPU::new();

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = cpu.set_sp(Register { value: TOTAL_MEMORY - 8 });
        cpu = miscellaneous::pop(0b000, cpu);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 0);
        assert_eq!(cpu.a.value, 10);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }

    #[test]
    fn test_pop_overflow() {
        let mut cpu: CPU = CPU::new();

        cpu.memory.write_64bit(TOTAL_MEMORY - 8, 10);
        cpu = miscellaneous::pop(0b000, cpu);

        assert_eq!(cpu.sp.value, TOTAL_MEMORY);
        assert_eq!(cpu.f.value, 1);
        assert_eq!(cpu.a.value, 0);
        assert_eq!(cpu.memory.read_64bit(TOTAL_MEMORY - 8), 10);
    }
}