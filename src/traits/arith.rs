use crate::traits::{branch::Branch, seek::Pos};

pub type Word = (Pos, Pos);

pub mod word {
    use super::Word;

    pub const A: Word = (0, 1);
    pub const D: Word = (3, 4);
    pub const M: Word = (6, 7);
    pub const R: Word = (9, 10);
}

pub trait Arith: Branch {
    fn add_move_word(&mut self, src: Word, dests: &[Word]) -> anyhow::Result<&mut Self> {
        let upper_dests: Vec<_> = dests.iter().map(|w| w.0).collect();
        let lower_dests: Vec<_> = dests.iter().map(|w| w.1).collect();

        self.add_move_cell(src.0, &upper_dests)?
            .add_move_cell(src.1, &lower_dests)
    }
    fn move_word(&mut self, src: Word, dests: &[Word]) -> anyhow::Result<&mut Self> {
        let dest_cells: Vec<_> = dests.iter().map(|w| [w.0, w.1]).flatten().collect();

        self.clear_cell(&dest_cells)?.add_move_word(src, dests)
    }
    fn add_copy_word(&mut self, src: Word, dests: &[Word], temp: Pos) -> anyhow::Result<&mut Self> {
        let upper_dests: Vec<_> = dests.iter().map(|w| w.0).collect();
        let lower_dests: Vec<_> = dests.iter().map(|w| w.1).collect();

        self.add_copy_cell(src.0, &upper_dests, temp)?
            .add_copy_cell(src.1, &lower_dests, temp)
    }
    fn copy_word(&mut self, src: Word, dests: &[Word], temp: Pos) -> anyhow::Result<&mut Self> {
        let dest_cells: Vec<_> = dests.iter().map(|w| [w.0, w.1]).flatten().collect();

        self.clear_cell(&dest_cells)?
            .add_copy_word(src, dests, temp)
    }
}
impl<T: Branch> Arith for T {}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            coder::Coder,
            test,
            traits::seek::{pos, Seek},
        },
    };

    #[test]
    fn add_move_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .add_move_word(word::R, &[word::A, word::D, word::M])?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            0,
            &[6, 6, 4, 4, 10, 9, 5, 11, 5, 0, 0],
            0,
        );
        Ok(())
    }

    #[test]
    fn move_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .move_word(word::R, &[word::A, word::D, word::M])?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            0,
            &[3, 5, 4, 3, 5, 9, 3, 5, 5, 0, 0],
            0,
        );
        Ok(())
    }

    #[test]
    fn add_copy_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .add_copy_word(word::R, &[word::A, word::D, word::M], pos::T0)?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[3, 1, 0, 1, 5, 9, 2, 6, 5, 3, 5],
            0,
            &[6, 6, 0, 4, 10, 9, 5, 11, 5, 3, 5],
            0,
        );
        Ok(())
    }

    #[test]
    fn copy_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .copy_word(word::R, &[word::A, word::D, word::M], pos::T0)?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[3, 1, 0, 1, 5, 9, 2, 6, 5, 3, 5],
            0,
            &[3, 5, 0, 3, 5, 9, 3, 5, 5, 3, 5],
            0,
        );
        Ok(())
    }
}
