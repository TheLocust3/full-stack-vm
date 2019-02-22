# The Stack

Memory space associated with push and pop commands. Used for user defined functions and assembler subroutines.

Address Breakdown (start and end values are inclusive):
0x3FFF0000-0x40000000: Stack space

Stack Pointer (SP):
Points to address of the most recent value pushed onto the stack. Decremented on every push and incremented on every pop. The bottom of the stack is at the highest memory location (the stack descends downwards).
