use super::instruction::{self, Instruction};
use super::tape::Tape;

use anyhow::{bail, Result};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;

pub struct Interpreter {
    pub tape: Tape,
    instructions: Vec<Instruction>,
    sdl_context: Sdl,
    canvas: Canvas<Window>,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>) -> Interpreter {
        let sdl_context = sdl2::init().unwrap();

        let window = sdl_context
            .video()
            .unwrap()
            .window("BrainHack", 512, 256)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        Interpreter {
            tape: Tape::new(),
            instructions,
            sdl_context,
            canvas,
        }
    }

    pub fn load(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
    }

    pub fn eval(&mut self) -> Result<()> {
        let mut i = 0;
        while i < self.instructions.len() {
            match self.instructions[i] {
                Instruction::IncPtr(batch) => self.tape.inc_ptr(batch)?,
                Instruction::DecPtr(batch) => self.tape.dec_ptr(batch)?,
                Instruction::IncCell(batch, offset) => self.tape.inc_cell(batch, offset)?,
                Instruction::DecCell(batch, offset) => self.tape.dec_cell(batch, offset)?,
                Instruction::StartLoop(index) => {
                    if self.tape.get_current_cell() == 0 {
                        i = index
                    }
                }
                Instruction::EndLoop(index) => {
                    if self.tape.get_current_cell() != 0 {
                        i = index
                    }
                }
                Instruction::BreakPoint => self.tape.breakpoint(),
            }
            i += 1;
        }
        Ok(())
    }

    fn io_operations(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { keycode, .. } => {}
                    _ => {}
                }
            }

            let points = [Point::new(0, 0); 256];
            self.canvas.draw_points(points.as_slice()).unwrap();
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // sloppy FPS limit
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut i = 0;
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'event_loop: while i < self.instructions.len() {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'event_loop,
                    Event::KeyDown { keycode, .. } => self.tape.update_kbd(keycode.unwrap()),
                    _ => {}
                }
            }
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.canvas.clear();
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));

            let points = self.tape.get_pixels();
            self.canvas.draw_points(points.as_slice()).unwrap();
            self.canvas.present();
            // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // sloppy FPS limit

            match self.instructions[i] {
                Instruction::IncPtr(batch) => self.tape.inc_ptr(batch)?,
                Instruction::DecPtr(batch) => self.tape.dec_ptr(batch)?,
                Instruction::IncCell(batch, offset) => self.tape.inc_cell(batch, offset)?,
                Instruction::DecCell(batch, offset) => self.tape.dec_cell(batch, offset)?,
                Instruction::StartLoop(index) => {
                    if self.tape.get_current_cell() == 0 {
                        i = index
                    }
                }
                Instruction::EndLoop(index) => {
                    if self.tape.get_current_cell() != 0 {
                        i = index
                    }
                }
                Instruction::BreakPoint => self.tape.breakpoint(),
            }
            i += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod InterpreterTests {
    use super::*;
    use crate::interpreter::{instruction, parser::Parser};

    #[test]
    fn test_optimizations() {
        // +>       // 0 = 1
        // ++++++>  // 1 = 6
        // ->>>>>   // 2 = 255
        // -------- // 7 = 248
        // >>++++<< // 9 = 4
        // >++++++++ // 8 = 8
        // [-]      // 8 = 0
        // >>+++++-- // 10 = 3
        // >>>>>><<- // 14 = 255
        // >++++++--++++++++----------++-+++ // 15 = 6
        //
        let program = b"+>++++++>->>>>>-------->>++++<<>++++++++[-]>>+++++-->>>>>><<->++++++--++++++++----------++-+++";
        // Tests cases where optimization might change outcome but shouldn't
        // If optimization works as intended, tape will look like:
        // |  1  |  6  | 255 |  0  |  0  |  0  |  0  |  248  |  0  |  4  |  3  |  0  |  0  |  0  |  255  |  6  |

        let mut interpreter =
            Interpreter::new(Parser::from_bytes(program).unwrap().optimized_parse());

        interpreter.eval().unwrap();

        assert_eq!(
            interpreter.tape.get_slice(0, 15).unwrap(),
            &[1, 6, 255, 0, 0, 0, 0, 248, 0, 4, 3, 0, 0, 0, 255, 6]
        )
    }

    #[test]
    fn test_screen() {}
}
