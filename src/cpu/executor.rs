use log::{info};

use cpu::cpu::CPU;

pub fn execute(cpu: &CPU) {
    info!("PC: {}", cpu.pc.value);

    let instruction: u16 = cpu.memory.read_16bit(cpu.pc.value);

    info!("Instruction: {}", instruction);
}
