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


use super::utils::pause;
use anyhow::{bail, Result};
use std::io::Read;

pub const REGISTER_BUFFER: usize = 18;
pub const SCREEN: usize = 24576;
pub const RAM: usize = 49152;
pub const KBD: usize = 3;
pub const TAPE_SIZE: usize = REGISTER_BUFFER + RAM + SCREEN + KBD;

pub struct Tape {
    mem_ptr: usize,
    mem_buffer: [u8; TAPE_SIZE],
}

impl Tape {
    pub fn inc_ptr(&mut self, batch: usize) -> Result<()> {
        if self.mem_ptr + batch > TAPE_SIZE {
            bail!("Memory pointer overflow")
        }
        self.mem_ptr += batch;
        Ok(())
    }

    pub fn dec_ptr(&mut self, batch: usize) -> Result<()> {
        if (self.mem_ptr as isize - batch as isize) < 0 {
            bail!("Memory pointer underflow")
        }
        self.mem_ptr -= batch;
        Ok(())
    }

    pub fn inc_cell(&mut self, batch_size: usize, mem_ptr_offset: isize) -> Result<()> {
        if (self.mem_ptr as isize + mem_ptr_offset) < 0
            || (self.mem_ptr as isize + mem_ptr_offset) > TAPE_SIZE as isize
        {
            bail!("Memory pointer out of bounds")
        }

        self.mem_buffer[(self.mem_ptr as isize + mem_ptr_offset) as usize] = self.mem_buffer
            [(self.mem_ptr as isize + mem_ptr_offset) as usize]
            .wrapping_add(batch_size as u8);
        Ok(())
    }

    pub fn dec_cell(&mut self, batch_size: usize, mem_ptr_offset: isize) -> Result<()> {
        if (self.mem_ptr as isize + mem_ptr_offset) < 0
            || (self.mem_ptr as isize + mem_ptr_offset) > TAPE_SIZE as isize
        {
            bail!("Memory pointer out of bounds")
        }

        self.mem_buffer[(self.mem_ptr as isize + mem_ptr_offset) as usize] = self.mem_buffer
            [(self.mem_ptr as isize + mem_ptr_offset as isize) as usize]
            .wrapping_sub(batch_size as u8);
        Ok(())
    }

    pub fn breakpoint(&self) {
        eprintln!("\n\n+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+     +-----+-----+");
        eprintln!("|  T0 |  Au |  Al |  T1 |  Du |  Dl |  T2 |  Mu |  Ml |  T3 |  Ru |  Rl |  T4 |  Pu |  Pl |  T5 |  F  |  T6 |  T7 |     | KBD | KBD |");
        eprintln!("+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+ ... +-----+-----+");
        eprintln!(
            "| {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} | {: ^3} |     | {: ^3} | {: ^3} |", 
            self.mem_buffer[0],
            self.mem_buffer[1],
            self.mem_buffer[2],
            self.mem_buffer[3],
            self.mem_buffer[4],
            self.mem_buffer[5],
            self.mem_buffer[6],
            self.mem_buffer[7],
            self.mem_buffer[8],
            self.mem_buffer[9],
            self.mem_buffer[10],
            self.mem_buffer[11],
            self.mem_buffer[12],
            self.mem_buffer[13],
            self.mem_buffer[14],
            self.mem_buffer[15],
            self.mem_buffer[16],
            self.mem_buffer[17],
            self.mem_buffer[18],
            self.mem_buffer[TAPE_SIZE - 2],
            self.mem_buffer[TAPE_SIZE - 1]
        );
        eprintln!("+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+     +-----+-----+\n");
        eprintln!("+----------+-------+");
        eprintln!("| Register | Value |");
        eprintln!("+----------+-------+");
        eprintln!("|     A    |{: ^7}|", 5); // TODO: replace with 16 Bit representation
        eprintln!("+----------+-------+");
        eprintln!("|     D    |{: ^7}|", 5); // TODO: replace with 16 Bit representation
        eprintln!("+----------+-------+");
        eprintln!("|     M    |{: ^7}|", 5); // TODO: replace with 16 Bit representation
        eprintln!("+----------+-------+");
        eprintln!("|    KBD   |{: ^7}|", 5); // TODO: replace with Pressed key representation
        eprintln!("+----------+-------+");
        pause()
    }
}

impl Tape {
    pub fn new() -> Self {
        Self {
            mem_ptr: 0,
            mem_buffer: [0; TAPE_SIZE],
        }
    }

    pub fn get_current_cell(&self) -> u8 {
        self.mem_buffer[self.mem_ptr]
    }

    pub fn get_cell(&self, index: usize) -> Option<u8> {
        self.mem_buffer.get(index).copied()
    }

    pub fn get_slice(&self, start: usize, end: usize) -> Option<&[u8]> {
        if start > end || end > TAPE_SIZE {
            return None;
        }
        Some(&self.mem_buffer[start..=end])
    }

    pub fn clear(&mut self) {
        self.mem_ptr = 0;
        self.mem_buffer = [0; TAPE_SIZE];
    }
}
