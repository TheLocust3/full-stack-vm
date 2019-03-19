use instruction::Instruction;
use instruction::InstructionTree;

pub fn compile(instructions: Vec<Instruction>) -> String {
    let mut compiled: String = "".to_string();

    for instruction in instructions {
        compiled = format!("{}\n{}", compiled, compile_instruction(instruction));
    }

    compiled
}

pub fn compile_instruction(instruction: Instruction) -> String {
    format!("{}\n{}", compile_instruction_tree(instruction.arg1), compile_instruction_tree(instruction.arg2))
}

pub fn compile_instruction_tree(tree: InstructionTree) -> String {
    match tree {
        InstructionTree::Nodes(instructions) => {
            let mut compiled: String = "".to_string();

            for instruction in instructions {
                // handle instruction
                compiled = format!("{}\n{}", compiled, compile_instruction(instruction));
            }

            compiled
        },
        InstructionTree::Value(value) => {
            // handle value
            "".to_string()
        }
    }
}
