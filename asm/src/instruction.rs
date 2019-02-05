pub struct Instruction {
    pub command: String,
    pub arg1: String,
    pub arg2: String,
}

impl Instruction {
    pub fn new(command: &str, arg1: &str, arg2: &str) -> Instruction {
        Instruction {
            command: command.to_string(),
            arg1: arg1.to_string(),
            arg2: arg2.to_string()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.command, self.arg1, self.arg2)
    }
}
