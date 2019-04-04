pub enum InstructionTree {
    Nodes(Vec<Instruction>),
    Value(Value)
}

pub struct Instruction {
    pub command: String,
    pub arg1: InstructionTree,
    pub arg2: InstructionTree,
}

pub enum Value {
    Variable(Variable),
    Number(Number),
    Thunk(Thunk)
}

pub struct Variable {
    pub var: String,
}

pub struct Number {
    pub number: String,
}

pub struct Thunk {
    pub instructions: Vec<Instruction>,
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
