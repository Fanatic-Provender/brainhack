use anyhow::{bail, Result};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Instruction {
    IncPtr(usize),         // Batch size
    DecPtr(usize),         // Batch size
    IncCell(usize, isize), // Batch size, mem_ptr offset
    DecCell(usize, isize), // Batch size, mem_ptr offset
    StartLoop(usize),      // Index of matching EndLoop
    EndLoop(usize),        // Index of matching StartLoop
    BreakPoint,
}

impl Instruction {
    pub fn to_byte(self) -> u8 {
        match self {
            Instruction::IncPtr(_) => b'>',
            Instruction::DecPtr(_) => b'<',
            Instruction::IncCell(_, _) => b'+',
            Instruction::DecCell(_, _) => b'-',
            Instruction::StartLoop(_) => b'[',
            Instruction::EndLoop(_) => b']',
            Instruction::BreakPoint => b'*',
        }
    }

    pub(super) fn update_batch(&mut self, batch_size: usize) -> Result<()> {
        match self {
            Instruction::IncPtr(batch) => *batch = batch_size,
            Instruction::DecPtr(batch) => *batch = batch_size,
            Instruction::IncCell(batch, _) => *batch = batch_size,
            Instruction::DecCell(batch, _) => *batch = batch_size,
            _ => bail!("Cannot update batch size of this instruction"),
        }
        Ok(())
    }

    pub(super) fn update_offset(&mut self, offset: isize) -> Result<()> {
        match self {
            Instruction::IncCell(_, mem_ptr_offset) => *mem_ptr_offset = offset,
            Instruction::DecCell(_, mem_ptr_offset) => *mem_ptr_offset = offset,
            _ => bail!("Cannot update memory pointer offset of this instruction"),
        }
        Ok(())
    }

    pub(super) fn update_loop(&mut self, index: usize) -> Result<()> {
        match self {
            Instruction::StartLoop(i) => *i = index,
            Instruction::EndLoop(i) => *i = index,
            _ => bail!("Cannot update loop index of this instruction"),
        }
        Ok(())
    }

    pub(super) fn cell_op(&self) -> bool {
        if let &Instruction::IncCell(_, _) | &Instruction::DecCell(_, _) = self {
            true
        } else {
            false
        }
    }

    pub(super) fn mem_op(&self) -> bool {
        if let &Instruction::IncPtr(_) | &Instruction::DecPtr(_) = self {
            true
        } else {
            false
        }
    }
}
