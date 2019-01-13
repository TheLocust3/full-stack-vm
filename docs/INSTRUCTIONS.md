# Instructions

All instructions that interact with flags reset the flag register first.

## Register Operations

### set

Sets value stored in register to next four bytes after this instruction.

## Arithmetic Operations

### add

Adds two registers together and stores the result in register A.

Flags:

- Overflow: If number is greater than u64::max_value()

### sub

Subtracts register from register A and stores the result in register A.

Flags:

- Negative: If number is less than 0

## Bitwise Operations

### and

Runs an and operation on A and a register and stores the result in A.

### or

Runs an or operation on A and a register and stores the result in A.

### not

Runs a not operation on a register and stores the result in that register.

### shift-left

Shifts the register to the left one bit and stores the result in that register. Highest bit is placed on the new lowest order bit if operation is selected, otherwise the bit is lost.

Flags:

- Overflow: stores old highest order bit if wrap-around isn't selected

### shift-right

Shifts the register to the right one bit and stores the result in that register. Lowest bit is placed on the new highest order bit if operation is selected, otherwise the bit is lost.

Flags:

- Overflow: stores old lowest order bit if wrap-around isn't selected

## Control Operations

### jump

Jump to location stored in the next four bytes.

### jump0

If value stored in register A is 0 jump to location stored in the next four bytes.

## Stack Operations

### push

Pushes value from register onto top of stack.

### pop

Pops value from stack into register.

## Memory Operations

### read8

Reads an a byte of information at location stored in HL and loads it into register

### read16

Reads 16bits of information at location stored in HL and loads it into register

### read32

Reads an 32bits of information at location stored in HL and loads it into register

### read64

Reads 64 bits of information at location stored in HL and loads it into register

### write8

Writes information stored in register to location stored in HL. Only writes lowest order byte in register.

### write16

Writes information stored in register to location stored in HL. Only writes lowest two order bytes in register.

### write32

Writes information stored in register to location stored in HL. Only writes lowest three order bytes in register.

### write64

Writes information stored in register to location stored in HL.

## Miscellaneous

### nop

Does nothing.
