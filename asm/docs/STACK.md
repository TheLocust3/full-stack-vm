# The Stack

Memory space associated with push and pop commands. Used for user defined functions and assembler subroutines.

Address Breakdown (start and end values are inclusive):
0-7: Stack pointer
8-65535: Stack space

Stack Pointer:
Points to address of the most recent value pushed onto the stack. Incremented by 8 on ever push command and decremented by 8 on every pop command.
