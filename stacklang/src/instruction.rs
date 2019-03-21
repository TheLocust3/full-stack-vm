pub enum InstructionTree {
    Nodes(Vec<Instruction>),
    Value(String)
}

pub struct Instruction {
    pub command: String,
    pub arg1: InstructionTree,
    pub arg2: InstructionTree,
}

impl Instruction {
    pub fn new(command: &str, arg1: InstructionTree, arg2: InstructionTree) -> Instruction {
        Instruction {
            command: command.to_string(),
            arg1: arg1,
            arg2: arg2
        }
    }
}
