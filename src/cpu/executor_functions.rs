use cpu::cpu::CPU;
use cpu::register::Register;
use instructions::register;
use instructions::arithmetic;
use instructions::bitwise;
use instructions::control;
use instructions::memory;
use instructions::instruction_return::RegisterReturn;
use instructions::instruction_return::MemoryReturn;

pub fn execute_set(cpu: CPU, pc: Register, code: u8) -> CPU {
    let mut out_cpu = cpu;
    let register: Register = out_cpu.get_from_code(code);

    let value = out_cpu.memory.read_64bit(pc.value + 1);

    let register_return: RegisterReturn = register::set(register, value);

    out_cpu = out_cpu.set_from_code(code, register_return.out);
    out_cpu.set_pc(Register { value: pc.value + 8 })
}

pub fn execute_move(cpu: CPU, out_code: u8, in_code: u8) -> CPU {
    let mut out_cpu = cpu;
    let out_register: Register = out_cpu.get_from_code(out_code);
    let in_register: Register = out_cpu.get_from_code(in_code);

    let register_return: RegisterReturn = register::move_reg(out_register, in_register);

    out_cpu.set_from_code(out_code, register_return.out)
}

pub fn execute_add(cpu: CPU, code: u8) -> CPU {
    let mut out_cpu = cpu;
    let register: Register = out_cpu.get_from_code(code);

    let register_return: RegisterReturn = arithmetic::add(out_cpu.a, register);

    out_cpu = out_cpu.set_a(register_return.out);
    out_cpu.set_f_from_register_return(register_return)
}

pub fn execute_sub(cpu: CPU, code: u8) -> CPU {
    let mut out_cpu = cpu;
    let register: Register = out_cpu.get_from_code(code);

    let register_return: RegisterReturn = arithmetic::sub(out_cpu.a, register);

    out_cpu = out_cpu.set_a(register_return.out);
    out_cpu.set_f_from_register_return(register_return)
}

pub fn execute_and(cpu: CPU, code: u8) -> CPU {
    let mut out_cpu = cpu;
    let register: Register = out_cpu.get_from_code(code);

    let register_return: RegisterReturn = bitwise::and(out_cpu.a, register);

    out_cpu.set_a(register_return.out)
}

pub fn execute_or(cpu: CPU, code: u8) -> CPU {
    let mut out_cpu = cpu;
    let register: Register = out_cpu.get_from_code(code);

    let register_return: RegisterReturn = bitwise::or(out_cpu.a, register);

    out_cpu.set_a(register_return.out)
}

pub fn execute_not(cpu: CPU, code: u8) -> CPU {
    let mut out_cpu = cpu;
    let register: Register = out_cpu.get_from_code(code);

    let register_return: RegisterReturn = bitwise::not(register);

    out_cpu.set_from_code(code, register_return.out)
}

pub fn execute_shift_left(cpu: CPU, code: u8, wrap: bool) -> CPU {
    let mut out_cpu = cpu;
    let register: Register = out_cpu.get_from_code(code);

    let register_return: RegisterReturn = bitwise::shift_left(register, wrap);

    out_cpu = out_cpu.set_from_code(code, register_return.out);
    out_cpu.set_f_from_register_return(register_return)
}

pub fn execute_shift_right(cpu: CPU, code: u8, wrap: bool) -> CPU {
    let mut out_cpu = cpu;
    let register: Register = out_cpu.get_from_code(code);

    let register_return: RegisterReturn = bitwise::shift_right(register, wrap);

    out_cpu = out_cpu.set_from_code(code, register_return.out);
    out_cpu.set_f_from_register_return(register_return)
}

pub fn execute_jump(cpu: CPU, pc: Register) -> CPU {
    let mut out_cpu = cpu;

    let value = out_cpu.memory.read_64bit(pc.value + 1);

    let register_return: RegisterReturn = control::jump(pc, value);

    out_cpu.set_pc(register_return.out)
}

pub fn execute_jump0(cpu: CPU, pc: Register) -> CPU {
    let mut out_cpu = cpu;

    let value = out_cpu.memory.read_64bit(pc.value + 1);

    let register_return: RegisterReturn = control::jump0(pc, out_cpu.a, value);

    out_cpu.set_pc(register_return.out)
}

pub fn execute_read8(cpu: CPU, code: u8) -> CPU {
    let memory_return: MemoryReturn = memory::read8(&cpu);

    cpu.set_from_code(code, Register { value: memory_return.value })
}

pub fn execute_read16(cpu: CPU, code: u8) -> CPU {
    let memory_return: MemoryReturn = memory::read16(&cpu);

    cpu.set_from_code(code, Register { value: memory_return.value })
}

pub fn execute_read32(cpu: CPU, code: u8) -> CPU {
    let memory_return: MemoryReturn = memory::read32(&cpu);

    cpu.set_from_code(code, Register { value: memory_return.value })
}

pub fn execute_read64(cpu: CPU, code: u8) -> CPU {
    let memory_return: MemoryReturn = memory::read64(&cpu);

    cpu.set_from_code(code, Register { value: memory_return.value })
}

pub fn execute_write8(cpu: CPU, code: u8) -> CPU {
    let mut out_cpu = cpu;
    let hl: Register = out_cpu.hl;
    let register: Register = out_cpu.get_from_code(code);

    let memory_return: MemoryReturn = memory::write8(hl, register);
    out_cpu.memory.write_8bit(memory_return.address, memory_return.value as u8);

    out_cpu
}

pub fn execute_write16(cpu: CPU, code: u8) -> CPU {
    let mut out_cpu = cpu;
    let hl: Register = out_cpu.hl;
    let register: Register = out_cpu.get_from_code(code);

    let memory_return: MemoryReturn = memory::write16(hl, register);
    out_cpu.memory.write_16bit(memory_return.address, memory_return.value as u16);

    out_cpu
}

pub fn execute_write32(cpu: CPU, code: u8) -> CPU {
    let mut out_cpu = cpu;
    let hl: Register = out_cpu.hl;
    let register: Register = out_cpu.get_from_code(code);

    let memory_return: MemoryReturn = memory::write32(hl, register);
    out_cpu.memory.write_32bit(memory_return.address, memory_return.value as u32);

    out_cpu
}

pub fn execute_write64(cpu: CPU, code: u8) -> CPU {
    let mut out_cpu = cpu;
    let hl: Register = out_cpu.hl;
    let register: Register = out_cpu.get_from_code(code);

    let memory_return: MemoryReturn = memory::write64(hl, register);
    out_cpu.memory.write_64bit(memory_return.address, memory_return.value as u64);

    out_cpu
}
