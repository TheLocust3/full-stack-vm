# Instructions

All possible instructions in asm. One instruction per line.

Structure: COMMAND ARG1 ARG2

ARG1 and ARG2 or either registers or addresses.

Registers are either `A`, `B`, `C`, `D`, `E`, `F`, `HL`.
Address are a 64bit unsigned integer.

## MOVE

Stores the value in ARG2 (whether it be the value of the register or the value stored at the address in memory) in ARG1 (either register or address)

Arguments:

- ARG1: destination register or destination address
- ARG2: register or address

Ex. MOVE A B
Ex. MOVE A 1
Ex. MOVE 1 B
Ex. MOVE 1 10

## ADD

Adds the value in register specified in ARG1 to the value in register specified in ARG2 and stores the result in the register specified in ARG2

Arguments:

- ARG1: destination register
- ARG2: register

Ex. ADD A B

## SUB

Subtracts the value in the register specified in ARG1 by the value in the register specified in ARG2 and stores the result in the register specified in ARG2

Arguments:

- ARG1: destination register
- ARG2: register

Ex. SUB A B

## AND

Runs bitwise and on the value in the register specified in ARG1 with the value in the register specificed in ARG2 and stores the result in the register specified in ARG2.

Arguments:

- ARG1: destination register
- ARG2: register

Ex. AND A B

## OR

Runs bitwise or on the value in the register specified in ARG1 with the value in the register specificed in ARG2 and stores the result in the register specified in ARG2.

Arguments:

- ARG1: destination register
- ARG2: register

Ex. OR A B

## NOT

Runs bitwise not on the value in the register specified in ARG1 and stores the result in the register specified in ARG2.

Arguments:

- ARG1: destination register
- ARG2: none

Ex. NOT A

## SHIFT_LEFT

Runs bitwise shift left once on the value in the register specified in ARG1 and stores the result in the register specified in ARG2. Does not wrap.

Arguments:

- ARG1: destination register
- ARG2: none

Ex. SHIFT_LEFT A

## SHIFT_LEFT_W

Runs bitwise shift left once on the value in the register specified in ARG1 and stores the result in the register specified in ARG2. Wraps the shifted bit.

Arguments:

- ARG1: destination register
- ARG2: none

Ex. SHIFT_LEFT_W A

## SHIFT_RIGHT

Runs bitwise shift right once on the value in the register specified in ARG1 and stores the result in the register specified in ARG2. Does not wrap.

Arguments:

- ARG1: destination register
- ARG2: none

Ex. SHIFT_RIGHT A

## SHIFT_RIGHT_W

Runs bitwise shift right once on the value in the register specified in ARG1 and stores the result in the register specified in ARG2. Wraps the shifted bit.

Arguments:

- ARG1: destination register
- ARG2: none

Ex. SHIFT_RIGHT_W A

## JUMP

Jumps to instructions starting at memory address ARG1.

Arguments:

- ARG1: destination address
- ARG2: none

Ex. JUMP 1

## JUMP0

Jumps to instructions starting at memory address ARG1 if the value in register A is zero.

Arguments:

- ARG1: destination address
- ARG2: none

Ex. JUMP0 1

## NOP

Does nothing.

Arguments:

- ARG1: none
- ARG2: none

Ex: NOP
