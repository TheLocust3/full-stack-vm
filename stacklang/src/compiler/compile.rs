use instruction::Instruction;
use instruction::InstructionTree;
use compiler::helpers::add_instruction;
use compiler::arithmetic;

pub fn compile(instructions: Vec<Instruction>) -> String {
    let mut compiled: String = "".to_string();

    for instruction in instructions {
        compiled = add_instruction(compiled, &compile_instruction(instruction));
    }

    compiled
}

pub fn compile_instruction(instruction: Instruction) -> String {
    add_instruction(compile_instruction_tree(instruction.arg1), &compile_instruction_tree(instruction.arg2))
}

pub fn compile_instruction_tree(tree: InstructionTree) -> String {
    match tree {
        InstructionTree::Nodes(instructions) => {
            let mut compiled: String = "".to_string();

            for instruction in instructions {
                let command = instruction.command.clone();
                compiled = add_instruction(compiled, &compile_instruction(instruction));

                // let single: String = "".to_string();
                let single: String = match command.as_str() {
                    "push" => {
                        "push".to_string()
                    },
                    "add" => {
                        arithmetic::compile_add()
                    },
                    "sub" => {
                        arithmetic::compile_sub()
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

                compiled = add_instruction(compiled, &single);
            }

            compiled
        },
        InstructionTree::Value(value) => {
            value
        }
    }
}
