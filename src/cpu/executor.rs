use log::{info, error};

use cpu::cpu::CPU;
use cpu::register::Register;
use instructions::arithmetic;
use instructions::miscellaneous;

pub fn execute(cpu: CPU) -> CPU {
    info!("PC: {}", cpu.pc.value);

    let instruction: u16 = cpu.memory.read_16bit(cpu.pc.value);

    info!("Instruction: {}", instruction);

    match instruction {
        0x0001 => {
            let a: Register = cpu.a;
            let b: Register = cpu.b;

            cpu.set_a(arithmetic::add(a, b))
        },
        0x0000 => miscellaneous::nop(cpu),
        _ => {
            error!("Instruction: {} not handled", instruction);
            miscellaneous::nop(cpu)
        }
    }
}
