use super::instruction::Instruction;
use anyhow::{Error, Result};

use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Parser {
    instructions: Vec<Instruction>,
}

impl Parser {
    /// Create a new parser from a file
    pub fn from_file(file_path: &Path) -> Result<Self> {
        let mut f = File::open(file_path)?;
        let metadata = fs::metadata(file_path)?;
        let mut buffer = vec![0; metadata.len() as usize];
        f.read_exact(&mut buffer)?;
        Self::from_bytes(buffer.as_slice())
    }

    /// Create a new parser from a byte array
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
                b'#' => parsed_instructions.push(Instruction::BreakPoint),
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

    /// 1 to 1 execution of loaded program
    #[allow(dead_code)]
    pub fn parse(mut self) -> Vec<Instruction> {
        self.fix_loops().unwrap();
        self.instructions
    }

    /// Performs a series of optimizations on the loaded program
    #[allow(dead_code)]
    pub fn optimized_parse(mut self, debug: bool) -> Vec<Instruction> {
        self.batch_optimization();
        self.order_optimization(debug);
        self.redundancy_optimization();
        self.batch_optimization();
        // self.direct_cell_mod_optimization();
        self.fix_loops().unwrap();
        // self.bounded_loop_optimization();
        // self.fix_loops().unwrap();
        self.instructions
    }

    /// Pairs StartLoop and EndLoop instructions
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

    /// Combines consecutive instructions of the same type
    fn batch_optimization(&mut self) {
        let mut prev = Instruction::StartLoop(0);
        let mut batch = 1;

        let mut new_instructions = vec![];

        for &instruction in &self.instructions {
            // Can't do batch optimization on loops and breakpoints
            if let Instruction::StartLoop(_) | Instruction::EndLoop(_) | Instruction::BreakPoint =
                instruction
            {
                new_instructions.push(instruction);
                prev = instruction;
            } else if instruction == prev {
                batch += 1;
                new_instructions
                    .last_mut()
                    .unwrap()
                    .update_batch(batch)
                    .unwrap();
            } else {
                new_instructions.push(instruction);
                prev = instruction;
                batch = 1;
            }
        }
        self.instructions = new_instructions;
    }

    /// Combines consecutive instructions of contradictory purpose
    fn redundancy_optimization(&mut self) {
        // Goes through and checks if consecutive instructions contradict, can be done multiple times

        // if both are cell op
        // They don't have the same batch size
        // and have the same offset
        // append an instruction with the one with the larger batch subtracted from the other one
        // If they are both cell ops, since this step follows batch optimization
        // inst1 cannot be the same type as inst2
        let mut new_instructions = vec![];

        let mut i = 0;
        while i < self.instructions.len() {
            let mut inst1 = self.instructions[i];
            let mut inst2 = match self.instructions.get(i + 1) {
                Some(&inst) => inst,
                None => {
                    new_instructions.push(inst1);
                    break;
                }
            };

            if inst1.cell_op() && inst2.cell_op() {
                let (batch1, offset1) = if let Instruction::IncCell(batch, offset)
                | Instruction::DecCell(batch, offset) = inst1
                {
                    (batch, offset)
                } else {
                    unreachable!("Cannot be cell op and not IncCell or DecCell")
                };

                let (batch2, offset2) = if let Instruction::IncCell(batch, offset)
                | Instruction::DecCell(batch, offset) = inst2
                {
                    (batch, offset)
                } else {
                    unreachable!("Cannot be cell op and not IncCell or DecCell")
                };

                // Operation is not on the same cell
                if offset1 != offset2 {
                    new_instructions.push(inst1);
                } else {
                    // Current instructions are combined,
                    // cannot be used in next window
                    i += 1;
                    match batch1.cmp(&batch2) {
                        Ordering::Less => {
                            inst2.update_batch(batch2 - batch1).unwrap();
                            new_instructions.push(inst2);
                        }
                        Ordering::Greater => {
                            inst1.update_batch(batch1 - batch2).unwrap();
                            new_instructions.push(inst1);
                        }
                        Ordering::Equal => {} // THe instructions cancel out
                    }
                }
            } else if inst1.mem_op() && inst2.mem_op() {
                let batch1 = if let Instruction::IncPtr(batch) | Instruction::DecPtr(batch) = inst1
                {
                    batch
                } else {
                    unreachable!("Cannot be cell op and not IncCell or DecCell")
                };

                let batch2 = if let Instruction::IncPtr(batch) | Instruction::DecPtr(batch) = inst2
                {
                    batch
                } else {
                    unreachable!("Cannot be cell op and not IncCell or DecCell")
                };

                i += 1;
                match batch1.cmp(&batch2) {
                    Ordering::Less => {
                        #[allow(unused_must_use)]
                        inst2.update_batch(batch2 - batch1).unwrap();
                        new_instructions.push(inst2);
                    }
                    Ordering::Greater => {
                        inst1.update_batch(batch1 - batch2).unwrap();
                        new_instructions.push(inst1);
                    }
                    Ordering::Equal => {} // THe instructions cancel out
                }
            } else {
                // Add older instruction to new instructions as is since now grouping/ batching has been done
                new_instructions.push(inst1);
            }

            // Next iteration
            i += 1
        }

        self.instructions = new_instructions
    }

    /// Order instructions to decrease the number of times the pointer is moved
    fn order_optimization(&mut self, debug: bool) {
        // SHOULD BE PERFORMED BEFORE ANY OTHER OFFSETS ARE CREATED

        // change execution order to increase batching
        let mut offset = 0;
        // Every time a memory operation is encountered this will be adjusted
        // instead of actually doing the mem operation to improve runtime performance
        let mut new_instructions = vec![];

        for instruction in &self.instructions {
            match instruction {
                Instruction::IncPtr(batch) => offset += *batch as isize,
                Instruction::DecPtr(batch) => offset -= *batch as isize,
                Instruction::IncCell(batch, _) => {
                    new_instructions.push(Instruction::IncCell(*batch, offset))
                }
                Instruction::DecCell(batch, _) => {
                    new_instructions.push(Instruction::DecCell(*batch, offset))
                }
                Instruction::StartLoop(_) | Instruction::EndLoop(_) => {
                    match offset.cmp(&0) {
                        Ordering::Greater => new_instructions.push(Instruction::IncPtr(offset.unsigned_abs())),
                        Ordering::Less => new_instructions.push(Instruction::DecPtr(offset.unsigned_abs())),
                        Ordering::Equal => {}
                    }
                    new_instructions.push(*instruction);
                    offset = 0;
                }
                Instruction::BreakPoint => {
                    if debug {
                        match offset.cmp(&0) {
                            Ordering::Greater => new_instructions.push(Instruction::IncPtr(offset.unsigned_abs())),
                            Ordering::Less => new_instructions.push(Instruction::DecPtr(offset.unsigned_abs())),
                            Ordering::Equal => {}
                        }
                        new_instructions.push(*instruction);
                        offset = 0;
                    }
                }
            }
        }

        self.instructions = new_instructions;
    }

    /// Predecessor to order optimization
    #[allow(dead_code)]
    fn direct_cell_mod_optimization(&mut self) {
        let mut new_instructions = vec![];

        let mut i = 0;
        while i < self.instructions.len() {
            let inst1 = self.instructions[i];
            let mut inst2 = match self.instructions.get(i + 1) {
                Some(&inst) => inst,
                None => {
                    new_instructions.push(inst1);
                    break;
                }
            };
            let inst3 = match self.instructions.get(i + 2) {
                Some(&inst) => inst,
                None => {
                    new_instructions.push(inst1);
                    new_instructions.push(inst2);
                    break;
                }
            };

            match (inst1, inst2.cell_op(), inst3) {
                (Instruction::IncPtr(bl), true, Instruction::DecPtr(br)) => {
                    inst2.update_offset(bl as isize).unwrap();
                    new_instructions.push(inst2);
                    match bl.cmp(&br) {
                        Ordering::Greater => new_instructions.push(Instruction::IncPtr(bl - br)),
                        Ordering::Less => new_instructions.push(Instruction::DecPtr(br - bl)),
                        Ordering::Equal => {} // No additional mem operations necessary
                    }
                }
                (Instruction::DecPtr(bl), true, Instruction::IncPtr(br)) => {
                    inst2.update_offset(-(bl as isize)).unwrap();
                    new_instructions.push(inst2);
                    match bl.cmp(&br) {
                        Ordering::Greater => new_instructions.push(Instruction::DecPtr(bl - br)),
                        Ordering::Less => new_instructions.push(Instruction::IncPtr(br - bl)),
                        Ordering::Equal => {} // No additional mem operations necessary
                    }
                }
                _ => {
                    new_instructions.push(inst1);
                    i += 1; // Does not match pattern, move on to next triplet
                    continue;
                }
            }

            i += 3; // Matched pattern, instructions minimized
        }

        self.instructions = new_instructions
    }

    /// Inline bounded loops and turn them into batched cell instructions
    /// To do this modification will need to be made for the Instruction enum
    /// currently not implemented
    #[allow(dead_code)]
    fn bounded_loop_optimization(&mut self) {
        todo!()
    }
}
