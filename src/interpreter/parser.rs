use super::instruction::Instruction;
use anyhow::{Error, Result};

use std::collections::{HashMap, VecDeque};
use std::cmp::Ordering;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Parser {
    instructions: Vec<Instruction>,
}

impl Parser {
    pub fn from_file(file_path: &Path) -> Result<Self> {
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

    pub fn parse(mut self) -> Vec<Instruction> {
        self.fix_loops();
        self.instructions
    }

    pub fn optimized_parse(mut self) -> Vec<Instruction> {
        self.batch_optimization();
        self.redundancy_optimization();
        // println!("\n{:?}", self.instructions);
        self.direct_cell_mod_optimization();
        // println!("\n{:?}", self.instructions);
        self.fix_loops();
        // println!("\n{:?}", self.instructions);
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
            // Can't do batch optimization on loops and breakpoints
            if let Instruction::StartLoop(_) | Instruction::EndLoop(_) | Instruction::BreakPoint =
                instruction
            {
                new_instructions.push(instruction);
                prev = instruction;
            } else if instruction == prev {
                batch += 1;
                new_instructions.last_mut().unwrap().update_batch(batch);
            } else {
                new_instructions.push(instruction);
                prev = instruction;
                batch = 1;
            }
        }
        self.instructions = new_instructions;
    }

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
                            inst2.update_batch(batch2 - batch1);
                            new_instructions.push(inst2);
                        }
                        Ordering::Greater => {
                            inst1.update_batch(batch1 - batch2);
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
                        inst2.update_batch(batch2 - batch1);
                        new_instructions.push(inst2);
                    }
                    Ordering::Greater => {
                        inst1.update_batch(batch1 - batch2);
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
                    inst2.update_offset(bl as isize);
                    new_instructions.push(inst2);
                    match bl.cmp(&br) {
                        Ordering::Greater => new_instructions.push(Instruction::IncPtr(bl - br)),
                        Ordering::Less => new_instructions.push(Instruction::DecPtr(br - bl)),
                        Ordering::Equal => {} // No additional mem operations necessary
                    }
                }
                (Instruction::DecPtr(bl), true, Instruction::IncPtr(br)) => {
                    inst2.update_offset(-(bl as isize));
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

    fn order_optimization(&mut self) {
        // change execution order to increase batching
        todo!()
    }

    fn bounded_loop_optimization(&mut self) {
        // Inline bounded loops and turn them into batched cell instructions
        // To do this modification will need to be made for
        todo!()
    }

    fn dead_code_optimization(&mut self) {
        // Removes consecutive open and closed brackets
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_test() {
        let inst1 = Instruction::DecPtr(10);
        let mut inst2 = Instruction::IncCell(5, 0);
        let inst3 = Instruction::IncPtr(5);
        if let (Instruction::DecPtr(bl), Instruction::IncCell(b, _), Instruction::IncPtr(br)) =
            (inst1, inst2, inst3)
        {
            eprintln!("Found case");
            inst2.update_offset(-(bl as isize));
            println!("{inst2:?}");
        }
    }
}
