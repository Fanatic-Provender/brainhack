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

    pub fn inc_ptr(&mut self, i: usize) -> Result<()> {
        if self.mem_ptr + i >= TAPE_SIZE {
            Err(Error::msg("Memory Pointer Out of bounds!"))
        } else {
            self.mem_ptr += i;
            Ok(())
        }
        
    }

    pub fn dec_ptr(&mut self, i: usize) -> Result<()> {
        match self.mem_ptr.checked_sub(i) {
            Some(_) => {
                self.mem_ptr -= i;
                Ok(())
            },
            None => Err(Error::msg("Memory Pointer Below bounds!"))
        }
    }

    pub fn inc_cell(&mut self, i: u8) -> Result<()> { 
        self.mem_buffer[self.mem_ptr] = if self.cell_wrapping {
            self.mem_buffer[self.mem_ptr].wrapping_add(i)
        } else {
            self.mem_buffer[self.mem_ptr]
            .checked_add(i)
            .ok_or(Error::msg("Cell Overflow!"))?
        };
        Ok(())
    }

    pub fn dec_cell(&mut self, i: u8) -> Result<()> {
        self.mem_buffer[self.mem_ptr] = if self.cell_wrapping {
            self.mem_buffer[self.mem_ptr].wrapping_sub(i)
        } else {
            self.mem_buffer[self.mem_ptr]
            .checked_sub(i)
            .ok_or(Error::msg("Cell Underflow!"))?
        };
        Ok(())
    }

    pub fn output(&self) {
        print!("{}", self.mem_buffer[self.mem_ptr] as char);
    }

    pub fn input(&mut self) {
        // This is not urgent as HackFuck does not use IO operations
        todo!("Program input not yet implemented!");
    }

    pub fn get_cell(&self) -> u8 {
        self.mem_buffer[self.mem_ptr]
    }

    pub fn get_pixels(&self) -> Vec<(u8, u8, u8)> {
        self.mem_buffer[REGISTER_BUFFER + RAM - 1..TAPE_SIZE - 1]
            .chunks(3)
            .map(|window| (window[0], window[1], window[2]))
            .collect()
    }

    pub fn update_kbd(&mut self, key: Keycode) {
        todo!("Updating keyboard in memory is not yet implemented, task is easy but tedious!");
    }
}


#[cfg(test)]
mod TapeTests {
    use super::*;

    #[test]
    fn test_inc_test() {
        let mut tape = Tape::new(true, true);
        tape.inc_cell(1).unwrap();
        assert_eq!(tape.get_cell(), 1);
        tape.inc_cell(5).unwrap();
        assert_eq!(tape.get_cell(), 6);
    }
    #[test]
    fn test_inc_wrapping() {
        let mut tape = Tape::new(true, true);
        tape.inc_cell(255).unwrap();
        tape.inc_cell(1).unwrap();
        assert_eq!(tape.get_cell(), 0);
        tape.inc_cell(255).unwrap();
        tape.inc_cell(10).unwrap();
        assert_eq!(tape.get_cell(), 9);
    }

    #[test]
    #[should_panic]
    fn test_inc_overflow() {
        let mut tape = Tape::new(false, true);
        tape.inc_cell(255).unwrap();
        tape.inc_cell(2).unwrap()
    }

    #[test]
    fn test_dec_test() {
        let mut tape = Tape::new(true, true);
        tape.inc_cell(10).unwrap();
        tape.dec_cell(1).unwrap();
        assert_eq!(tape.get_cell(), 9);
        tape.dec_cell(5).unwrap();
        assert_eq!(tape.get_cell(), 4);
    }

    #[test]
    fn test_dec_wrapping() {
        let mut tape = Tape::new(true, true);
        tape.dec_cell(1).unwrap();
        assert_eq!(tape.get_cell(), 255);
        tape.inc_cell(1).unwrap();
        assert_eq!(tape.get_cell(), 0);
        tape.dec_cell(10).unwrap();
        assert_eq!(tape.get_cell(), 246);
    }

    #[test]
    #[should_panic]
    fn test_dec_underflow() {
        let mut tape = Tape::new(false, true);
        tape.dec_cell(1).unwrap();
    }

    #[test]
    fn test_inc_ptr() {
        let mut tape = Tape::new(true, true);
        tape.inc_ptr(1).unwrap();
        assert_eq!(tape.mem_ptr, 1);
        tape.inc_ptr(5).unwrap();
        assert_eq!(tape.mem_ptr, 6);
    }

    #[test]
    #[should_panic]
    fn test_inc_ptr_overflow() {
        let mut tape = Tape::new(true, true);
        tape.inc_ptr(TAPE_SIZE + 1).unwrap();
    }

    #[test]
    fn test_dec_ptr() {
        let mut tape = Tape::new(true, true);
        tape.inc_ptr(10).unwrap();
        tape.dec_ptr(1).unwrap();
        assert_eq!(tape.mem_ptr, 9);
        tape.dec_ptr(5).unwrap();
        assert_eq!(tape.mem_ptr, 4);
    }

    #[test]
    #[should_panic]
    fn test_dec_ptr_underflow() {
        let mut tape = Tape::new(true, true);
        tape.dec_ptr(1).unwrap();
    }

    #[test]
    fn test_get_pixels() {
        let mut tape = Tape::new(true, true);
        tape.inc_ptr(REGISTER_BUFFER + RAM - 1).unwrap();
        tape.inc_cell(1).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(2).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(3).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(4).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(5).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(6).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(7).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(8).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(9).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(10).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(11).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(12).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(13).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(14).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(15).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(16).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(17).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(18).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(19).unwrap();
        tape.inc_ptr(1).unwrap();
        tape.inc_cell(20).unwrap();

        let pixels = tape.get_pixels();
        assert_eq!(pixels[0].0, 1);
        assert_eq!(pixels[0].1, 2);
        assert_eq!(pixels[0].2, 3);
        assert_eq!(pixels[1].0, 4);
        assert_eq!(pixels[1].1, 5);
        assert_eq!(pixels[1].2, 6);
        assert_eq!(pixels[2].0, 7);
        assert_eq!(pixels[2].1, 8);
        assert_eq!(pixels[2].2, 9);
        assert_eq!(pixels[3].0, 10);
        assert_eq!(pixels[3].1, 11);
        assert_eq!(pixels[3].2, 12);
        assert_eq!(pixels[4].0, 13);
        assert_eq!(pixels[4].1, 14);
        assert_eq!(pixels[4].2, 15);
        assert_eq!(pixels[5].0, 16);
        assert_eq!(pixels[5].1, 17);
        assert_eq!(pixels[5].2, 18);
        assert_eq!(pixels[6].0, 19);
        assert_eq!(pixels[6].1, 20);
    }

    #[test]
    fn test_update_keyboard() {
        todo!()
    }

    #[test]
    fn test_input() {
        todo!()
    }

    #[test]
    fn test_output() {
        todo!()
    }
}