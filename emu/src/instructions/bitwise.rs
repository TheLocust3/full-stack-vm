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

pub fn shift_left(out: Register, wrap: bool) -> RegisterReturn {
    let value: u64;
    let unwrapped_value = ((u64::max_value() - (1 << 63)) & out.value) << 1;

    let overflow: bool;
    let overflow_bit = ((1 << 63) & out.value) >> 63;

    if wrap {
        value = unwrapped_value + overflow_bit;
        overflow = false;
    } else {
        value = unwrapped_value;
        overflow = if overflow_bit == 1 { true } else { false };
    }

    RegisterReturn {
        out: out.set_value(value),
        overflow: overflow,
        negative: false
    }
}

pub fn shift_right(out: Register, wrap: bool) -> RegisterReturn {
    let value: u64;
    let unwrapped_value = ((u64::max_value() - 1) & out.value) >> 1;

    let overflow: bool;
    let overflow_bit = 1 & out.value;

    if wrap {
        value = unwrapped_value + (overflow_bit << 63);
        overflow = false;
    } else {
        value = unwrapped_value;
        overflow = if overflow_bit == 1 { true } else { false };
    }

    RegisterReturn {
        out: out.set_value(value),
        overflow: overflow,
        negative: false
    }
}
