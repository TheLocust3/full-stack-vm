use log::{info, error};

use cpu::cpu::CPU;
use cpu::register::Register;
use instructions::register;
use instructions::arithmetic;
use instructions::bitwise;
use instructions::control;
use instructions::memory;
use instructions::miscellaneous;
use instructions::instruction_return::RegisterReturn;
use instructions::instruction_return::MemoryReturn;

pub fn execute(cpu: CPU) -> CPU {
    let pc = Register { value: cpu.pc.value + 1 };
    info!("PC: {}", pc.value);

    let instruction: u8 = cpu.memory.read_8bit(pc.value);
    info!("Instruction: {}", instruction);

    let mut out_cpu = cpu;

    match instruction {
        // arithmatic
        0b10000000 => { // set A
            out_cpu = execute_set(out_cpu, pc, 0b000);
        },
        0b10000001 => { // set B
            out_cpu = execute_set(out_cpu, pc, 0b001);
        },
        0b10000010 => { // set C
            out_cpu = execute_set(out_cpu, pc, 0b010);
        },
        0b10000011 => { // set D
            out_cpu = execute_set(out_cpu, pc, 0b011);
        },
        0b10000100 => { // set E
            out_cpu = execute_set(out_cpu, pc, 0b100);
        },
        0b10000110 => { // set HL
            out_cpu = execute_set(out_cpu, pc, 0b110);
        },
        0b00010000 => { // add A A
            out_cpu = execute_add(out_cpu, 0b000);
        },
        0b00010001 => { // add A B
            out_cpu = execute_add(out_cpu, 0b001);
        },
        0b00010010 => { // add A C
            out_cpu = execute_add(out_cpu, 0b010);
        },
        0b00010011 => { // add A D
            out_cpu = execute_add(out_cpu, 0b011);
        },
        0b00010100 => { // add A E
            out_cpu = execute_add(out_cpu, 0b100);
        },
        0b00010101 => { // add A F
            out_cpu = execute_add(out_cpu, 0b101);
        },
        0b00010110 => { // add A HL
            out_cpu = execute_add(out_cpu, 0b110);
        },
        0b00100000 => { // sub A A
            out_cpu = execute_sub(out_cpu, 0b000);
        },
        0b00100001 => { // sub A B
            out_cpu = execute_sub(out_cpu, 0b001);
        },
        0b00100010 => { // sub A C
            out_cpu = execute_sub(out_cpu, 0b010);
        },
        0b00100011 => { // sub A D
            out_cpu = execute_sub(out_cpu, 0b011);
        },
        0b00100100 => { // sub A E
            out_cpu = execute_sub(out_cpu, 0b100);
        },
        0b00100101 => { // sub A E
            out_cpu = execute_sub(out_cpu, 0b101);
        },
        
        // bitwise
        0b00100110 => { // sub A HL
            out_cpu = execute_sub(out_cpu, 0b110);
        },
        0b00001000 => { // and A A
            out_cpu = execute_and(out_cpu, 0b000);
        },
        0b00001001 => { // and A B
            out_cpu = execute_and(out_cpu, 0b001);
        },
        0b00001010 => { // and A C
            out_cpu = execute_and(out_cpu, 0b010);
        },
        0b00001011 => { // and A D
            out_cpu = execute_and(out_cpu, 0b011);
        },
        0b00001100 => { // and A E
            out_cpu = execute_and(out_cpu, 0b100);
        },
        0b00001101 => { // and A F
            out_cpu = execute_and(out_cpu, 0b101);
        },
        0b00001110 => { // and A HL
            out_cpu = execute_and(out_cpu, 0b110);
        },
        0b00011000 => { // or A A
            out_cpu = execute_or(out_cpu, 0b000);
        },
        0b00011001 => { // or A B
            out_cpu = execute_or(out_cpu, 0b001);
        },
        0b00011010 => { // or A C
            out_cpu = execute_or(out_cpu, 0b010);
        },
        0b00011011 => { // or A D
            out_cpu = execute_or(out_cpu, 0b011);
        },
        0b00011100 => { // or A E
            out_cpu = execute_or(out_cpu, 0b100);
        },
        0b00011101 => { // or A F
            out_cpu = execute_or(out_cpu, 0b101);
        },
        0b00011110 => { // or A HL
            out_cpu = execute_or(out_cpu, 0b110);
        },
        0b00111000 => { // not A
            out_cpu = execute_not(out_cpu, 0b000);
        },
        0b00111001 => { // not B
            out_cpu = execute_not(out_cpu, 0b001);
        },
        0b00111010 => { // not C
            out_cpu = execute_not(out_cpu, 0b010);
        },
        0b00111011 => { // not D
            out_cpu = execute_not(out_cpu, 0b011);
        },
        0b00111100 => { // not E
            out_cpu = execute_not(out_cpu, 0b100);
        },
        0b00111110 => { // not HL
            out_cpu = execute_not(out_cpu, 0b110);
        },
        0b00110000 => { // shift-left A, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b000, false);
        },
        0b00110001 => { // shift-left B, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b001, false);
        },
        0b00110010 => { // shift-left C, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b010, false);
        },
        0b00110011 => { // shift-left D, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b011, false);
        },
        0b00110100 => { // shift-left E, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b100, false);
        },
        0b00110110 => { // shift-left HL, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b110, false);
        },
        0b10110000 => { // shift-left A, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b000, true);
        },
        0b10110001 => { // shift-left B, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b001, true);
        },
        0b10110010 => { // shift-left C, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b010, true);
        },
        0b10110011 => { // shift-left D, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b011, true);
        },
        0b10110100 => { // shift-left E, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b100, true);
        },
        0b10110110 => { // shift-left HL, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b110, true);
        },
        0b00101000 => { // shift-right A, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b000, false);
        },
        0b00101001 => { // shift-right B, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b001, false);
        },
        0b00101010 => { // shift-right C, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b010, false);
        },
        0b00101011 => { // shift-right D, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b011, false);
        },
        0b00101100 => { // shift-right E, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b100, false);
        },
        0b00101110 => { // shift-right HL, wrap=false
            out_cpu = execute_shift_left(out_cpu, 0b110, false);
        },
        0b10101000 => { // shift-right A, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b000, true);
        },
        0b10101001 => { // shift-right B, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b001, true);
        },
        0b10101010 => { // shift-right C, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b010, true);
        },
        0b10101011 => { // shift-right D, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b011, true);
        },
        0b10101100 => { // shift-right E, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b100, true);
        },
        0b10101110 => { // shift-right HL, wrap=true
            out_cpu = execute_shift_left(out_cpu, 0b110, true);
        },

        // control
        0b11111111 => { // jump
            out_cpu = execute_jump(out_cpu, pc);
        },
        0b11111110 => { // jump0
            out_cpu = execute_jump0(out_cpu, pc);
        },

        // memory
        0b10001001 => { // read8 A
            out_cpu = execute_read8(out_cpu, 0b000);
        },
        0b10011001 => { // read8 B
            out_cpu = execute_read8(out_cpu, 0b001);
        },
        0b10101001 => { // read8 C
            out_cpu = execute_read8(out_cpu, 0b010);
        },
        0b10111001 => { // read8 D
            out_cpu = execute_read8(out_cpu, 0b011);
        },
        0b11001001 => { // read8 E
            out_cpu = execute_read8(out_cpu, 0b100);
        },
        0b10001010 => { // read16 A
            out_cpu = execute_read16(out_cpu, 0b000);
        },
        0b10011010 => { // read16 B
            out_cpu = execute_read16(out_cpu, 0b001);
        },
        0b10101010 => { // read16 C
            out_cpu = execute_read16(out_cpu, 0b010);
        },
        0b10111010 => { // read16 D
            out_cpu = execute_read16(out_cpu, 0b011);
        },
        0b11001010 => { // read16 E
            out_cpu = execute_read16(out_cpu, 0b100);
        },
        0b10001011 => { // read32 A
            out_cpu = execute_read32(out_cpu, 0b000);
        },
        0b10011011 => { // read32 B
            out_cpu = execute_read32(out_cpu, 0b001);
        },
        0b10101011 => { // read32 C
            out_cpu = execute_read32(out_cpu, 0b010);
        },
        0b10111011 => { // read32 D
            out_cpu = execute_read32(out_cpu, 0b011);
        },
        0b11001011 => { // read32 E
            out_cpu = execute_read32(out_cpu, 0b100);
        }
        0b10001100 => { // read64 A
            out_cpu = execute_read64(out_cpu, 0b000);
        },
        0b10011100 => { // read64 B
            out_cpu = execute_read64(out_cpu, 0b001);
        },
        0b10101100 => { // read64 C
            out_cpu = execute_read64(out_cpu, 0b010);
        },
        0b10111100 => { // read64 D
            out_cpu = execute_read64(out_cpu, 0b011);
        },
        0b11001100 => { // read64 E
            out_cpu = execute_read64(out_cpu, 0b100);
        },
        
        0b10000001 => { // write8 A
            out_cpu = execute_write8(out_cpu, 0b000);
        },
        0b10010001 => { // write8 B
            out_cpu = execute_write8(out_cpu, 0b001);
        },
        0b10100001 => { // write8 C
            out_cpu = execute_write8(out_cpu, 0b010);
        },
        0b10110001 => { // write8 D
            out_cpu = execute_write8(out_cpu, 0b011);
        },
        0b11000001 => { // write8 E
            out_cpu = execute_write8(out_cpu, 0b100);
        },
        0b11010001 => { // write8 F
            out_cpu = execute_write8(out_cpu, 0b101);
        },
        0b10000010 => { // write16 A
            out_cpu = execute_write16(out_cpu, 0b000);
        },
        0b10010010 => { // write16 B
            out_cpu = execute_write16(out_cpu, 0b001);
        },
        0b10100010 => { // write16 C
            out_cpu = execute_write16(out_cpu, 0b010);
        },
        0b10110010 => { // write16 D
            out_cpu = execute_write16(out_cpu, 0b011);
        },
        0b11000010 => { // write16 E
            out_cpu = execute_write16(out_cpu, 0b100);
        },
        0b11010010 => { // write16 F
            out_cpu = execute_write16(out_cpu, 0b101);
        },
        0b10000011 => { // write32 A
            out_cpu = execute_write32(out_cpu, 0b000);
        },
        0b10010011 => { // write32 B
            out_cpu = execute_write32(out_cpu, 0b001);
        },
        0b10100011 => { // write32 C
            out_cpu = execute_write32(out_cpu, 0b010);
        },
        0b10110011 => { // write32 D
            out_cpu = execute_write32(out_cpu, 0b011);
        },
        0b11000011 => { // write32 E
            out_cpu = execute_write32(out_cpu, 0b100);
        },
        0b11010011 => { // write32 F
            out_cpu = execute_write32(out_cpu, 0b101);
        },
        0b10000100 => { // write64 A
            out_cpu = execute_write64(out_cpu, 0b000);
        },
        0b10010100 => { // write64 B
            out_cpu = execute_write64(out_cpu, 0b001);
        },
        0b10100100 => { // write64 C
            out_cpu = execute_write64(out_cpu, 0b010);
        },
        0b10110100 => { // write64 D
            out_cpu = execute_write64(out_cpu, 0b011);
        },
        0b11000100 => { // write64 E
            out_cpu = execute_write64(out_cpu, 0b100);
        },
        0b11010100 => { // write64 F
            out_cpu = execute_write64(out_cpu, 0b101);
        },

        // miscellaneous
        0b00000000 => {
            out_cpu = miscellaneous::nop(out_cpu);
        },
        _ => {
            error!("Instruction: {} not handled", instruction);
        }
    }

    out_cpu
}

pub fn execute_set(cpu: CPU, pc: Register, code: u8) -> CPU {
    let mut out_cpu = cpu;
    let register: Register = out_cpu.get_from_code(code);

    let value = out_cpu.memory.read_64bit(pc.value + 1);

    let register_return: RegisterReturn = register::set(register, value);

    out_cpu = out_cpu.set_from_code(code, register_return.out);
    out_cpu.set_pc(Register { value: pc.value + 1 })
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
