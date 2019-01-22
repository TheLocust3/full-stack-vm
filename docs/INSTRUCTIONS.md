# Instructions

All instructions that interact with flags reset the flag register first.

## Deciphering Opcodes

Opcodes are 64bit words but are described for convenience as the lowest order 8bits.

Example (nop):
0000 0000

Charactes found in opcodes:

- rrr = three bit register code found in REGISTERS.md

## Register Operations

### set

Sets value stored in register to next four bytes after this instruction.

Exceptions:

- Cannot set F register

Flags:

- None (does not overwrite register)

Opcode: 1000 0rrr

### move

Sets output register to the value stored in input register

Exceptions:

- Cannot set F register

Flags:

- None (does not overwrite register)

Opcode: 0rrr 0rrr

First rrr is output register, second rrr is input register

## Arithmetic Operations

### add

Adds two registers together and stores the result in register A.

Flags:

- Overflow: If number is greater than u64::max_value()

Opcode: 0001 0rrr

### sub

Subtracts register from register A and stores the result in register A.

Flags:

- Negative: If number is less than 0

Opcode: 0010 0rrr

## Bitwise Operations

### and

Runs an and operation on register A and a register and stores the result in A.

Opcode: 0000 1rrr

### or

Runs an or operation on register A and a register and stores the result in A.

Opcode: 0001 1rrr

### not

Runs a not operation on a register and stores the result in that register.

Opcode: 0011 1rrr

### shift-left

Shifts the register to the left one bit and stores the result in that register. Highest bit is placed on the new lowest order bit if operation is selected, otherwise the bit is lost.

Flags:

- Overflow: stores old highest order bit if wrap-around isn't selected

Opcode: w011 0rrr

w: wraparound bit. If 0 wraps

### shift-right

Shifts the register to the right one bit and stores the result in that register. Lowest bit is placed on the new highest order bit if operation is selected, otherwise the bit is lost.

Flags:

- Overflow: stores old lowest order bit if wrap-around isn't selected

Opcode: w010 1rrr

w: wraparound bit. If 0 wraps

## Control Operations

### jump

Jump to location stored in the next four bytes.

Opcode: 1111 1111

### jump0

If value stored in register A is 0 jump to location stored in the next four bytes.

Opcode: 1111 1110

## Memory Operations

### read8

Reads an a byte of information at location stored in HL and loads it into register.

Opcode: 1rrr 1001

Exceptions:

- Cannot set HL register

### read16

Reads 16bits of information at location stored in HL and loads it into register.

Opcode: 1rrr 1010

Exceptions:

- Cannot set HL register

### read32

Reads an 32bits of information at location stored in HL and loads it into register.

Opcode: 1rrr 1011

Exceptions:

- Cannot set HL register

### read64

Reads 64 bits of information at location stored in HL and loads it into register.

Opcode: 1rrr 1100

Exceptions:

- Cannot set HL register

### write8

Writes information stored in register to location stored in HL. Only writes lowest order byte in register.

Opcode: 1rrr 0001

Exceptions:

- Cannot specify HL register

### write16

Writes information stored in register to location stored in HL. Only writes lowest two order bytes in register.

Opcode: 1rrr 0010

Exceptions:

- Cannot specify HL register

### write32

Writes information stored in register to location stored in HL. Only writes lowest three order bytes in register.

Opcode: 1rrr 0011

Exceptions:

- Cannot specify HL register

### write64

Writes information stored in register to location stored in HL.

Opcode: 1rrr 0100

Exceptions:

- Cannot specify HL register

## Miscellaneous

### nop

Does nothing.

Opcode: 0
