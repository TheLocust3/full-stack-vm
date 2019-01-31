use cpu::register::Register;
use instructions::instruction_return::RegisterReturn;

pub fn add(out: Register, r1: Register) -> RegisterReturn {
    let overflow = u64::max_value() - r1.value < out.value;
    let mut value = u64::max_value();

    if !overflow {
        value = out.value + r1.value;
    }

    RegisterReturn {
        out: out.set_value(value),
        overflow: overflow,
        negative: false
    }
}

pub fn sub(out: Register, r1: Register) -> RegisterReturn {
    let negative = u64::min_value() + r1.value > out.value;
    let mut value = u64::min_value();

    if !negative {
        value = out.value - r1.value;
    }

    RegisterReturn {
        out: out.set_value(value),
        overflow: false,
        negative: negative
    }
}
