use cpu::register::Register;
use instructions::instruction_return::RegisterReturn;

// TODO: Tests

pub fn jump(pc: Register, address: u64) -> RegisterReturn {
    RegisterReturn {
        out: pc.set_value(address),
        overflow: false,
        negative: false
    }
}

pub fn jump0(pc: Register, a: Register, address: u64) -> RegisterReturn {
    if (a.value == 0) {
        RegisterReturn {
            out: pc.set_value(address),
            overflow: false,
            negative: false
        }
    } else {
        RegisterReturn {
            out: pc,
            overflow: false,
            negative: false
        }
    }
}
