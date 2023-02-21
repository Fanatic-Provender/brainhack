/*

In ASM              | In BrainFuck
--------------------+----------------------
16384 RAM Words     | 49152
8192 Screen Words   | 24576
1 Kbd Word          | 3

  0   1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| A | A | T | D | D | T | M | M | T | R | R | T | F | T | T | T | T | T |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+

  18  19  20  21  22  23  24  25  26  27  28  29  30  31  32  33  34  35
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| S | S | G | S | S | G | S | S | G | S | S | G | S | S | G | S | S | G |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+

...

  18  19  20  21  22  23  24  25  26  27  28  29  30  31  32  33  34  35
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| S | S | G | S | S | G | S | S | G | S | S | G | S | S | G | S | S | G |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+

*/

use sdl2::keyboard::Keycode;
use anyhow::{Error, Result};


const REGISTER_BUFFER: usize = 18;
const SCREEN: usize = 24576;
const RAM: usize = 49152;
const KBD: usize = 3;
const TAPE_SIZE: usize = REGISTER_BUFFER + RAM + SCREEN + KBD;

pub struct Tape {
    cell_wrapping: bool,
    io: bool,
    mem_ptr: usize,
    mem_buffer: [u8; TAPE_SIZE] // TODO: Update data type to reduce memory foot-print
}

impl Tape {
    pub fn new(cell_wrapping: bool, io: bool) -> Self {
        Self {
            cell_wrapping: cell_wrapping,
            io: io,
            mem_ptr: 0,
            mem_buffer: [0; TAPE_SIZE]
        }
    }

    pub fn inc_ptr(&mut self) -> Result<()> {
        if self.mem_ptr >= TAPE_SIZE {
            Error::msg("Memory Pointer Overflow!");
        }
        self.mem_ptr += 1;
        Ok(())
    }

    pub fn dec_ptr(&mut self) -> Result<()> {
        if self.mem_ptr == 0 {
            Error::msg("Memory Pointer Underflow!");
        }
        self.mem_ptr -= 1;
        Ok(())
    }

    pub fn inc_cell(&mut self) -> Result<()> { 
        if self.cell_wrapping {
            self.mem_buffer[self.mem_ptr] = self.mem_buffer[self.mem_ptr].wrapping_add(1);
        } else if self.mem_buffer[self.mem_ptr] == 255 {
            Error::msg("Cell Overflow!");
        } else {
            self.mem_buffer[self.mem_ptr] += 1;
        }
        Ok(())
    }
    pub fn dec_cell(&mut self) -> Result<()> {
        if self.cell_wrapping {
            self.mem_buffer[self.mem_ptr] = self.mem_buffer[self.mem_ptr].wrapping_sub(1);
        } else if self.mem_buffer[self.mem_ptr] == 0 {
            Error::msg("Cell Underflow!");
        } else {
            self.mem_buffer[self.mem_ptr] -= 1;
        }
        Ok(())
    }

    pub fn cell(&self) -> u8 {
        self.mem_buffer[self.mem_ptr]
    }


    pub fn get_pixels(&self) -> Vec<(u8, u8, u8)> {
        self.mem_buffer[REGISTER_BUFFER + RAM - 1..TAPE_SIZE - 1]
            .chunks(3)
            .map(|window| (window[0], window[1], window[2]))
            .collect()
    }

    pub fn update_kbd(&mut self, key: Keycode) {
        todo!()
    }
}
