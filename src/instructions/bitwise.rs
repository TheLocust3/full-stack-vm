use cpu::register::Register;

pub fn and(out: Register, r1: Register) -> Register {
    out.set_value(out.value & r1.value)
}

pub fn or(out: Register, r1: Register) -> Register {
    out.set_value(out.value | r1.value)
}

pub fn not(out: Register) -> Register {
    out.set_value(!out.value)
}