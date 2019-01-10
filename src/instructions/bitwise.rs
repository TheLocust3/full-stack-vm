use cpu::register::Register;
use instructions::instruction_return::RegisterReturn;

pub fn and(out: Register, r1: Register) -> RegisterReturn {
    RegisterReturn {
        out: out.set_value(out.value & r1.value),
        overflow: false,
        negative: false
    }
}

pub fn or(out: Register, r1: Register) -> RegisterReturn {
    RegisterReturn {
        out: out.set_value(out.value | r1.value),
        overflow: false,
        negative: false
    }
}

pub fn not(out: Register) -> RegisterReturn {
    RegisterReturn {
        out: out.set_value(!out.value),
        overflow: false,
        negative: false
    }
}