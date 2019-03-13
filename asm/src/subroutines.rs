use instruction::Instruction;

pub const TEMPORARY_REGISTER: u64 = 320;

pub fn beginning_subroutine() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    // free space begins at 320 but leave 8 bytes of memory for temporary register
    instructions.push(Instruction::new("JUMP", "328", ""));

    instructions
}

pub fn ending_subroutine() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    instructions.push(Instruction::new("HALT", "", "")); // stop cpu after program is done

    instructions
}
