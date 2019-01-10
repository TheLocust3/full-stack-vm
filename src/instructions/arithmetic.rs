use cpu::register::Register;
use instructions::instruction_return::RegisterReturn;

pub fn add(out: Register, r1: Register) -> RegisterReturn {
    RegisterReturn {
        out: out.set_value(out.value + r1.value),
        overflow: false,
        negative: false
    }
}

pub fn sub(out: Register, r1: Register) -> RegisterReturn {
    RegisterReturn {
        out: out.set_value(out.value - r1.value),
        overflow: false,
        negative: false
    }
}
