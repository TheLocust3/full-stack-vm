pub enum InstructionTree {
    Nodes(Vec<Instruction>),
    Value(Value)
}

pub struct Instruction {
    pub command: String,
    pub arg1: InstructionTree,
    pub arg2: InstructionTree,
}

pub struct Value {
    pub value_type: String,
    pub value: String
}
