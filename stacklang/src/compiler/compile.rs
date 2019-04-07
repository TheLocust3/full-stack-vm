use instruction::Instruction;
use instruction::InstructionTree;
use instruction::Value;
use instruction::Variable;
use instruction::Number;
use instruction::Thunk;
use compiler::helpers::add_instruction;
use compiler::arithmetic;
use compiler::control;
use compiler::miscellaneous;
use compiler::values;

pub fn compile(instructions: Vec<Instruction>) -> String {
    let mut compiled: String = "".to_string();

    for instruction in instructions {
        compiled = add_instruction(compiled, &compile_instruction(instruction));
    }

    compiled
}

pub fn compile_instruction(instruction: Instruction) -> String {
    let command = instruction.command.clone();

    match command.as_str() {
        "push" => {
            match instruction.arg1 {
                InstructionTree::Nodes(nodes) => {
                    println!("Bad input on push!");
                    "".to_string()
                },
                InstructionTree::Value(value) => {
                    // TODO: Different compile push for different values
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
    }
}

pub fn compile_value(value: Value) -> String {
    match value {
        Value::Variable(var) => {
            values::compile_variable(var.var)
        },
        Value::Number(num) => {
            values::compile_number(num.number)
        },
        Value::Thunk(thunk) => {
            values::compile_thunk(compile(thunk.instructions))
        }
    }
}
