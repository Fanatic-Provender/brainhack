use crate::prelude::*;

pub type Word = (Pos, Pos);

pub mod word {
    use super::Word;

    pub const A: Word = (0, 1);
    pub const D: Word = (3, 4);
    pub const M: Word = (6, 7);
    pub const R: Word = (9, 10);
}

pub trait Arith: Logic {
    fn add_move_word(&mut self, src: Word, dests: &[Word]) -> anyhow::Result<&mut Self> {
        let upper_dests: Vec<_> = dests.iter().map(|w| w.0).collect();
        let lower_dests: Vec<_> = dests.iter().map(|w| w.1).collect();

        self.add_move_cell(src.0, &upper_dests)?
            .add_move_cell(src.1, &lower_dests)
    }
    fn move_word(&mut self, src: Word, dests: &[Word]) -> anyhow::Result<&mut Self> {
        let dest_cells: Vec<_> = dests.iter().flat_map(|w| [w.0, w.1]).collect();

        self.clear_cell(&dest_cells)?.add_move_word(src, dests)
    }
    fn add_copy_word(&mut self, src: Word, dests: &[Word], temp: Pos) -> anyhow::Result<&mut Self> {
        let upper_dests: Vec<_> = dests.iter().map(|w| w.0).collect();
        let lower_dests: Vec<_> = dests.iter().map(|w| w.1).collect();

        self.add_copy_cell(src.0, &upper_dests, temp)?
            .add_copy_cell(src.1, &lower_dests, temp)
    }
    fn copy_word(&mut self, src: Word, dests: &[Word], temp: Pos) -> anyhow::Result<&mut Self> {
        let dest_cells: Vec<_> = dests.iter().flat_map(|w| [w.0, w.1]).collect();

        self.clear_cell(&dest_cells)?
            .add_copy_word(src, dests, temp)
    }

    fn is_nonzero_move(&mut self, word: Word, dest: Pos) -> anyhow::Result<&mut Self> {
        self.logical_or_move(word.0, word.1, dest)
    }
    fn is_nonzero(
        &mut self,
        word: Word,
        dest: Pos,
        temp_1: Pos,
        temp_2: Pos,
    ) -> anyhow::Result<&mut Self> {
        self.logical_or(word.0, word.1, dest, temp_1, temp_2)
    }
    fn is_zero_move(&mut self, word: Word, dest: Pos, temp: Pos) -> anyhow::Result<&mut Self> {
        self.is_nonzero_move(word, temp)?
            .logical_not_move(temp, dest)
    }
    fn is_zero(
        &mut self,
        word: Word,
        dest: Pos,
        temp_1: Pos,
        temp_2: Pos,
    ) -> anyhow::Result<&mut Self> {
        self.is_nonzero(word, temp_1, dest, temp_2)?
            .logical_not_move(temp_1, dest)
    }

    fn inc_word(&mut self, word: Word, temp_1: Pos, temp_2: Pos) -> anyhow::Result<&mut Self> {
        self.seek(word.1)?
            .inc_val()?
            .copy_cell(word.1, &[temp_1], temp_2)?
            .if_else_move(temp_1, temp_2, |s| Ok(s), |s| s.seek(word.0)?.inc_val())
    }
    fn dec_word(&mut self, word: Word, temp_1: Pos, temp_2: Pos) -> anyhow::Result<&mut Self> {
        self.copy_cell(word.1, &[temp_1], temp_2)?
            .if_else_move(temp_1, temp_2, |s| Ok(s), |s| s.seek(word.0)?.dec_val())?
            .seek(word.1)?
            .dec_val()
    }

    fn add_word_move(
        &mut self,
        src: Word,
        dest: Word,
        temp_1: Pos,
        temp_2: Pos,
        temp_3: Pos,
    ) -> anyhow::Result<&mut Self> {
        self.is_nonzero(src, temp_1, temp_2, temp_3)?
            .while_(temp_1, |s| {
                s.dec_word(src, temp_2, temp_3)?
                    .inc_word(dest, temp_2, temp_3)?
                    .clear_cell(&[temp_1])?
                    .is_nonzero(src, temp_1, temp_2, temp_3)
            })
    }
}
impl<T: Logic> Arith for T {}

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

    #[test]
    fn is_nonzero_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_nonzero_move((0, 1), 2)?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 41], 0, &[0, 0, 1], 0);
        test::compare_tape(coder.writer(), &[255, 0], 0, &[0, 0, 1], 0);
        test::compare_tape(coder.writer(), &[31, 41], 0, &[0, 0, 2], 0);
        Ok(())
    }

    #[test]
    fn is_nonzero() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_nonzero((0, 1), 2, 3, 4)?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 41], 0, &[0, 41, 1, 0, 0], 0);
        test::compare_tape(coder.writer(), &[255, 0], 0, &[255, 0, 1, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 41], 0, &[31, 41, 2, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn is_zero_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_zero_move((0, 1), 2, 3)?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 1, 0], 0);
        test::compare_tape(coder.writer(), &[0, 41], 0, &[0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[255, 0], 0, &[0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 41], 0, &[0, 0, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn is_zero() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_zero((0, 1), 2, 3, 4)?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 1, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 41], 0, &[0, 41, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[255, 0], 0, &[255, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 41], 0, &[31, 41, 0, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn inc_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.inc_word((0, 1), 2, 3)?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 41], 0, &[0, 42, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 41], 0, &[31, 42, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 255], 0, &[32, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[255, 255], 0, &[0, 0, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn dec_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.dec_word((0, 1), 2, 3)?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[255, 255, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 41], 0, &[0, 40, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 0], 0, &[30, 255, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 41], 0, &[31, 40, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn add_word_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.add_word_move((0, 1), (2, 3), 4, 5, 6)?.seek(0)?;

        eprintln!("{}", std::str::from_utf8(coder.writer())?);

        test::compare_tape(coder.writer(), &[1, 2, 3, 4], 0, &[0, 0, 4, 6, 0, 0, 0], 0);
        test::compare_tape(
            coder.writer(),
            &[1, 100, 3, 200],
            0,
            &[0, 0, 5, 44, 0, 0, 0],
            0,
        );
        // TODO: more tests for overflowing (need another interpreter)
        Ok(())
    }
}
