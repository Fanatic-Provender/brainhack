use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;

use std::collections::{VecDeque, HashMap};
use std::sync::Mutex;
use std::time::Duration;
use sdl2::Sdl;
use sdl2::video::Window;

use super::instruction::Instruction;
use super::tape::Tape;
use super::cell_to_bin;


struct Brainfuck {
    tape: Tape,
    instructions: Vec<Instruction>,
    sdl_context: Sdl,
    canvas: Canvas<Window>
}

impl Brainfuck {
    fn from_string(instructions: String) -> Self {
        let mut parsed_instructions: Vec<Instruction> = vec![];

        let mut loop_stack = VecDeque::new();
        let mut loop_map = HashMap::new();
        
        let mut prev_inst = b' ';
        let mut count = 1;

        // Parse instructions
        // WIP: Instruction grouping
        // for inst in instructions.bytes() {
        //     if inst == b'>' || inst == b'<' || inst == b'+' || inst == b'-' {
        //         if inst == prev_inst {
        //             count += 1;
        //         } else {
        //             if count > 1 {
        //                 parsed_instructions.push(Instruction::from_byte(prev_inst).unwrap().update_batch(count));
        //                 count = 1;
        //             }
        //             parsed_instructions.push(Instruction::from_byte(inst).unwrap());
        //             prev_inst = inst;
        //         }
        //     }
        // }

        // Non-batching parsing
        for (i, inst) in instructions.bytes().enumerate() {
            match inst {
                b'>' => parsed_instructions.push(Instruction::IncPtr(1)),
                b'<' => parsed_instructions.push(Instruction::DecPtr(1)),
                b'+' => parsed_instructions.push(Instruction::IncCell(1)),
                b'-' => parsed_instructions.push(Instruction::DecCell(1)),
                b'.' => parsed_instructions.push(Instruction::Read),
                b',' => parsed_instructions.push(Instruction::Write),
                b'[' => {
                    loop_stack.push_back(i);
                    parsed_instructions.push(Instruction::StartLoop(usize::MAX));
                },
                b']' => {
                    let start = loop_stack.pop_back().unwrap();
                    loop_map.insert(start, i);
                    parsed_instructions.push(Instruction::EndLoop(usize::MAX));
                },
                _ => ()
            }
        }

        // Updates indexes for loops
        for (start, end) in loop_map {
            parsed_instructions[start] = parsed_instructions[start].update_loop(end);
            parsed_instructions[end] = parsed_instructions[end].update_loop(start);
        }


        let sdl_context = sdl2::init().unwrap();
        let canvas = sdl_context
                .video()
                .unwrap()
                .window("HackFuck", 512, 256)
                .position_centered()
                .build()
                .unwrap()
                .into_canvas()
                .build()
                .unwrap();

        Self {
            tape: Tape::new(true, true),
            instructions: parsed_instructions,
            sdl_context: sdl_context,
            canvas: canvas
        }
        
    }

    fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        let mut points = vec![];

        for (i, (word1, word2, _)) in self.tape.get_pixels().into_iter().enumerate() {
            let x = i / 16;
            let y = i / 256;

            let word1_bin = cell_to_bin(word1);
            let word2_bin = cell_to_bin(word2);

            for (j, bit) in word1_bin.into_iter().rev().enumerate() {
                if bit {
                    points.push(Point::new((x + j) as i32, y as i32));
                }
            }

            for (j, bit) in word2_bin.into_iter().rev().enumerate() {
                if bit {
                    points.push(Point::new((x + j + 8) as i32, y as i32));
                }
            }
        }

        self.canvas.draw_points(points.as_slice()).unwrap();
        self.canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // sloppy FPS limit

    }

    fn eval(&mut self) {
        // TODO: Possibly move keypress/ display updating into separate thread and join when instructions are done

        let mut i = 0;
        while i < self.instructions.len() {
            // Update the display
            self.render();

            // Update the keypress register, if any
            let mut event_pump = self.sdl_context.event_pump().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown { keycode, ..} => self.tape.update_kbd(keycode.unwrap()),
                    _ => {}
                }
            }

            // Execute the Instruction
            let mut jumped = false;
            match self.instructions[i] {
                Instruction::IncCell(n) => self.tape.inc_cell(n as u8).unwrap(),
                Instruction::DecCell(n) => self.tape.dec_cell(n as u8).unwrap(),
                Instruction::IncPtr(n) => self.tape.inc_ptr(n).unwrap(),
                Instruction::DecPtr(n) => self.tape.dec_ptr(n).unwrap(),
                Instruction::Write => self.tape.output(),
                Instruction::Read => self.tape.input(),
                Instruction::StartLoop(index) => {
                    if self.tape.get_cell() == 0 {
                        i = index;
                        jumped = true;
                    }
                },
                Instruction::EndLoop(index) => {
                    if self.tape.get_cell() != 0 {
                        i = index;
                        jumped = true;
                    }
                }
            }
            if !jumped { i += 1 }
        }
    }
}
