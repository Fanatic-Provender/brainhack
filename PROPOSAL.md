# CSC590 Design Project Proposal: `brainhack`

Authors: Altan Mehmet Ünver and Shiqiao Zhang

## Overview

`brainhack` compiles Hack assembly as defined in the [Nand2Tetris] project into the [brainfuck] programming language, partially to demonstrate the Turing completeness of brainfuck.

## Brainfuck

Invented by Urban Müller in 1993, [brainfuck] is one of the oldest esoteric programming languages.  It is a minimalistic epitome of a Turing complete language with a cell-based memory model.  A brainfuck program operates on a memory tape consisting of cells, all initially set to zero.  A memory pointer always points to a memory cell during the execution of a brainfuck program.  There are only 8 instructions in brainfuck:

| Command | Description |
|:-------:|-------------|
| `>` | Move the memory pointer to the right |
| `<` | Move the memory pointer to the left |
| `+` | Increment the value of the current cell |
| `-` | Decrement the value of the current cell |
| `.` | Write the value of the current cell to standard output as a byte |
| `,` | Read a byte from standard input and store it in the current cell |
| `[` | Jump past the matching `]` instruction if the current cell is `0` |
| `]` | Jump back to the matching `[` instruction if the current cell is nonzero |

`brainhack` assumes 8-bit cells with wrapping arithmetic.  The minimum number of cells required is `3 * ceil(M / 2) + 18`, where `M` is the maximum address accessed by the Hack assembly program.  `brainhack` does not use the `.` and `,` instructions, so every Hack assembly program is compiled to a brainfuck program constructed only from the six instructions `><+-[]`.

## Additional Functionality

To compensate for the memory-bound I/O functionalities supported by the Hack assembly language, `brainhack` ships with a custom brainfuck interpreter.  The interpreter designates parts of the memory buffer to the screen and keyboard to allow for complex I/O operations from the brainfuck code.  It also removes the memory limit of 30,000 cells found in many brainfuck implementations.

## Compilation Process

The compilation of assembly to brainfuck will be achieved through 3 steps.

1. Symbol Resolution
2. Template Substitution
3. Sequential Fabrication

### Symbol Resolution

In this step, all symbolic references in the Hack assembly code are resolved to physical addresses.

### Template Substitution

In this step, we break apart each assembly instruction into its various parts: `comp`, `jump`, `dest`, etc.  Each individual action is substituted with an equivalent brainfuck snippet.  These snippets are short in length and simple in function.  However, chained together, they can perform great tasks.

### Sequential Fabrication

At this stage, the general actions the program will have been resolved.  However, for these instructions to work together to perform a greater task, an additional step must be taken to include brainfuck snippets to update the A, M, and D registers, reset the memory pointer, and handle arbitrary jumps.  This step handles all the heavy lifting allowing the logic from previous steps to be abstracted.

## Timeline

* February 15: Submission of project proposal
* February 16: Implementation of custom brainfuck interpreter
* February 20: Design of core brainfuck templates
* February 23: Establishment of memory model
* February 27: Integration of sequential fabrication
* March 3: Conclusion of project

[Nand2Tetris]: https://www.nand2tetris.org/
[brainfuck]: https://esolangs.org/wiki/brainfuck
