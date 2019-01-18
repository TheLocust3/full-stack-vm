use log::{info, error};

use cpu::cpu::CPU;
use cpu::register::Register;
use instructions::register;
use instructions::arithmetic;
use instructions::bitwise;
use instructions::miscellaneous;
use instructions::instruction_return::RegisterReturn;

pub fn execute(cpu: CPU) -> CPU {
    let pc = Register { value: cpu.pc.value + 1 };
    info!("PC: {}", pc.value);

    let instruction: u8 = cpu.memory.read_8bit(pc.value);
    info!("Instruction: {}", instruction);

    let mut out_cpu = cpu;

    match instruction {
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
        0b00111000 => { // not A A
            out_cpu = execute_not(out_cpu, 0b000);
        },
        0b00111001 => { // not A B
            out_cpu = execute_not(out_cpu, 0b001);
        },
        0b00111010 => { // not A C
            out_cpu = execute_not(out_cpu, 0b010);
        },
        0b00111011 => { // not A D
            out_cpu = execute_not(out_cpu, 0b011);
        },
        0b00111100 => { // not A E
            out_cpu = execute_not(out_cpu, 0b100);
        },
        0b00111110 => { // not A HL
            out_cpu = execute_not(out_cpu, 0b110);
        },
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
