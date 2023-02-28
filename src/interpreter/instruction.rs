use anyhow::{bail, Result};

/// Data structure to represent abstract brainfuck operations
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    IncPtr(usize),         // (Batch size)
    DecPtr(usize),         // (Batch size)
    IncCell(usize, isize), // (Batch size, mem_ptr offset)
    DecCell(usize, isize), // (Batch size, mem_ptr offset)
    StartLoop(usize),      // (Index of matching EndLoop)
    EndLoop(usize),        // (Index of matching StartLoop)
    // Custom instruction for debugging
    BreakPoint, 
}

impl Instruction {
    /// Updates Instruction batch
    /// 
    /// # Arguments
    /// * `batch_size` - Number of instructions being batched
    /// 
    /// # Returns
    /// Returns Err if batch of un-groupable instructions is updated
    /// 
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

    /// Updates Instruction offset, changes which relative cell the instruction operates on
    /// 
    /// # Arguments
    /// * `offset` - Offset from current mem pointer
    /// 
    /// # Returns
    /// Returns Err if instruction doesn't operate on the cell value
    /// 
    pub(super) fn update_offset(&mut self, offset: isize) -> Result<()> {
        match self {
            Instruction::IncCell(_, mem_ptr_offset) => *mem_ptr_offset = offset,
            Instruction::DecCell(_, mem_ptr_offset) => *mem_ptr_offset = offset,
            _ => bail!("Cannot update memory pointer offset of this instruction"),
        }
        Ok(())
    }

    /// Updates Jump index for loops
    /// 
    /// # Arguments
    /// * `index` - index of loop pair in instruction vector
    /// 
    /// # Returns
    /// Returns Err if instruction isn't a loop
    /// 
    pub(super) fn update_loop(&mut self, index: usize) -> Result<()> {
        match self {
            Instruction::StartLoop(i) => *i = index,
            Instruction::EndLoop(i) => *i = index,
            _ => bail!("Cannot update loop index of this instruction"),
        }
        Ok(())
    }

    /// Determine if instruction operates on cell
    pub(super) fn cell_op(&self) -> bool {
        if let &Instruction::IncCell(_, _) | &Instruction::DecCell(_, _) = self {
            true
        } else {
            false
        }
    }

    /// Determine if instruction operates on memory
    pub(super) fn mem_op(&self) -> bool {
        if let &Instruction::IncPtr(_) | &Instruction::DecPtr(_) = self {
            true
        } else {
            false
        }
    }
}

impl PartialEq for Instruction {
    /// This equality check does not consider instruction batches
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Pointer operations are always equal
            (Self::IncPtr(_), Self::IncPtr(_)) | (Self::DecPtr(_), Self::DecPtr(_)) => true,
            // Mem cell operations are equal if they are working on the same mem cell
            (Self::IncCell(_, l1), Self::IncCell(_, r1)) => l1 == r1,
            (Self::DecCell(_, l1), Self::DecCell(_, r1)) => l1 == r1,
            _ => false, // Loops and breakpoints should never be equal
        }
    }
}
