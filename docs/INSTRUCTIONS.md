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

Format: 0000 1rrr

- 0000 1000 - SET A
- 0000 1001 - SET B
- 0000 1010 - SET C
- 0000 1100 - SET E
- 0000 1110 - SET HL

### move

Sets output register to the value stored in input register

Exceptions:

- Cannot set F register

Flags:

- None (does not overwrite register)

#### Opcodes

Format: 0rrr 0rrr

- 0000 0001 MOVE A B
- 0000 0010 MOVE A C
- 0000 0011 MOVE A D
- 0000 0100 MOVE A E
- 0000 0101 MOVE A F
- 0000 0110 MOVE A HL
- 0001 0000 MOVE B A
- 0001 0010 MOVE B C
- 0001 0011 MOVE B D
- 0001 0100 MOVE B E
- 0001 0101 MOVE B F
- 0001 0110 MOVE B HL
- 0010 0000 MOVE C A
- 0010 0001 MOVE C B
- 0010 0011 MOVE C D
- 0010 0100 MOVE C E
- 0010 0101 MOVE C F
- 0010 0110 MOVE C HL
- 0011 0000 MOVE D A
- 0011 0001 MOVE D B
- 0011 0010 MOVE D C
- 0011 0100 MOVE D E
- 0011 0101 MOVE D F
- 0011 0110 MOVE D HL
- 0100 0000 MOVE E A
- 0100 0001 MOVE E B
- 0100 0010 MOVE E C
- 0100 0011 MOVE E D
- 0100 0101 MOVE E F
- 0100 0110 MOVE E HL
- 0110 0000 MOVE HL A
- 0110 0001 MOVE HL B
- 0110 0010 MOVE HL C
- 0110 0011 MOVE HL D
- 0110 0100 MOVE HL E
- 0110 0101 MOVE HL F

First rrr is output register, second rrr is input register

## Arithmetic Operations

### add

Adds two registers together and stores the result in register A.

Flags:

- Overflow: If number is greater than u64::max_value()

#### Opcodes

Format: rrr0 1111

- 0000 1111 ADD A
- 0010 1111 ADD B
- 0100 1111 ADD C
- 0110 1111 ADD D
- 1000 1111 ADD E
- 1010 1111 ADD F
- 1100 1111 ADD HL

### sub

Subtracts register from register A and stores the result in register A.

Flags:

- Negative: If number is less than 0

#### Opcodes

Format: rrr1 1111

- 0001 1111 SUB A
- 0011 1111 SUB B
- 0101 1111 SUB C
- 0111 1111 SUB D
- 1001 1111 SUB E
- 1011 1111 SUB F
- 1101 1111 SUB HL

## Bitwise Operations

### and

Runs an and operation on register A and a register and stores the result in A.

#### Opcodes

Format: rrr1 1000

- 0001 1000 AND A
- 0011 1000 AND B
- 0101 1000 AND C
- 0111 1000 AND D
- 1001 1000 AND E
- 1011 1000 AND F
- 1101 1000 AND HL

### or

Runs an or operation on register A and a register and stores the result in A.

#### Opcodes

Format: rrr1 1001

- 0001 1001 OR A
- 0011 1001 OR B
- 0101 1001 OR C
- 0111 1001 OR D
- 1001 1001 OR E
- 1011 1001 OR F
- 1101 1001 OR HL

### not

Runs a not operation on a register and stores the result in that register.

#### Opcodes

Format: rrr1 1011

- 0001 1011 NOT A
- 0011 1011 NOT B
- 0101 1011 NOT C
- 0111 1011 NOT D
- 1001 1011 NOT E
- 1101 1011 NOT HL

### shift-left

Shifts the register to the left one bit and stores the result in that register. Highest bit is placed on the new lowest order bit if operation is selected, otherwise the bit is lost.

Flags:

- Overflow: stores old highest order bit if wrap-around isn't selected

#### Opcodes

Format: rrr1 1w01

w: wraparound bit. If 0 wraps

- 0001 1001 SHIFT_LEFT A, wrap=true
- 0011 1001 SHIFT_LEFT B, wrap=true
- 0101 1001 SHIFT_LEFT C, wrap=true
- 0111 1001 SHIFT_LEFT D, wrap=true
- 1001 1001 SHIFT_LEFT E, wrap=true
- 1101 1001 SHIFT_LEFT HL, wrap=true
- 0001 1101 SHIFT_LEFT A, wrap=false
- 0011 1101 SHIFT_LEFT B, wrap=false
- 0101 1101 SHIFT_LEFT C, wrap=false
- 0111 1101 SHIFT_LEFT D, wrap=false
- 1001 1101 SHIFT_LEFT E, wrap=false
- 1101 1101 SHIFT_LEFT HL, wrap=false

### shift-right

Shifts the register to the right one bit and stores the result in that register. Lowest bit is placed on the new highest order bit if operation is selected, otherwise the bit is lost.

Flags:

- Overflow: stores old lowest order bit if wrap-around isn't selected

#### Opcodes

Format: rrr1 1w11

w: wraparound bit. If 0 wraps

- 0001 1011 SHIFT_RIGHT A, wrap=true
- 0011 1011 SHIFT_RIGHT B, wrap=true
- 0101 1011 SHIFT_RIGHT C, wrap=true
- 0111 1011 SHIFT_RIGHT D, wrap=true
- 1001 1011 SHIFT_RIGHT E, wrap=true
- 1101 1011 SHIFT_RIGHT HL, wrap=true
- 0001 1111 SHIFT_RIGHT A, wrap=false
- 0011 1111 SHIFT_RIGHT B, wrap=false
- 0101 1111 SHIFT_RIGHT C, wrap=false
- 0111 1111 SHIFT_RIGHT D, wrap=false
- 1001 1111 SHIFT_RIGHT E, wrap=false
- 1101 1111 SHIFT_RIGHT HL, wrap=false

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
- Cannot set F register

#### Opcodes

Format: 1111 1rrr

- 1111 1000 read8 A
- 1111 1001 read8 B
- 1111 1010 read8 C
- 1111 1011 read8 D
- 1111 1100 read8 E

### read16

Reads 16bits of information at location stored in HL and loads it into register.

Exceptions:

- Cannot set HL register
- Cannot set F register

#### Opcodes

Format: 1111 0rrr

- 1111 0000 read16 A
- 1111 0001 read16 B
- 1111 0010 read16 C
- 1111 0011 read16 D
- 1111 0100 read16 E

### read32

Reads an 32bits of information at location stored in HL and loads it into register.

Exceptions:

- Cannot set HL register
- Cannot set F register

#### Opcodes

Format: 1110 0rrr

- 1110 0000 read32 A
- 1110 0001 read32 B
- 1110 0010 read32 C
- 1110 0011 read32 D
- 1110 0100 read32 E

### read64

Reads 64 bits of information at location stored in HL and loads it into register.

Exceptions:

- Cannot set HL register
- Cannot set F register

#### Opcodes

Format: 1100 0rrr

- 1100 0000 read64 A
- 1100 0001 read64 B
- 1100 0010 read64 C
- 1100 0011 read64 D
- 1100 0100 read64 E

### write8

Writes information stored in register to location stored in HL. Only writes lowest order byte in register.

Exceptions:

- Cannot specify HL register

#### Opcodes

Format: 1000 0rrr

- 1000 0000 write8 A
- 1000 0001 write8 B
- 1000 0010 write8 C
- 1000 0011 write8 D
- 1000 0100 write8 E
- 1000 0101 write8 F

### write16

Writes information stored in register to location stored in HL. Only writes lowest two order bytes in register.

Exceptions:

- Cannot specify HL register

#### Opcodes

Format: 1001 0rrr

- 1001 1000 write16 A
- 1001 1001 write16 B
- 1001 1010 write16 C
- 1001 1011 write16 D
- 1001 1100 write16 E
- 1001 1101 write16 F

### write32

Writes information stored in register to location stored in HL. Only writes lowest three order bytes in register.

Exceptions:

- Cannot specify HL register

#### Opcodes

Format: 1011 0rrr

- 1011 1000 write32 A
- 1011 1001 write32 B
- 1011 1010 write32 C
- 1011 1011 write32 D
- 1011 1100 write32 E
- 1011 1101 write32 F

### write64

Writes information stored in register to location stored in HL.

Exceptions:

- Cannot specify HL register

#### Opcodes

Format: 1010 0rrr

- 1010 1000 write32 A
- 1010 1001 write32 B
- 1010 1010 write32 C
- 1010 1011 write32 D
- 1010 1100 write32 E
- 1010 1101 write32 F

## Miscellaneous

### nop

Does nothing.

#### Opcodes

Format: 0000 0000
