extern crate log;
extern crate env_logger;
extern crate bitwise;

mod instruction;
mod compiler;
mod parser;
mod converter;

fn main() {
    env_logger::init();
    
    print!("TEST");
}
