use cpu::register::Register;

pub fn set(out: Register, value: u64) -> Register {
    out.set_value(out.value + value)
}
