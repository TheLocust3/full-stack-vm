use cpu::cpu::CPU;
use cpu::register::Register;
use instructions::instruction_return::MemoryReturn;

pub fn read8(cpu: CPU) -> MemoryReturn {
    MemoryReturn {
        value: cpu.memory.read_8bit(cpu.hl.value) as u64,
        address: 0,
        overflow: false,
        negative: false
    }
}

pub fn read16(cpu: CPU) -> MemoryReturn {
    MemoryReturn {
        value: cpu.memory.read_16bit(cpu.hl.value) as u64,
        address: 0,
        overflow: false,
        negative: false
    }
}

pub fn read32(cpu: CPU) -> MemoryReturn {
    MemoryReturn {
        value: cpu.memory.read_32bit(cpu.hl.value) as u64,
        address: 0,
        overflow: false,
        negative: false
    }
}

pub fn read64(cpu: CPU) -> MemoryReturn {
    MemoryReturn {
        value: cpu.memory.read_64bit(cpu.hl.value),
        address: 0,
        overflow: false,
        negative: false
    }
}

pub fn write8(cpu: CPU, register: Register) -> MemoryReturn {
    MemoryReturn {
        value: register.value,
        address: cpu.hl.value,
        overflow: false,
        negative: false
    }
}

pub fn write16(cpu: CPU, register: Register) -> MemoryReturn {
    MemoryReturn {
        value: register.value,
        address: cpu.hl.value,
        overflow: false,
        negative: false
    }
}

pub fn write32(cpu: CPU, register: Register) -> MemoryReturn {
    MemoryReturn {
        value: register.value,
        address: cpu.hl.value,
        overflow: false,
        negative: false
    }
}

pub fn write64(cpu: CPU, register: Register) -> MemoryReturn {
    MemoryReturn {
        value: register.value,
        address: cpu.hl.value,
        overflow: false,
        negative: false
    }
}
