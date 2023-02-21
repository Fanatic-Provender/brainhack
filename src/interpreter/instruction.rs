
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    IncPtr(usize),
    DecPtr(usize),
    IncCell(usize),
    DecCell(usize),
    StartLoop(usize),
    EndLoop(usize),
    Read,
    Write
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            b'>' => Some(Instruction::IncPtr(1)),
            b'<' => Some(Instruction::DecPtr(1)),
            b'+' => Some(Instruction::IncCell(1)),
            b'-' => Some(Instruction::DecCell(1)),
            b'[' => Some(Instruction::StartLoop(0)),
            b']' => Some(Instruction::EndLoop(0)),
            b',' => Some(Instruction::Read),
            b'.' => Some(Instruction::Write),
            _ => None
        }
    }
    pub fn update_batch(self, n: usize) -> Self {
        match self {
            Instruction::IncPtr(x) => Instruction::IncPtr(x + n),
            Instruction::DecPtr(x) => Instruction::DecPtr(x + n),
            Instruction::IncCell(x) => Instruction::IncCell(x + n),
            Instruction::DecCell(x) => Instruction::DecCell(x + n),
            _ => self
        }
    }

    pub fn update_loop(self, i: usize) -> Self {
        match self {
            Instruction::StartLoop(_) => Instruction::StartLoop(i),
            Instruction::EndLoop(_) => Instruction::EndLoop(i),
            _ => self
        }
    }

    // TODO: Find to char trait and make better
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

    #[test]
    fn test_instruction_from_byte() {
        assert_eq!(Instruction::from_byte(b'>'), Some(Instruction::IncPtr(1)));
        assert_eq!(Instruction::from_byte(b'<'), Some(Instruction::DecPtr(1)));
        assert_eq!(Instruction::from_byte(b'+'), Some(Instruction::IncCell(1)));
        assert_eq!(Instruction::from_byte(b'-'), Some(Instruction::DecCell(1)));
        assert_eq!(Instruction::from_byte(b'['), Some(Instruction::StartLoop(0)));
        assert_eq!(Instruction::from_byte(b']'), Some(Instruction::EndLoop(0)));
        assert_eq!(Instruction::from_byte(b','), Some(Instruction::Read));
        assert_eq!(Instruction::from_byte(b'.'), Some(Instruction::Write));
        assert_eq!(Instruction::from_byte(b' '), None);
    }

    #[test]
    fn test_instruction_batch_update() {
        // TODO: Write failing and edge case variants for this test too
        todo!()
    }

    #[test]
    fn test_instruction_loop_index() {
        // TODO: Write failing and edge case variants for this test too
        todo!()
    }
}