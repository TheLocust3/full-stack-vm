extern crate stacklang;

use self::stacklang::compiler::compile;
use self::stacklang::instruction::Instruction;
use self::stacklang::instruction::InstructionTree;

#[test]
fn test_compile() {
    let program: Vec<Instruction> = vec!(Instruction::new("add", InstructionTree::Value("".to_string()), InstructionTree::Value("".to_string())));
    let compiled = compile::compile(program);

    assert_eq!(compiled, "");
}