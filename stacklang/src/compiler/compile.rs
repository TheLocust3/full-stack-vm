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
                let command = instruction.command.clone();
                compiled = format!("{}\n{}", compiled, compile_instruction(instruction));

                // let single: String = "".to_string();
                let single: String = match command.as_str() {
                    "push" => {
                        "push".to_string()
                    },
                    "add" => {
                        "add".to_string()
                    },
                    "sub" => {
                        "sub".to_string()
                    },
                    "if0" => {
                        "if0".to_string()
                    },
                    "call" => {
                        "call".to_string()
                    },
                    "lam" => {
                        "lam".to_string()
                    },
                    _ => {
                        "".to_string()
                    }
                };

                compiled = format!("{}\n{}", compiled, single);
            }

            compiled
        },
        InstructionTree::Value(value) => {
            value
        }
    }
}
