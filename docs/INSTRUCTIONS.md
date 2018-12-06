# Instructions

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

### lam

## Memory Operations
