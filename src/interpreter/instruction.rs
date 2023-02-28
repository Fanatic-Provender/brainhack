use anyhow::{bail, Result};

#[derive(Debug, Copy, Clone)]
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

#[cfg(test)]
mod instruction_tests {
    use super::*;

    #[test]
    fn test_eq() {
        let inst1 = Instruction::IncCell(1, 0);
        let inst2 = Instruction::IncCell(1, 0);
        assert!(inst1 == inst2)
    }
}
