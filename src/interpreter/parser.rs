use super::instruction::Instruction;
use anyhow::{Error, Result};
use std::{
    collections::{HashMap, VecDeque},
    hash,
};

use std::fs;
use std::fs::File;
use std::io::Read;

pub struct Parser {
    instructions: Vec<Instruction>,
}

impl Parser {
    pub fn from_file(file_path: &String) -> Result<Self> {
        let mut f = File::open(&file_path)?;
        let metadata = fs::metadata(&file_path)?;
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer)?;
        Self::from_bytes(buffer.as_slice())
    }

    pub fn from_bytes(instructions: &[u8]) -> Result<Self> {
        let mut parsed_instructions: Vec<Instruction> = vec![];
        let mut loop_stack = VecDeque::new();
        let mut loop_map = HashMap::new();

        for inst in instructions {
            match inst {
                b'>' => parsed_instructions.push(Instruction::IncPtr(1)),
                b'<' => parsed_instructions.push(Instruction::DecPtr(1)),
                b'+' => parsed_instructions.push(Instruction::IncCell(1, 0)),
                b'-' => parsed_instructions.push(Instruction::DecCell(1, 0)),
                b'[' => {
                    loop_stack.push_back(parsed_instructions.len());
                    parsed_instructions.push(Instruction::StartLoop(usize::MAX));
                }
                b']' => {
                    let start = loop_stack.pop_back().unwrap();
                    loop_map.insert(start, parsed_instructions.len());
                    parsed_instructions.push(Instruction::EndLoop(usize::MAX));
                }
                b'*' => parsed_instructions.push(Instruction::BreakPoint),
                _ => {}
            }
        }

        if !loop_stack.is_empty() {
            return Err(Error::msg("Found unclosed loop"));
        }

        Ok(Self {
            instructions: parsed_instructions,
        })
    }


    pub fn parse(mut self) -> Vec<Instruction> {
        self.fix_loops();
        self.instructions
    }
    pub fn optimized_parse(mut self) -> Vec<Instruction> {
        self.batch_optimization();
        self.redundancy_optimization();
        self.fix_loops();
        self.instructions
    }

    fn fix_loops(&mut self) -> Result<()> {
        let mut loop_stack = VecDeque::new();
        let mut loop_map = HashMap::new();

        for (i, instruction) in self.instructions.iter().enumerate() {
            if let Instruction::StartLoop(_) = instruction {
                loop_stack.push_back(i);
            } else if let Instruction::EndLoop(_) = instruction {
                let start_index = loop_stack.pop_back().unwrap();
                loop_map.insert(start_index, i);
            }
        }

        for (start, end) in loop_map {
            self.instructions[start].update_loop(end)?;
            self.instructions[end].update_loop(start)?;
        }
        Ok(())
    }

    fn batch_optimization(&mut self) {
        let mut prev = Instruction::StartLoop(0);
        let mut batch = 1;

        let mut new_instructions = vec![];

        for &instruction in &self.instructions {
            if let Instruction::StartLoop(_) | Instruction::EndLoop(_) | Instruction::BreakPoint =
                instruction
            {
                new_instructions.push(instruction);
                prev = instruction;
                continue;
            }
            if instruction == prev {
                batch += 1;
            } else if batch > 1 {
                prev.update_batch(batch);
                new_instructions.push(prev);
                new_instructions.push(instruction);
                prev = instruction;
                batch = 1;
            } else {
                new_instructions.push(instruction);
            }
        }

        self.instructions = new_instructions;
    }

    /// Removes instructions that undo each other, must be performed after batch optimization
    fn redundancy_optimization(&mut self) {
        let mut new_instruction = vec![];

        let mut i = 1;
        while i < self.instructions.len() {
            let (mut inst1, mut inst2) = (self.instructions[i - 1], self.instructions[i]);

            if inst1.cell_op() && inst2.cell_op() {
                let (batch1, offset1) = if let Instruction::IncCell(batch, offset) | Instruction::DecCell(batch, offset) = inst1 {
                    (batch, offset)
                } else {
                    i += 1;
                    continue;
                };

                let (batch2, offset2) = if let Instruction::IncCell(batch, offset) | Instruction::DecCell(batch, offset) = inst1 {
                    (batch, offset)
                } else {
                    i += 1;
                    continue;
                };

                if offset1 != offset2 || batch1 == batch2 {
                    i += 1;
                    continue;
                }

                let inst = if batch1 > batch2 {
                    inst1.update_batch(batch1 - batch2);
                    inst1
                } else {
                    inst2.update_batch(batch2 - batch1);
                    inst2
                };

                new_instruction.push(inst);
                i += 2
            } else if inst1.mem_op() && inst2.mem_op() {
                let batch1 = if let Instruction::IncPtr(batch) | Instruction::DecPtr(batch) = inst1
                {
                    batch
                } else {
                    i += 1;
                    continue;
                };

                let batch2 = if let Instruction::IncPtr(batch) | Instruction::DecPtr(batch) = inst1
                {
                    batch
                } else {
                    i += 1;
                    continue;
                };

                if batch1 == batch2 {
                    i += 1;
                    continue;
                }

                let inst = if batch1 > batch2 {
                    inst1.update_batch(batch1 - batch2);
                    inst1
                } else {
                    inst2.update_batch(batch2 - batch1);
                    inst2
                };

                new_instruction.push(inst);
                i += 2
            }
        }

        self.instructions = new_instruction
    }

    fn simple_loop_optimization(self) -> Self {
        todo!()
    }

    fn complex_loop_optimization(self) -> Self {
        todo!()
    }

    fn movement_postponing_optimization(self) -> Self {
        todo!()
    }
}
