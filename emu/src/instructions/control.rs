use cpu::register::Register;
use instructions::instruction_return::RegisterReturn;

pub fn jump(reg: Register) -> RegisterReturn {
    RegisterReturn {
        out: reg,
        overflow: false,
        negative: false
    }
}

pub fn jump0(pc: Register, a: Register, reg: Register) -> RegisterReturn {
    if a.value == 0 {
        RegisterReturn {
            out: reg,
            overflow: false,
            negative: false
        }
    } else {
        RegisterReturn {
            out: Register { value: pc.value + 1 }, // increment pc register because address was in pc + 1
            overflow: false,
            negative: false
        }
    }
}
