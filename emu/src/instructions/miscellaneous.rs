use cpu::cpu::CPU;
use cpu::register::Register;
use instructions::instruction_return::MemoryReturn;
use memory::TOTAL_MEMORY;

pub fn nop(cpu: CPU) -> CPU {
    cpu
}

pub fn push(reg: Register, cpu: CPU) -> CPU {
    let mut out_cpu = cpu;
    let sp: Register = out_cpu.sp;

    if u64::min_value() + 8 > sp.value {
        return out_cpu.set_f(Register { value: 2 })
    } else {
        out_cpu = out_cpu.set_f(Register { value: 0 });
    }

    out_cpu.memory.write_64bit(sp.value - 8, reg.value);

    out_cpu.set_sp(Register { value: sp.value - 8 })
}

pub fn pop(code: u8, cpu: CPU) -> CPU {
    let mut out_cpu = cpu;
    let sp: Register = out_cpu.sp;

    if TOTAL_MEMORY - 8 < sp.value {
        return out_cpu.set_f(Register { value: 1 })
    } else {
        out_cpu = out_cpu.set_f(Register { value: 0 });
    }
    
    let value = out_cpu.memory.read_64bit(sp.value);

    out_cpu.set_from_code(code, Register { value: value }).set_sp(Register { value: sp.value + 8 })
}
