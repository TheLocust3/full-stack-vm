# Instructions

## Register Operations

### set

Sets value stored in register to next four bytes after this instruction.

## Arithmetic Operations

### add

Adds the first two values on the Stack together and pushes the result.

Opcode: 0x01  
Arguments: None

Implementation:

1. a = pop()
2. b = pop()
3. push(a + b)

### sub

Subtracts the second value on the Stack from the first value on the stack and pushes the result.

Opcode: 0x02  
Arguments: None

Implementation:

1. a = pop()
2. b = pop()
3. push(a - b)

## Control Operations

### if0

Pops the top value on the stack off. If that value is zero, set PC to first argument, otherwise set it to second argument.

Opcode: 0x05  
Arguments: \[true branch\] \[false branch\]

Implementation:

1. a = pop()
2. if a == 0 then pc = \[true location\] else pc = \[true location\]

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

### ei

Enables interrupts.

### di

Disables interrupts.
