extern crate stacklang;

use self::stacklang::compiler::compile;
use self::stacklang::instruction::Instruction;
use self::stacklang::instruction::InstructionTree;

#[test]
fn test_compile_add() {
    let program: Vec<Instruction> = vec!(Instruction::new("add", InstructionTree::Nodes(vec!()), InstructionTree::Nodes(vec!())));
    let compiled = compile::compile(program);

    assert_eq!(compiled, "POP A\nPOP B\nADD A B\nPUSH A\n\n");
}

#[test]
fn test_compile_sub() {
    let program: Vec<Instruction> = vec!(Instruction::new("sub", InstructionTree::Nodes(vec!()), InstructionTree::Nodes(vec!())));
    let compiled = compile::compile(program);

    assert_eq!(compiled, "POP A\nPOP B\nSUB A B\nPUSH A\n\n");
}
