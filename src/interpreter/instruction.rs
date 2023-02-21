
pub enum Instruction {
    IncPtr(usize),
    DecPtr(usize),
    IncCell(u8),
    DecCell(u8),
    StartLoop(usize),
    EndLoop(usize),
    Read,
    Write
}

impl Instruction {
    // TODO: Find to char trait
    fn into_char(self) -> char {
        match self {
            Instruction::IncPtr(_) => '>',
            Instruction::DecPtr(_) => '<',
            Instruction::IncCell(_) => '+',
            Instruction::DecCell(_) => '-',
            Instruction::StartLoop(_) => '[',
            Instruction::EndLoop(_) => ']',
            Instruction::Read => ',',
            Instruction::Write => '.'
        }
    }
}

#[cfg(test)]
mod InstructionTests{
    use super::*;

    #[test]
    fn test_instruction_to_char() {
        assert_eq!(Instruction::IncPtr(1).into_char(), '>');
        assert_eq!(Instruction::DecPtr(1).into_char(), '<');
        assert_eq!(Instruction::IncCell(1).into_char(), '+');
        assert_eq!(Instruction::DecCell(1).into_char(), '-');
        assert_eq!(Instruction::StartLoop(1).into_char(), '[');
        assert_eq!(Instruction::EndLoop(1).into_char(), ']');
        assert_eq!(Instruction::Read.into_char(), ',');
        assert_eq!(Instruction::Write.into_char(), '.');
    }
}