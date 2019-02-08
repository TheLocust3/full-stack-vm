extern crate log;
extern crate env_logger;
extern crate bitwise;

mod register;
mod instruction;
mod compiler;
mod parser;
mod converter;

fn main() {
    env_logger::init();

    // Sample compile
    // Read file
    // Run parser::parse on resulting String
    // Run converter::convert on resulting Instructions
    // Run compiler::compile on resulting Instructions
    // Concatinate resulting binary to a string
    // Save the string into a file
    
    print!("TEST");
}
