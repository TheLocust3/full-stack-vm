use log::{info, error};

use cpu::cpu::CPU;
use cpu::register::Register;
use cpu::executor_functions;
use instructions::miscellaneous;

pub fn execute(cpu: CPU) -> CPU {
    info!("PC: {}", cpu.pc.value);

    let instruction: u8 = cpu.memory.read_8bit(cpu.pc.value);
    info!("Instruction: {}", instruction);

    let pc = cpu.pc;
    let mut has_set_pc = false;
    let mut out_cpu = cpu;

    match instruction {
        // register
        0b00001000 => { // set A
            out_cpu = executor_functions::execute_set(out_cpu, pc, 0b000);
            has_set_pc = true;
        },
        0b00001001 => { // set B
            out_cpu = executor_functions::execute_set(out_cpu, pc, 0b001);
            has_set_pc = true;
        },
        0b00001010 => { // set C
            out_cpu = executor_functions::execute_set(out_cpu, pc, 0b010);
            has_set_pc = true;
        },
        0b00001011 => { // set D
            out_cpu = executor_functions::execute_set(out_cpu, pc, 0b011);
            has_set_pc = true;
        },
        0b00001100 => { // set E
            out_cpu = executor_functions::execute_set(out_cpu, pc, 0b100);
            has_set_pc = true;
        },
        0b00001110 => { // set HL
            out_cpu = executor_functions::execute_set(out_cpu, pc, 0b110);
            has_set_pc = true;
        },

        0b00000001 => { // move A B
            out_cpu = executor_functions::execute_move(out_cpu, 0b000, 0b001);
        },
        0b00000010 => { // move A C
            out_cpu = executor_functions::execute_move(out_cpu, 0b000, 0b010);
        },
        0b00000011 => { // move A D
            out_cpu = executor_functions::execute_move(out_cpu, 0b000, 0b011);
        },
        0b00000100 => { // move A E
            out_cpu = executor_functions::execute_move(out_cpu, 0b000, 0b100);
        },
        0b00000101 => { // move A F
            out_cpu = executor_functions::execute_move(out_cpu, 0b000, 0b101);
        },
        0b00000110 => { // move A HL
            out_cpu = executor_functions::execute_move(out_cpu, 0b000, 0b110);
        },
        0b00010000 => { // move B A
            out_cpu = executor_functions::execute_move(out_cpu, 0b001, 0b000);
        },
        0b00010010 => { // move B C
            out_cpu = executor_functions::execute_move(out_cpu, 0b001, 0b010);
        },
        0b00010011 => { // move B D
            out_cpu = executor_functions::execute_move(out_cpu, 0b001, 0b011);
        },
        0b00010100 => { // move B E
            out_cpu = executor_functions::execute_move(out_cpu, 0b001, 0b100);
        },
        0b00010101 => { // move B F
            out_cpu = executor_functions::execute_move(out_cpu, 0b001, 0b101);
        },
        0b00010110 => { // move B HL
            out_cpu = executor_functions::execute_move(out_cpu, 0b001, 0b110);
        },
        0b00100000 => { // move C A
            out_cpu = executor_functions::execute_move(out_cpu, 0b010, 0b000);
        },
        0b00100001 => { // move C B
            out_cpu = executor_functions::execute_move(out_cpu, 0b010, 0b001);
        },
        0b00100011 => { // move C D
            out_cpu = executor_functions::execute_move(out_cpu, 0b010, 0b011);
        },
        0b00100100 => { // move C E
            out_cpu = executor_functions::execute_move(out_cpu, 0b010, 0b100);
        },
        0b00100101 => { // move C F
            out_cpu = executor_functions::execute_move(out_cpu, 0b010, 0b101);
        },
        0b00100110 => { // move C HL
            out_cpu = executor_functions::execute_move(out_cpu, 0b010, 0b110);
        },
        0b00110000 => { // move D A
            out_cpu = executor_functions::execute_move(out_cpu, 0b011, 0b000);
        },
        0b00110001 => { // move D B
            out_cpu = executor_functions::execute_move(out_cpu, 0b011, 0b001);
        },
        0b00110010 => { // move D C
            out_cpu = executor_functions::execute_move(out_cpu, 0b011, 0b010);
        },
        0b00110100 => { // move D E
            out_cpu = executor_functions::execute_move(out_cpu, 0b011, 0b100);
        },
        0b00110101 => { // move D F
            out_cpu = executor_functions::execute_move(out_cpu, 0b011, 0b101);
        },
        0b00110110 => { // move D HL
            out_cpu = executor_functions::execute_move(out_cpu, 0b011, 0b110);
        },
        0b01000000 => { // move E A
            out_cpu = executor_functions::execute_move(out_cpu, 0b100, 0b000);
        },
        0b01000001 => { // move E B
            out_cpu = executor_functions::execute_move(out_cpu, 0b100, 0b001);
        },
        0b01000010 => { // move E C
            out_cpu = executor_functions::execute_move(out_cpu, 0b100, 0b010);
        },
        0b01000011 => { // move E D
            out_cpu = executor_functions::execute_move(out_cpu, 0b100, 0b011);
        },
        0b01000101 => { // move E F
            out_cpu = executor_functions::execute_move(out_cpu, 0b100, 0b101);
        },
        0b01000110 => { // move E HL
            out_cpu = executor_functions::execute_move(out_cpu, 0b100, 0b110);
        },
        0b01100000 => { // move HL A
            out_cpu = executor_functions::execute_move(out_cpu, 0b110, 0b000);
        },
        0b01100001 => { // move HL B
            out_cpu = executor_functions::execute_move(out_cpu, 0b110, 0b001);
        },
        0b01100010 => { // move HL C
            out_cpu = executor_functions::execute_move(out_cpu, 0b110, 0b010);
        },
        0b01100011 => { // move HL D
            out_cpu = executor_functions::execute_move(out_cpu, 0b110, 0b011);
        },
        0b01100100 => { // move HL E
            out_cpu = executor_functions::execute_move(out_cpu, 0b110, 0b100);
        },
        0b01100101 => { // move HL F
            out_cpu = executor_functions::execute_move(out_cpu, 0b110, 0b101);
        },

        // arithmatic
        0b00001111 => { // add A A
            out_cpu = executor_functions::execute_add(out_cpu, 0b000);
        },
        0b00101111 => { // add A B
            out_cpu = executor_functions::execute_add(out_cpu, 0b001);
        },
        0b01001111 => { // add A C
            out_cpu = executor_functions::execute_add(out_cpu, 0b010);
        },
        0b01101111 => { // add A D
            out_cpu = executor_functions::execute_add(out_cpu, 0b011);
        },
        0b10001111 => { // add A E
            out_cpu = executor_functions::execute_add(out_cpu, 0b100);
        },
        0b10101111 => { // add A F
            out_cpu = executor_functions::execute_add(out_cpu, 0b101);
        },
        0b11001111 => { // add A HL
            out_cpu = executor_functions::execute_add(out_cpu, 0b110);
        },
        0b00011111 => { // sub A A
            out_cpu = executor_functions::execute_sub(out_cpu, 0b000);
        },
        0b00111111 => { // sub A B
            out_cpu = executor_functions::execute_sub(out_cpu, 0b001);
        },
        0b01011111 => { // sub A C
            out_cpu = executor_functions::execute_sub(out_cpu, 0b010);
        },
        0b01111111 => { // sub A D
            out_cpu = executor_functions::execute_sub(out_cpu, 0b011);
        },
        0b10011111 => { // sub A E
            out_cpu = executor_functions::execute_sub(out_cpu, 0b100);
        },
        0b10111111 => { // sub A F
            out_cpu = executor_functions::execute_sub(out_cpu, 0b101);
        },
        0b11011111 => { // sub A HL
            out_cpu = executor_functions::execute_sub(out_cpu, 0b110);
        },
        
        // bitwise
        0b00011000 => { // and A A
            out_cpu = executor_functions::execute_and(out_cpu, 0b000);
        },
        0b00111000 => { // and A B
            out_cpu = executor_functions::execute_and(out_cpu, 0b001);
        },
        0b01011000 => { // and A C
            out_cpu = executor_functions::execute_and(out_cpu, 0b010);
        },
        0b01111000 => { // and A D
            out_cpu = executor_functions::execute_and(out_cpu, 0b011);
        },
        0b10011000 => { // and A E
            out_cpu = executor_functions::execute_and(out_cpu, 0b100);
        },
        0b10111000 => { // and A F
            out_cpu = executor_functions::execute_and(out_cpu, 0b101);
        },
        0b11011000 => { // and A HL
            out_cpu = executor_functions::execute_and(out_cpu, 0b110);
        },
        0b00011001 => { // or A A
            out_cpu = executor_functions::execute_or(out_cpu, 0b000);
        },
        0b00111001 => { // or A B
            out_cpu = executor_functions::execute_or(out_cpu, 0b001);
        },
        0b01011001 => { // or A C
            out_cpu = executor_functions::execute_or(out_cpu, 0b010);
        },
        0b01111001 => { // or A D
            out_cpu = executor_functions::execute_or(out_cpu, 0b011);
        },
        0b10011001 => { // or A E
            out_cpu = executor_functions::execute_or(out_cpu, 0b100);
        },
        0b10111001 => { // or A F
            out_cpu = executor_functions::execute_or(out_cpu, 0b101);
        },
        0b11011001 => { // or A HL
            out_cpu = executor_functions::execute_or(out_cpu, 0b110);
        },
        0b00011011 => { // not A
            out_cpu = executor_functions::execute_not(out_cpu, 0b000);
        },
        0b00111011 => { // not B
            out_cpu = executor_functions::execute_not(out_cpu, 0b001);
        },
        0b01011011 => { // not C
            out_cpu = executor_functions::execute_not(out_cpu, 0b010);
        },
        0b01111011 => { // not D
            out_cpu = executor_functions::execute_not(out_cpu, 0b011);
        },
        0b10011011 => { // not E
            out_cpu = executor_functions::execute_not(out_cpu, 0b100);
        },
        0b11011011 => { // not HL
            out_cpu = executor_functions::execute_not(out_cpu, 0b110);
        },
        0b00011101 => { // shift-left A, wrap=false
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b000, false);
        },
        0b00111101 => { // shift-left B, wrap=false
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b001, false);
        },
        0b01011101 => { // shift-left C, wrap=false
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b010, false);
        },
        0b01111101 => { // shift-left D, wrap=false
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b011, false);
        },
        0b10011101 => { // shift-left E, wrap=false
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b100, false);
        },
        0b11011101 => { // shift-left HL, wrap=false
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b110, false);
        },
        0b00000111 => { // shift-left A, wrap=true
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b000, true);
        },
        0b00100111 => { // shift-left B, wrap=true
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b001, true);
        },
        0b01000111 => { // shift-left C, wrap=true
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b010, true);
        },
        0b01100111 => { // shift-left D, wrap=true
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b011, true);
        },
        0b10000111 => { // shift-left E, wrap=true
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b100, true);
        },
        0b11000111 => { // shift-left HL, wrap=true
            out_cpu = executor_functions::execute_shift_left(out_cpu, 0b110, true);
        },
        0b00011110 => { // shift-right A, wrap=false
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b000, false);
        },
        0b00111110 => { // shift-right B, wrap=false
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b001, false);
        },
        0b01011110 => { // shift-right C, wrap=false
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b010, false);
        },
        0b01111110 => { // shift-right D, wrap=false
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b011, false);
        },
        0b10011110 => { // shift-right E, wrap=false
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b100, false);
        },
        0b11011110 => { // shift-right HL, wrap=false
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b110, false);
        },
        0b00010111 => { // shift-right A, wrap=true
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b000, true);
        },
        0b00110111 => { // shift-right B, wrap=true
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b001, true);
        },
        0b01010111 => { // shift-right C, wrap=true
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b010, true);
        },
        0b01110111 => { // shift-right D, wrap=true
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b011, true);
        },
        0b10010111 => { // shift-right E, wrap=true
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b100, true);
        },
        0b11010111 => { // shift-right HL, wrap=true
            out_cpu = executor_functions::execute_shift_right(out_cpu, 0b110, true);
        },

        // control
        0b11111111 => { // jump
            out_cpu = executor_functions::execute_jump(out_cpu, pc);
            has_set_pc = true;
        },
        0b11111110 => { // jump0
            out_cpu = executor_functions::execute_jump0(out_cpu, pc);
            has_set_pc = true;
        },

        // memory
        0b11111000 => { // read8 A
            out_cpu = executor_functions::execute_read8(out_cpu, 0b000);
        },
        0b11111001 => { // read8 B
            out_cpu = executor_functions::execute_read8(out_cpu, 0b001);
        },
        0b11111010 => { // read8 C
            out_cpu = executor_functions::execute_read8(out_cpu, 0b010);
        },
        0b11111011 => { // read8 D
            out_cpu = executor_functions::execute_read8(out_cpu, 0b011);
        },
        0b11111100 => { // read8 E
            out_cpu = executor_functions::execute_read8(out_cpu, 0b100);
        },
        0b11110000 => { // read16 A
            out_cpu = executor_functions::execute_read16(out_cpu, 0b000);
        },
        0b11110001 => { // read16 B
            out_cpu = executor_functions::execute_read16(out_cpu, 0b001);
        },
        0b11110010 => { // read16 C
            out_cpu = executor_functions::execute_read16(out_cpu, 0b010);
        },
        0b11110011 => { // read16 D
            out_cpu = executor_functions::execute_read16(out_cpu, 0b011);
        },
        0b11110100 => { // read16 E
            out_cpu = executor_functions::execute_read16(out_cpu, 0b100);
        },
        0b11100000 => { // read32 A
            out_cpu = executor_functions::execute_read32(out_cpu, 0b000);
        },
        0b11100001 => { // read32 B
            out_cpu = executor_functions::execute_read32(out_cpu, 0b001);
        },
        0b11100010 => { // read32 C
            out_cpu = executor_functions::execute_read32(out_cpu, 0b010);
        },
        0b11100011 => { // read32 D
            out_cpu = executor_functions::execute_read32(out_cpu, 0b011);
        },
        0b11100100 => { // read32 E
            out_cpu = executor_functions::execute_read32(out_cpu, 0b100);
        }
        0b11000000 => { // read64 A
            out_cpu = executor_functions::execute_read64(out_cpu, 0b000);
        },
        0b11000001 => { // read64 B
            out_cpu = executor_functions::execute_read64(out_cpu, 0b001);
        },
        0b11000010 => { // read64 C
            out_cpu = executor_functions::execute_read64(out_cpu, 0b010);
        },
        0b11000011 => { // read64 D
            out_cpu = executor_functions::execute_read64(out_cpu, 0b011);
        },
        0b11000100 => { // read64 E
            out_cpu = executor_functions::execute_read64(out_cpu, 0b100);
        },
        
        0b10000000 => { // write8 A
            out_cpu = executor_functions::execute_write8(out_cpu, 0b000);
        },
        0b10000001 => { // write8 B
            out_cpu = executor_functions::execute_write8(out_cpu, 0b001);
        },
        0b10000010 => { // write8 C
            out_cpu = executor_functions::execute_write8(out_cpu, 0b010);
        },
        0b10000011 => { // write8 D
            out_cpu = executor_functions::execute_write8(out_cpu, 0b011);
        },
        0b10000100 => { // write8 E
            out_cpu = executor_functions::execute_write8(out_cpu, 0b100);
        },
        0b10000101 => { // write8 F
            out_cpu = executor_functions::execute_write8(out_cpu, 0b101);
        },
        0b10001000 => { // write16 A
            out_cpu = executor_functions::execute_write16(out_cpu, 0b000);
        },
        0b10001001 => { // write16 B
            out_cpu = executor_functions::execute_write16(out_cpu, 0b001);
        },
        0b10001010 => { // write16 C
            out_cpu = executor_functions::execute_write16(out_cpu, 0b010);
        },
        0b10001011 => { // write16 D
            out_cpu = executor_functions::execute_write16(out_cpu, 0b011);
        },
        0b10001100 => { // write16 E
            out_cpu = executor_functions::execute_write16(out_cpu, 0b100);
        },
        0b10001101 => { // write16 F
            out_cpu = executor_functions::execute_write16(out_cpu, 0b101);
        },
        0b10010000 => { // write32 A
            out_cpu = executor_functions::execute_write32(out_cpu, 0b000);
        },
        0b10010001 => { // write32 B
            out_cpu = executor_functions::execute_write32(out_cpu, 0b001);
        },
        0b10010010 => { // write32 C
            out_cpu = executor_functions::execute_write32(out_cpu, 0b010);
        },
        0b10010011 => { // write32 D
            out_cpu = executor_functions::execute_write32(out_cpu, 0b011);
        },
        0b10010100 => { // write32 E
            out_cpu = executor_functions::execute_write32(out_cpu, 0b100);
        },
        0b10010101 => { // write32 F
            out_cpu = executor_functions::execute_write32(out_cpu, 0b101);
        },
        0b10101000 => { // write64 A
            out_cpu = executor_functions::execute_write64(out_cpu, 0b000);
        },
        0b10101001 => { // write64 B
            out_cpu = executor_functions::execute_write64(out_cpu, 0b001);
        },
        0b10101010 => { // write64 C
            out_cpu = executor_functions::execute_write64(out_cpu, 0b010);
        },
        0b10101011 => { // write64 D
            out_cpu = executor_functions::execute_write64(out_cpu, 0b011);
        },
        0b10101100 => { // write64 E
            out_cpu = executor_functions::execute_write64(out_cpu, 0b100);
        },
        0b10101101 => { // write64 F
            out_cpu = executor_functions::execute_write64(out_cpu, 0b101);
        },

        // stack
        0b11001000 => { // push A
            out_cpu = executor_functions::execute_push(out_cpu, 0b000);
            has_set_pc = true;
        },
        0b11001001 => { // push B
            out_cpu = executor_functions::execute_push(out_cpu, 0b001);
            has_set_pc = true;
        },
        0b11001010 => { // push C
            out_cpu = executor_functions::execute_push(out_cpu, 0b010);
            has_set_pc = true;
        },
        0b11001011 => { // push D
            out_cpu = executor_functions::execute_push(out_cpu, 0b011);
            has_set_pc = true;
        },
        0b11001100 => { // push E
            out_cpu = executor_functions::execute_push(out_cpu, 0b101);
            has_set_pc = true;
        },
        0b11001101 => { // push F
            out_cpu = executor_functions::execute_push(out_cpu, 0b100);
            has_set_pc = true;
        },
        0b11001110 => { // push HL
            out_cpu = executor_functions::execute_push(out_cpu, 0b110);
            has_set_pc = true;
        },
        0b11101000 => { // pop A
            out_cpu = executor_functions::execute_pop(out_cpu, 0b000);
            has_set_pc = true;
        },
        0b11101001 => { // pop B
            out_cpu = executor_functions::execute_pop(out_cpu, 0b001);
            has_set_pc = true;
        },
        0b11101010 => { // pop C
            out_cpu = executor_functions::execute_pop(out_cpu, 0b010);
            has_set_pc = true;
        },
        0b11101011 => { // pop D
            out_cpu = executor_functions::execute_pop(out_cpu, 0b011);
            has_set_pc = true;
        },
        0b11101100 => { // pop E
            out_cpu = executor_functions::execute_pop(out_cpu, 0b101);
            has_set_pc = true;
        },
        0b11101100 => { // pop F
            out_cpu = executor_functions::execute_pop(out_cpu, 0b100);
            has_set_pc = true;
        },
        0b11101110 => { // pop HL
            out_cpu = executor_functions::execute_pop(out_cpu, 0b110);
            has_set_pc = true;
        },

        // miscellaneous
        0b00000000 => {
            out_cpu = miscellaneous::nop(out_cpu);
        },
        _ => {
            error!("Instruction: {} not handled", instruction);
        }
    }

    if !has_set_pc {
        out_cpu.set_pc(Register { value: pc.value + 1 })
    } else {
        out_cpu
    }
}
