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

use super::consts::*;
use super::utils::{cell_to_bin, pause};

use anyhow::{bail, Result};
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::Point;

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
        // TODO: Update the mem layout, add additional registers
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
            self.mem_buffer[REGISTER_BUFFER + RAM + SCREEN],
            self.mem_buffer[REGISTER_BUFFER + RAM + SCREEN + 1]
        );
        eprintln!("+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+     +-----+-----+\n");
        eprintln!("+----------+-------+");
        eprintln!("| Register | Value |");
        eprintln!("+----------+-------+");
        eprintln!(
            "|     A    |{: ^7}|",
            256 * self.mem_buffer[1] as u32 + self.mem_buffer[2] as u32
        );
        eprintln!("+----------+-------+");
        eprintln!(
            "|     D    |{: ^7}|",
            256 * self.mem_buffer[4] as u32 + self.mem_buffer[5] as u32
        );
        eprintln!("+----------+-------+");
        eprintln!(
            "|     M    |{: ^7}|",
            256 * self.mem_buffer[7] as u32 + self.mem_buffer[8] as u32
        );
        eprintln!("+----------+-------+");
        eprintln!("|    KBD   |{: ^7}|", "TBA"); // TODO: replace with Pressed key representation
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

    pub fn update_kbd(&mut self, keycode: Keycode) {
        let key_val: u8 = match keycode {
            Keycode::Backspace => 129,
            Keycode::Return => 128,
            Keycode::Escape => 140,
            Keycode::Space => 32,
            Keycode::Exclaim => 33,
            Keycode::Quotedbl => 34,
            Keycode::Hash => 35,
            Keycode::Dollar => 36,
            Keycode::Percent => 37,
            Keycode::Ampersand => 38,
            Keycode::Quote => 39,
            Keycode::LeftParen => 40,
            Keycode::RightParen => 41,
            Keycode::Asterisk => 42,
            Keycode::Plus => 43,
            Keycode::Comma => 44,
            Keycode::Minus => 45,
            Keycode::Period => 46,
            Keycode::Slash => 47,
            Keycode::Num0 => 48,
            Keycode::Num1 => 49,
            Keycode::Num2 => 50,
            Keycode::Num3 => 51,
            Keycode::Num4 => 52,
            Keycode::Num5 => 53,
            Keycode::Num6 => 54,
            Keycode::Num7 => 55,
            Keycode::Num8 => 56,
            Keycode::Num9 => 57,
            Keycode::Colon => 58,
            Keycode::Semicolon => 59,
            Keycode::Less => 60,
            Keycode::Equals => 61,
            Keycode::Greater => 62,
            Keycode::Question => 63,
            Keycode::At => 64,
            Keycode::LeftBracket => 91,
            Keycode::Backslash => 92,
            Keycode::RightBracket => 93,
            Keycode::Caret => 94,
            Keycode::Underscore => 95,
            Keycode::Backquote => 96,
            Keycode::A => 65,
            Keycode::B => 66,
            Keycode::C => 67,
            Keycode::D => 68,
            Keycode::E => 69,
            Keycode::F => 70,
            Keycode::G => 71,
            Keycode::H => 72,
            Keycode::I => 73,
            Keycode::J => 74,
            Keycode::K => 75,
            Keycode::L => 76,
            Keycode::M => 77,
            Keycode::N => 78,
            Keycode::O => 79,
            Keycode::P => 80,
            Keycode::Q => 81,
            Keycode::R => 82,
            Keycode::S => 83,
            Keycode::T => 84,
            Keycode::U => 85,
            Keycode::V => 86,
            Keycode::W => 87,
            Keycode::X => 88,
            Keycode::Y => 89,
            Keycode::Z => 90,
            Keycode::Delete => 127,
            Keycode::F1 => 141,
            Keycode::F2 => 142,
            Keycode::F3 => 143,
            Keycode::F4 => 144,
            Keycode::F5 => 145,
            Keycode::F6 => 146,
            Keycode::F7 => 147,
            Keycode::F8 => 148,
            Keycode::F9 => 149,
            Keycode::F10 => 150,
            Keycode::F11 => 151,
            Keycode::F12 => 152,
            Keycode::Insert => 138,
            Keycode::Home => 134,
            Keycode::PageUp => 136,
            Keycode::End => 135,
            Keycode::PageDown => 137,
            Keycode::Right => 132,
            Keycode::Left => 130,
            Keycode::Down => 133,
            Keycode::Up => 131,
            // Keycode::LCtrl => todo!(),
            // Keycode::LShift => todo!(),
            // Keycode::LAlt => todo!(),
            // Keycode::RCtrl => todo!(),
            // Keycode::RShift => todo!(),
            // Keycode::RAlt => todo!(),
            _ => 0,
        };

        self.mem_buffer[REGISTER_BUFFER + RAM + SCREEN + 1] = key_val
    }

    pub fn get_pixels(&mut self) -> Vec<Point> {
        let mut pixels = vec![];

        let mut x = 0;
        let mut y = 0;

        for chunks in
            self.mem_buffer[(REGISTER_BUFFER + RAM)..(REGISTER_BUFFER + RAM + SCREEN)].chunks(3)
        {
            let (w1, w2) = (chunks[0], chunks[1]);

            for bit in cell_to_bin(w1) {
                if bit {
                    pixels.push(Point::new(x, y))
                }
                x += 1;
            }
            for bit in cell_to_bin(w2) {
                if bit {
                    pixels.push(Point::new(x, y))
                }
                x += 1;
            }

            if x % 512 == 0 {
                y += 1;
                x = 0;
            }
        }

        pixels
    }
}
