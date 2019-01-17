use log::{info, error};

use cpu::cpu::CPU;
use cpu::register::Register;
use instructions::register;
use instructions::arithmetic;
use instructions::miscellaneous;
use instructions::instruction_return::RegisterReturn;

pub fn execute(cpu: CPU) -> CPU {
    let mut pc = Register { value: cpu.pc.value + 1 };
    info!("PC: {}", pc.value);

    let instruction: u64 = cpu.memory.read_64bit(pc.value);
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
    out_cpu = out_cpu.set_f_from_register_return(register_return);
    out_cpu.set_pc(Register { value: pc.value + 1 })
}
