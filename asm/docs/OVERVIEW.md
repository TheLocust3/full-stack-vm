# Overview

A simple version of assembly the is assembled into custom machine code.

The compiled machine code is loaded into the CPU at address 0 and runs a small start up routine that makes sure that the program doesn't trample over CPU reserved memory space. It then executes the user's program starting at address 128.
