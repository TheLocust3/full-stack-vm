extern crate emu;
use self::emu::memory::Memory;

#[test]
fn test_new() {
    let mut memory = Memory::new();

    assert_eq!(memory.memory.len(), 1073741824);
    assert_eq!(memory.memory[124], 0);
}

#[test]
fn test_read_write_8bit() {
    let mut memory = Memory::new();
    memory.write_8bit(124, 10);

    assert_eq!(memory.read_8bit(124), 10);
}

#[test]
fn test_read_write_8bit_0() {
    let mut memory = Memory::new();
    memory.write_8bit(0, 10);

    assert_eq!(memory.read_8bit(0), 10);
}

#[test]
fn test_read_write_8bit_max() {
    let mut memory = Memory::new();
    memory.write_8bit(1073741824 - 1, 10);

    assert_eq!(memory.read_8bit(1073741824 - 1), 10);
}

#[test]
fn test_read_write_16bit() {
    let mut memory = Memory::new();
    memory.write_16bit(124, 10);

    assert_eq!(memory.read_16bit(124), 10);
}

#[test]
fn test_read_write_16bit_0() {
    let mut memory = Memory::new();
    memory.write_16bit(0, 10);

    assert_eq!(memory.read_16bit(0), 10);
}

#[test]
fn test_read_write_16bit_max() {
    let mut memory = Memory::new();
    memory.write_16bit(1073741824 - 2, 10);

    assert_eq!(memory.read_16bit(1073741824 - 2), 10);
}

#[test]
fn test_read_write_32bit() {
    let mut memory = Memory::new();
    memory.write_32bit(124, 10);

    assert_eq!(memory.read_32bit(124), 10);
}

#[test]
fn test_read_write_32bit_0() {
    let mut memory = Memory::new();
    memory.write_32bit(0, 10);

    assert_eq!(memory.read_32bit(0), 10);
}

#[test]
fn test_read_write_32bit_max() {
    let mut memory = Memory::new();
    memory.write_32bit(1073741824 - 4, 10);

    assert_eq!(memory.read_32bit(1073741824 - 4), 10);
}

#[test]
fn test_read_write_64bit() {
    let mut memory = Memory::new();
    memory.write_64bit(124, 10);

    assert_eq!(memory.read_64bit(124), 10);
}

#[test]
fn test_read_write_64bit_0() {
    let mut memory = Memory::new();
    memory.write_64bit(0, 10);

    assert_eq!(memory.read_64bit(0), 10);
}

#[test]
fn test_read_write_64bit_max() {
    let mut memory = Memory::new();
    memory.write_64bit(1073741824 - 8, 10);

    assert_eq!(memory.read_64bit(1073741824 - 8), 10);
}
