extern crate log;
extern crate env_logger;

mod instruction;
mod compiler;
mod parser;
mod converter;

fn main() {
    env_logger::init();
    
    print!("TEST");
}
