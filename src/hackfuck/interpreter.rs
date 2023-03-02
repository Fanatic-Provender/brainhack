use super::instruction::Instruction;
use super::tape::Tape;

use anyhow::Result;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::marker::PhantomData;

pub struct IO;
pub struct PURE;

pub struct Interpreter<Type> {
    pub tape: Tape,
    instructions: Vec<Instruction>,
    sdl_context: Option<Sdl>,
    canvas: Option<Canvas<Window>>,
    type_: PhantomData<Type>,
}

impl<Type> Interpreter<Type> {
    #[allow(dead_code)]
    pub fn load(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
    }

    #[allow(dead_code)]
    pub fn eval(&mut self) -> Result<()> {
        let mut i = 0;
        while i < self.instructions.len() {
            match self.instructions[i] {
                Instruction::IncPtr(batch) => self.tape.inc_ptr(batch)?,
                Instruction::DecPtr(batch) => self.tape.dec_ptr(batch)?,
                Instruction::IncCell(batch, offset) => self.tape.inc_cell(batch, offset)?,
                Instruction::DecCell(batch, offset) => self.tape.dec_cell(batch, offset)?,
                Instruction::StartLoop(index) => {
                    if self.tape.get_cell() == 0 {
                        i = index
                    }
                }
                Instruction::EndLoop(index) => {
                    if self.tape.get_cell() != 0 {
                        i = index
                    }
                }
                Instruction::BreakPoint => self.tape.breakpoint(),
            }
            i += 1;
        }

        println!("{:?}", self.tape.get_pixels());
        self.tape.breakpoint();

        Ok(())
    }
}

impl Interpreter<PURE> {
    pub fn new(instructions: Vec<Instruction>) -> Interpreter<PURE> {
        Interpreter::<PURE> {
            tape: Tape::new(),
            instructions,
            sdl_context: None,
            canvas: None,
            type_: PhantomData::<PURE>,
        }
    }

    pub fn init_screen(self) -> Interpreter<IO> {
        let sdl_context = sdl2::init().unwrap();

        let window = sdl_context
            .video()
            .unwrap()
            .window("BrainHack", 512, 256)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Interpreter::<IO> {
            tape: self.tape,
            instructions: self.instructions,
            sdl_context: Some(sdl_context),
            canvas: Some(canvas),
            type_: PhantomData::<IO>,
        }
    }
}

impl Interpreter<IO> {
    pub fn run(&mut self) -> Result<()> {
        let mut i = 0;
        let mut writes = 1;
        let mut event_pump = self.sdl_context.as_mut().unwrap().event_pump().unwrap();
        let canvas = self.canvas.as_mut().unwrap();
        'event_loop: loop {
            if i >= self.instructions.len() {
                break 'event_loop;
            }

            if self.tape.io_write {
                writes += 1;
                self.tape.io_write = false;
            }

            if writes % 100000 == 0 {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.clear();
                canvas.set_draw_color(Color::RGB(0, 0, 0));

                let points = self.tape.get_pixels();
                canvas.draw_points(points.as_slice()).unwrap();
                canvas.present();
                // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // sloppy FPS limit

                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => break 'event_loop,
                        Event::KeyDown { keycode, .. } => self.tape.update_kbd(keycode.unwrap()),
                        _ => {}
                    }
                }
                writes = 1;
            }

            match self.instructions[i] {
                Instruction::IncPtr(batch) => self.tape.inc_ptr(batch)?,
                Instruction::DecPtr(batch) => self.tape.dec_ptr(batch)?,
                Instruction::IncCell(batch, offset) => self.tape.inc_cell(batch, offset)?,
                Instruction::DecCell(batch, offset) => self.tape.dec_cell(batch, offset)?,
                Instruction::StartLoop(index) => {
                    if self.tape.get_cell() == 0 {
                        i = index
                    }
                }
                Instruction::EndLoop(index) => {
                    if self.tape.get_cell() != 0 {
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
mod interpreter_test {
    use super::*;
    use crate::hackfuck::parser::Parser;

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
            Interpreter::new(Parser::from_bytes(program).unwrap().optimized_parse(true));

        interpreter.eval().unwrap();

        assert_eq!(
            interpreter.tape.get_slice(0, 15).unwrap(),
            &[1, 6, 255, 0, 0, 0, 0, 248, 0, 4, 3, 0, 0, 0, 255, 6]
        )
    }

    #[test]
    fn test_screen() {}
}
