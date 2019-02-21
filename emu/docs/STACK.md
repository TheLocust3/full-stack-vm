# The Stack

Memory space associated with push and pop commands. Used for user defined functions and assembler subroutines.

Address Breakdown (start and end values are inclusive):
0-65535: Stack space

Stack Pointer (SP):
Points to address of the most recent value pushed onto the stack. Incremented by on every push and decremented on every pop.
