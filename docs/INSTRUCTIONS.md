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

#### Opcodes

Format: 1000 0rrr

- 1000 0000 - SET A
- 1000 0001 - SET B
- 1000 0010 - SET C
- 1000 0100 - SET E
- 1000 0110 - SET HL

### move

Sets output register to the value stored in input register

Exceptions:

- Cannot set F register

Flags:

- None (does not overwrite register)

#### Opcodes

Format: 0rrr 0rrr

-0000 0001 MOVE A B
-0000 0010 MOVE A C
-0000 0011 MOVE A D
-0000 0100 MOVE A E
-0000 0101 MOVE A F
-0000 0110 MOVE A HL
-0001 0000 MOVE B A
-0001 0010 MOVE B C
-0001 0011 MOVE B D
-0001 0100 MOVE B E
-0001 0101 MOVE B F
-0001 0110 MOVE B HL
-0010 0000 MOVE C A
-0010 0001 MOVE C B
-0010 0011 MOVE C D
-0010 0100 MOVE C E
-0010 0101 MOVE C F
-0010 0110 MOVE C HL
-0011 0000 MOVE D A
-0011 0001 MOVE D B
-0011 0010 MOVE D C
-0011 0100 MOVE D E
-0011 0101 MOVE D F
-0011 0110 MOVE D HL
-0100 0000 MOVE E A
-0100 0001 MOVE E B
-0100 0010 MOVE E C
-0100 0011 MOVE E D
-0100 0101 MOVE E F
-0100 0110 MOVE E HL
-0110 0000 MOVE HL A
-0110 0001 MOVE HL B
-0110 0010 MOVE HL C
-0110 0011 MOVE HL D
-0110 0100 MOVE HL E
-0110 0101 MOVE HL F

First rrr is output register, second rrr is input register

## Arithmetic Operations

### add

Adds two registers together and stores the result in register A.

Flags:

- Overflow: If number is greater than u64::max_value()

#### Opcodes

Format: 0001 0rrr

### sub

Subtracts register from register A and stores the result in register A.

Flags:

- Negative: If number is less than 0

#### Opcodes

Format: 0010 0rrr

## Bitwise Operations

### and

Runs an and operation on register A and a register and stores the result in A.

#### Opcodes

Format: 0000 1rrr

### or

Runs an or operation on register A and a register and stores the result in A.

Opcode: 0001 1rrr

### not

Runs a not operation on a register and stores the result in that register.

#### Opcodes

Format: 0011 1rrr

### shift-left

Shifts the register to the left one bit and stores the result in that register. Highest bit is placed on the new lowest order bit if operation is selected, otherwise the bit is lost.

Flags:

- Overflow: stores old highest order bit if wrap-around isn't selected

#### Opcodes

Format: w011 0rrr

w: wraparound bit. If 0 wraps

### shift-right

Shifts the register to the right one bit and stores the result in that register. Lowest bit is placed on the new highest order bit if operation is selected, otherwise the bit is lost.

Flags:

- Overflow: stores old lowest order bit if wrap-around isn't selected

#### Opcodes

Format: w010 1rrr

w: wraparound bit. If 0 wraps

## Control Operations

### jump

Jump to location stored in the next four bytes.

#### Opcodes

Format: 1111 1111

### jump0

If value stored in register A is 0 jump to location stored in the next four bytes.

#### Opcodes

Format: 1111 1110

## Memory Operations

### read8

Reads an a byte of information at location stored in HL and loads it into register.

Exceptions:

- Cannot set HL register

#### Opcodes

Format: 1rrr 1001

### read16

Reads 16bits of information at location stored in HL and loads it into register.

Exceptions:

- Cannot set HL register

#### Opcodes

Format: 1rrr 1010

### read32

Reads an 32bits of information at location stored in HL and loads it into register.

Exceptions:

- Cannot set HL register

#### Opcodes

Format: 1rrr 1011

### read64

Reads 64 bits of information at location stored in HL and loads it into register.

Exceptions:

- Cannot set HL register

#### Opcodes

Format: 1rrr 1100

### write8

Writes information stored in register to location stored in HL. Only writes lowest order byte in register.

Exceptions:

- Cannot specify HL register

#### Opcodes

Format: 1rrr 0001

### write16

Writes information stored in register to location stored in HL. Only writes lowest two order bytes in register.

Exceptions:

- Cannot specify HL register

#### Opcodes

Format: 1rrr 0010

### write32

Writes information stored in register to location stored in HL. Only writes lowest three order bytes in register.

Exceptions:

- Cannot specify HL register

#### Opcodes

Format: 1rrr 0011

### write64

Writes information stored in register to location stored in HL.

Exceptions:

- Cannot specify HL register

#### Opcodes

Format: 1rrr 0100

## Miscellaneous

### nop

Does nothing.

#### Opcodes

Format: 0000 0000
