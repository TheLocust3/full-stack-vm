use instruction::Instruction;

pub fn beginning_subroutine() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    // free space begins at 320 but leave 8 bytes of memory for temporary register
    instructions.push(Instruction::new("JUMP", "328", "")); // compiles to 9 bytes
    instructions.push(Instruction::new("HALT", "", "")); // compiles to 1 byte
    
    for i in 10..328 {
        instructions.push(Instruction::new("NOP", "", ""));
    }

    instructions
}

pub fn ending_subroutine() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    instructions.push(Instruction::new("HALT", "", "")); // stop cpu after program is done

    instructions
}