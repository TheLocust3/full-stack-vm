use cpu::register::Register;

pub fn add(out: Register, r1: Register) -> Register {
    out.set_value(out.value + r1.value)
}
