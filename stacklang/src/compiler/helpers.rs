pub fn add_instruction(compiled: String, instruction: &str) -> String {
    format!("{}\n{}", compiled, instruction)
}