use cpu::cpu::CPU;
use computer::Computer;

pub fn test_program(program: Vec<u8>) -> CPU {
    let mut computer: Computer = Computer::new();
    computer = computer.read_test_program(program);

    while !computer.is_stopped() {
        computer = computer.cycle();
    }

    computer.cpu
}