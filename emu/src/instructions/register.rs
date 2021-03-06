use cpu::register::Register;
use instructions::instruction_return::RegisterReturn;

pub fn set(out: Register, value: u64) -> RegisterReturn {
    RegisterReturn {
        out: out.set_value(value),
        overflow: false,
        negative: false
    }
}

pub fn move_reg(out: Register, input: Register) -> RegisterReturn {
    RegisterReturn {
        out: out.set_value(input.value),
        overflow: false,
        negative: false
    }
}
