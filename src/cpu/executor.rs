use log::{info, error};

use cpu::cpu::CPU;
use cpu::register::Register;
use instructions::arithmetic;
use instructions::miscellaneous;
use instructions::instruction_return::RegisterReturn;

pub fn execute(cpu: CPU) -> CPU {
    info!("PC: {}", cpu.pc.value);

    let instruction: u16 = cpu.memory.read_16bit(cpu.pc.value);

    info!("Instruction: {}", instruction);

    match instruction {
        0x0001 => {
            let a: Register = cpu.a;
            let b: Register = cpu.b;

            let register_return: RegisterReturn = arithmetic::add(a, b);

            let new_cpu = cpu.set_a(register_return.out);

            new_cpu.set_f_from_register_return(register_return)
        },
        0x0000 => miscellaneous::nop(cpu),
        _ => {
            error!("Instruction: {} not handled", instruction);
            miscellaneous::nop(cpu)
        }
    }
}
