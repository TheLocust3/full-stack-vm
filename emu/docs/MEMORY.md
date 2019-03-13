# Memory

64bit addresses, each address contains a byte.

Capped at 1024Mb of memory (1073741824 bytes):  
0x00000000 - 0x0000003F: Program space (64 bytes of memory)
0x00000040 - 0x0000013F: Reserved space (256 bytes of memory)
0x00000140 - 0x3FFEFFFF: Free Space
0x3FFF0000-0x40000000: Stack space
