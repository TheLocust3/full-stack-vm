#[macro_use] extern crate log;
extern crate env_logger;

pub mod bitwise;
pub mod memory;
pub mod cpu;
pub use cpu::register;
