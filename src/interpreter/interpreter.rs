use super::instruction::{self, Instruction};
use super::tape::Tape;
use super::utils::pause;
use anyhow::{bail, Result};

pub struct Interpreter {
    pub tape: Tape,
    instructions: Vec<Instruction>,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>) -> Interpreter {
        Interpreter {
            tape: Tape::new(),
            instructions,
        }
    }

    pub fn load(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
    }

    pub fn run(&mut self) -> Result<()> {
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


}

#[cfg(test)]
mod InterpreterTests {
    use super::*;
    use crate::interpreter::{instruction, parser::Parser};

    #[test]
    fn test_optimizations() {
        //      q   // 0 = 1
        // ++++++>  // 1 = 6
        // ->>>>>   // 2 = 255
        // -------- // 7 = 248
        // >>++++<< // 9 = 4
        // >++++++++ // 8 = 8
        // [-]      // 8 = 0
        // >>+++++-- // 10 = 3
        // >>>>>><<- // 14 = 255
        //
        let program = b"+>++++++>->>>>>-------->>++++<<>++++++++[-]>>+++++-->>>>>><<-";
        // Tests cases where optimization might change outcome but shouldn't
        // If optimization works as intended, tape will look like:
        // |  1  |  6  | 255 |  0  |  0  |  0  |  0  |  248  |  0  |  4  |  3  |  0  |  0  |  0  |  255  |

        let mut interpreter = Interpreter::new(Parser::from_bytes(program).unwrap().optimized_parse());

        interpreter.run().unwrap();

        assert_eq!(
            interpreter.tape.get_slice(0, 14).unwrap(),
            &[1, 6, 255, 0, 0, 0, 0, 248, 0, 4, 3, 0, 0, 0, 255]
        )
    }
}
