extern crate log;
extern crate env_logger;
extern crate bitwise;

pub mod memory;
pub mod instructions;
pub mod cpu;
pub mod computer;

pub use cpu::register;

pub mod test_program;
