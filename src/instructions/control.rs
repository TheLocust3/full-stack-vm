use cpu::register::Register;
use instructions::instruction_return::RegisterReturn;

pub fn jump(pc: Register, address: u64) -> RegisterReturn {
    RegisterReturn {
        out: pc.set_value(address),
        overflow: false,
        negative: false
    }
}

pub fn jump0(pc: Register, a: Register, address: u64) -> RegisterReturn {
    if a.value == 0 {
        RegisterReturn {
            out: pc.set_value(address),
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
