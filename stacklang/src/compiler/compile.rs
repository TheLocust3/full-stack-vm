use instruction::Instruction;
use instruction::InstructionTree;
use compiler::helpers::add_instruction;
use compiler::arithmetic;
use compiler::control;
use compiler::miscellaneous;

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

                // let single: String = "".to_string();
                let single: String = match command.as_str() {
                    "push" => {
                        match instruction.arg1 {
                            InstructionTree::Nodes(nodes) => {
                                println!("Bad input on push!");
                                "".to_string()
                            },
                            InstructionTree::Value(value) => {
                                miscellaneous::compile_push(compile_value(value))
                            }
                        }
                    },
                    "add" => {
                        arithmetic::compile_add()
                    },
                    "sub" => {
                        arithmetic::compile_sub()
                    },
                    "if0" => {
                        match instruction.arg1 {
                            InstructionTree::Nodes(branch1) => {
                                match instruction.arg2 {
                                    InstructionTree::Nodes(branch2) => {
                                        control::compile_if0(compile(branch1), compile(branch2))
                                    },
                                    InstructionTree::Value(value) => {
                                        println!("Bad input on if0!");
                                        "".to_string()
                                    }
                                }
                            },
                            InstructionTree::Value(value) => {
                                println!("Bad input on if0!");
                                "".to_string()
                            }
                        }
                    },
                    "call" => {
                        match instruction.arg1 {
                            InstructionTree::Nodes(nodes) => {
                                control::compile_call(compile(nodes))
                            },
                            InstructionTree::Value(value) => {
                                println!("Bad input on call!");
                                "".to_string()
                            }
                        }
                    },
                    "lam" => {
                        match instruction.arg1 {
                            InstructionTree::Nodes(nodes) => {
                                control::compile_lam(compile(nodes))
                            },
                            InstructionTree::Value(value) => {
                                println!("Bad input on lam!");
                                "".to_string()
                            }
                        }
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

pub fn compile_value(value: String) -> String {
    // TODO: Value could be thunk or non-string value
    "".to_string()
}
