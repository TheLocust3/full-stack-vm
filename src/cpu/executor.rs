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
            let a = out_cpu.a;
            let value = out_cpu.memory.read_64bit(pc.value + 1);

            let register_return: RegisterReturn = register::set(a, value);

            out_cpu = out_cpu.set_a(register_return.out);
            out_cpu = out_cpu.set_f_from_register_return(register_return);
            
            pc = Register { value: pc.value + 1 };
        },
        0b10000001 => { // set B
            let b = out_cpu.b;
            let value = out_cpu.memory.read_64bit(pc.value + 1);

            let register_return: RegisterReturn = register::set(b, value);

            out_cpu = out_cpu.set_b(register_return.out);
            out_cpu = out_cpu.set_f_from_register_return(register_return);
            
            pc = Register { value: pc.value + 1 };
        },
        0b10000010 => { // set C
            let c = out_cpu.c;
            let value = out_cpu.memory.read_64bit(pc.value + 1);

            let register_return: RegisterReturn = register::set(c, value);

            out_cpu = out_cpu.set_c(register_return.out);
            out_cpu = out_cpu.set_f_from_register_return(register_return);
            
            pc = Register { value: pc.value + 1 };
        },
        0b10000011 => { // set D
            let d = out_cpu.d;
            let value = out_cpu.memory.read_64bit(pc.value + 1);

            let register_return: RegisterReturn = register::set(d, value);

            out_cpu = out_cpu.set_d(register_return.out);
            out_cpu = out_cpu.set_f_from_register_return(register_return);
            
            pc = Register { value: pc.value + 1 };
        },
        0b10000100 => { // set E
            let e = out_cpu.e;
            let value = out_cpu.memory.read_64bit(pc.value + 1);

            let register_return: RegisterReturn = register::set(e, value);

            out_cpu = out_cpu.set_e(register_return.out);
            out_cpu = out_cpu.set_f_from_register_return(register_return);
            
            pc = Register { value: pc.value + 1 };
        },
        0b10000110 => { // set HL
            let hl = out_cpu.hl;
            let value = out_cpu.memory.read_64bit(pc.value + 1);

            let register_return: RegisterReturn = register::set(hl, value);

            out_cpu = out_cpu.set_hl(register_return.out);
            out_cpu = out_cpu.set_f_from_register_return(register_return);
            
            pc = Register { value: pc.value + 1 };
        },
        0b00000000 => {
            out_cpu = miscellaneous::nop(out_cpu);
        },
        _ => {
            error!("Instruction: {} not handled", instruction);
        }
    }

    out_cpu.set_pc(pc)
}
