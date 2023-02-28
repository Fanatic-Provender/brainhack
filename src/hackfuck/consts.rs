// In ASM              | In BrainFuck
// --------------------+----------------------
// 16384 RAM Words     | 49152
// 8192 Screen Words   | 24576
// 1 Kbd Word          | 3


/// Number of cells allocated to the named in the brainfuck memory-cell buffer
pub const REGISTER_BUFFER: usize = 27;

/// Number of cells allocated to the Screen in the brainfuck memory-cell buffer
pub const SCREEN: usize = 8192 * 3;

/// Number of cells allocated to the RAM in the brainfuck memory-cell buffer
pub const RAM: usize = 16384 * 3;

/// Number of cells allocated to the Keyboard in the brainfuck memory-cell buffer
pub const KBD: usize = 3;

/// Total number of cells to represent the whole system in brainfuck
pub const TAPE_SIZE: usize = REGISTER_BUFFER + RAM + SCREEN + KBD;
