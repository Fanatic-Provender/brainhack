use crate::prelude::*;

pub type Word = (Pos, Pos);

pub mod word {
    use super::{pos::*, Word};

    pub const A: Word = (AU, AL);
    pub const D: Word = (DU, DL);
    pub const M: Word = (MU, ML);
    pub const P: Word = (PU, PL);
    pub const Q: Word = (QU, QL);
    pub const R: Word = (RU, RL);
    pub const F: Word = (FU, FL);
    pub const V: Word = (VU, VL);
    pub const W: Word = (WU, WL);
}

pub trait Arith: Logic {
    fn move_word(&mut self, src: Word, dests: &[Word]) -> anyhow::Result<&mut Self> {
        let upper_dests: Vec<_> = dests.iter().map(|w| w.0).collect();
        let lower_dests: Vec<_> = dests.iter().map(|w| w.1).collect();

        self.move_cell(src.0, &upper_dests)?
            .move_cell(src.1, &lower_dests)
    }
    fn copy_word(&mut self, src: Word, dests: &[Word], temp: Pos) -> anyhow::Result<&mut Self> {
        let upper_dests: Vec<_> = dests.iter().map(|w| w.0).collect();
        let lower_dests: Vec<_> = dests.iter().map(|w| w.1).collect();

        self.copy_cell(src.0, &upper_dests, temp)?
            .copy_cell(src.1, &lower_dests, temp)
    }
    fn set_word(&mut self, word: Word, value: u16) -> anyhow::Result<&mut Self> {
        let [upper, lower] = value.to_be_bytes();
        self.seek(word.0)?
            .set_val(upper)?
            .seek(word.1)?
            .set_val(lower)
    }

    fn is_nonzero_move(&mut self, word: Word, dest: Pos) -> anyhow::Result<&mut Self> {
        self.logical_or_move(word.0, word.1, dest)
    }
    fn is_nonzero(&mut self, word: Word, dest: Pos, temp: [Pos; 2]) -> anyhow::Result<&mut Self> {
        self.logical_or(word.0, word.1, dest, [temp[0], temp[1]])
    }
    fn is_zero_move(&mut self, word: Word, dest: Pos, temp: Pos) -> anyhow::Result<&mut Self> {
        self.is_nonzero_move(word, temp)?
            .logical_not_move(temp, dest)
    }
    fn is_zero(&mut self, word: Word, dest: Pos, temp: [Pos; 2]) -> anyhow::Result<&mut Self> {
        self.is_nonzero(word, temp[0], [dest, temp[1]])?
            .logical_not_move(temp[0], dest)
    }

    fn is_le_zero_move(
        &mut self,
        word: Word,
        dest: Pos,
        temp: [Pos; 4],
    ) -> anyhow::Result<&mut Self> {
        self.copy_word(word, &[(temp[0], temp[1])], temp[2])?
            .is_zero_move((temp[0], temp[1]), temp[2], temp[3])?
            .if_else_move(
                temp[2],
                temp[3],
                |s| s.seek(dest)?.inc_val()?.clear_cell(&[word.0, word.1]),
                |s| s.is_lt_zero_move(word, dest, [temp[0], temp[1]]),
            )
    }
    fn is_le_zero(&mut self, word: Word, dest: Pos, temp: [Pos; 6]) -> anyhow::Result<&mut Self> {
        self.copy_word(word, &[(temp[0], temp[1])], dest)?
            .is_le_zero_move(
                (temp[0], temp[1]),
                dest,
                [temp[2], temp[3], temp[4], temp[5]],
            )
    }

    fn is_ge_zero_move(
        &mut self,
        word: Word,
        dest: Pos,
        temp: [Pos; 2],
    ) -> anyhow::Result<&mut Self> {
        self.copy_cell(word.0, &[temp[0]], temp[1])?
            .if_else_move(
                temp[0],
                temp[1],
                |s| {
                    s.clear_cell(&[word.1])?
                        .copy_cell(word.0, &[word.1], temp[0])?
                        .while_cond(
                            temp[0],
                            |s| {
                                s.clear_cell(&[temp[0]])?
                                    .logical_and(word.0, word.1, temp[0], temp[1])
                            },
                            |s| s.seek(word.0)?.inc_val()?.seek(word.1)?.dec_val(),
                        )?
                        .move_cell(word.0, &[dest])
                },
                |s| s.seek(dest)?.inc_val(),
            )?
            .clear_cell(&[word.1])
    }
    fn is_ge_zero(&mut self, word: Word, dest: Pos, temp: [Pos; 4]) -> anyhow::Result<&mut Self> {
        self.copy_word(word, &[(temp[0], temp[1])], dest)?
            .is_ge_zero_move((temp[0], temp[1]), dest, [temp[2], temp[3]])
    }

    fn is_lt_zero_move(
        &mut self,
        word: Word,
        dest: Pos,
        temp: [Pos; 2],
    ) -> anyhow::Result<&mut Self> {
        self.is_ge_zero_move(word, temp[0], [dest, temp[1]])?
            .logical_not_move(temp[0], dest)
    }
    fn is_lt_zero(&mut self, word: Word, dest: Pos, temp: [Pos; 4]) -> anyhow::Result<&mut Self> {
        self.copy_word(word, &[(temp[0], temp[1])], dest)?
            .is_lt_zero_move((temp[0], temp[1]), dest, [temp[2], temp[3]])
    }

    fn is_gt_zero_move(
        &mut self,
        word: Word,
        dest: Pos,
        temp: [Pos; 4],
    ) -> anyhow::Result<&mut Self> {
        self.is_le_zero_move(word, temp[0], [dest, temp[1], temp[2], temp[3]])?
            .logical_not_move(temp[0], dest)
    }
    fn is_gt_zero(&mut self, word: Word, dest: Pos, temp: [Pos; 6]) -> anyhow::Result<&mut Self> {
        self.copy_word(word, &[(temp[0], temp[1])], dest)?
            .is_gt_zero_move(
                (temp[0], temp[1]),
                dest,
                [temp[2], temp[3], temp[4], temp[5]],
            )
    }

    fn inc_word(&mut self, word: Word, temp: [Pos; 2]) -> anyhow::Result<&mut Self> {
        self.seek(word.1)?
            .inc_val()?
            .copy_cell(word.1, &[temp[0]], temp[1])?
            .if_else_move(temp[0], temp[1], |s| Ok(s), |s| s.seek(word.0)?.inc_val())
    }
    fn dec_word(&mut self, word: Word, temp: [Pos; 2]) -> anyhow::Result<&mut Self> {
        self.copy_cell(word.1, &[temp[0]], temp[1])?
            .if_else_move(temp[0], temp[1], |s| Ok(s), |s| s.seek(word.0)?.dec_val())?
            .seek(word.1)?
            .dec_val()
    }

    fn add_word_move(
        &mut self,
        src: Word,
        dest: Word,
        temp: [Pos; 3],
    ) -> anyhow::Result<&mut Self> {
        self.while_cond(
            temp[0],
            |s| {
                s.clear_cell(&[temp[0]])?
                    .is_nonzero(src, temp[0], [temp[1], temp[2]])
            },
            |s| {
                s.dec_word(src, [temp[1], temp[2]])?
                    .inc_word(dest, [temp[1], temp[2]])
            },
        )
    }
    fn add_word(&mut self, src: Word, dest: Word, temp: [Pos; 5]) -> anyhow::Result<&mut Self> {
        self.copy_word(src, &[(temp[0], temp[1])], temp[2])?
            .add_word_move((temp[0], temp[1]), dest, [temp[2], temp[3], temp[4]])
    }
    fn sub_word_move(
        &mut self,
        src: Word,
        dest: Word,
        temp: [Pos; 3],
    ) -> anyhow::Result<&mut Self> {
        self.while_cond(
            temp[0],
            |s| {
                s.clear_cell(&[temp[0]])?
                    .is_nonzero(src, temp[0], [temp[1], temp[2]])
            },
            |s| {
                s.dec_word(src, [temp[1], temp[2]])?
                    .dec_word(dest, [temp[1], temp[2]])
            },
        )
    }
    fn sub_word(&mut self, src: Word, dest: Word, temp: [Pos; 5]) -> anyhow::Result<&mut Self> {
        self.copy_word(src, &[(temp[0], temp[1])], temp[2])?
            .sub_word_move((temp[0], temp[1]), dest, [temp[2], temp[3], temp[4]])
    }
}
impl<T: Logic> Arith for T {}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{coder::Coder, test, traits::seek::Seek},
    };

    #[test]
    fn move_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .move_word((10, 11), &[(1, 2), (4, 5), (7, 8)])?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[0, 3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            0,
            &[0, 6, 6, 4, 4, 10, 9, 5, 11, 5, 0, 0],
            0,
        );
        Ok(())
    }

    #[test]
    fn copy_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .copy_word((10, 11), &[(1, 2), (4, 5), (7, 8)], 0)?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[0, 3, 1, 0, 1, 5, 9, 2, 6, 5, 3, 5],
            0,
            &[0, 6, 6, 0, 4, 10, 9, 5, 11, 5, 3, 5],
            0,
        );
        Ok(())
    }

    #[test]
    fn set_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .set_word((0, 2), 31 * 256 + 41)?
            .set_word((1, 4), 59 * 256 + 26)?
            .set_word((3, 5), 53 * 256 + 59)?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[27, 18, 28, 18, 28, 46],
            0,
            &[31, 59, 41, 53, 26, 59],
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
        coder.is_nonzero((0, 1), 2, [3, 4])?.seek(0)?;

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
        coder.is_zero((0, 1), 2, [3, 4])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 1, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 41], 0, &[0, 41, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[255, 0], 0, &[255, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 41], 0, &[31, 41, 0, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn is_le_zero_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_le_zero_move((0, 1), 2, [3, 4, 5, 6])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 1, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 5], 0, &[0, 0, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[12, 34], 0, &[0, 0, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[200, 100], 0, &[0, 0, 1, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[128, 0], 0, &[0, 0, 1, 0, 0, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn is_le_zero() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_le_zero((0, 1), 2, [3, 4, 5, 6, 7, 8])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 1, 0, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 5], 0, &[0, 5, 0, 0, 0, 0, 0, 0, 0], 0);
        test::compare_tape(
            coder.writer(),
            &[12, 34],
            0,
            &[12, 34, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[200, 100],
            0,
            &[200, 100, 1, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[128, 0],
            0,
            &[128, 0, 1, 0, 0, 0, 0, 0, 0],
            0,
        );
        Ok(())
    }

    #[test]
    fn is_ge_zero_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_ge_zero_move((0, 1), 2, [3, 4])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 1, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 5], 0, &[0, 0, 1, 0, 0], 0);
        test::compare_tape(coder.writer(), &[12, 34], 0, &[0, 0, 24, 0, 0], 0);
        test::compare_tape(coder.writer(), &[200, 100], 0, &[0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[128, 0], 0, &[0, 0, 0, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn is_ge_zero() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_ge_zero((0, 1), 2, [3, 4, 5, 6])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 1, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 5], 0, &[0, 5, 1, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[12, 34], 0, &[12, 34, 24, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[200, 6], 0, &[200, 6, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[128, 0], 0, &[128, 0, 0, 0, 0, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn is_lt_zero_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_lt_zero_move((0, 1), 2, [3, 4])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 5], 0, &[0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[12, 34], 0, &[0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[200, 100], 0, &[0, 0, 1, 0, 0], 0);
        test::compare_tape(coder.writer(), &[128, 0], 0, &[0, 0, 1, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn is_lt_zero() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_lt_zero((0, 1), 2, [3, 4, 5, 6])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 5], 0, &[0, 5, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[12, 34], 0, &[12, 34, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[200, 6], 0, &[200, 6, 1, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[128, 0], 0, &[128, 0, 1, 0, 0, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn is_gt_zero_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_gt_zero_move((0, 1), 2, [3, 4, 5, 6])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 5], 0, &[0, 0, 1, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[12, 34], 0, &[0, 0, 1, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[200, 100], 0, &[0, 0, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[128, 0], 0, &[0, 0, 0, 0, 0, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn is_gt_zero() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.is_gt_zero((0, 1), 2, [3, 4, 5, 6, 7, 8])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 0, 0, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 5], 0, &[0, 5, 1, 0, 0, 0, 0, 0, 0], 0);
        test::compare_tape(
            coder.writer(),
            &[12, 34],
            0,
            &[12, 34, 1, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[200, 100],
            0,
            &[200, 100, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[128, 0],
            0,
            &[128, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        Ok(())
    }

    #[test]
    fn inc_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.inc_word((0, 1), [2, 3])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 41], 0, &[0, 42, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 41], 0, &[31, 42, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 255], 0, &[32, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[255, 255], 0, &[0, 0, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn dec_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.dec_word((0, 1), [2, 3])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[255, 255, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 41], 0, &[0, 40, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 0], 0, &[30, 255, 0, 0], 0);
        test::compare_tape(coder.writer(), &[31, 41], 0, &[31, 40, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn add_word_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.add_word_move((0, 1), (2, 3), [4, 5, 6])?.seek(0)?;

        test::compare_tape(coder.writer(), &[1, 2, 3, 4], 0, &[0, 0, 4, 6, 0, 0, 0], 0);
        test::compare_tape(
            coder.writer(),
            &[1, 100, 3, 200],
            0,
            &[0, 0, 5, 44, 0, 0, 0],
            0,
        );
        Ok(())
    }

    #[test]
    #[ignore = "exceeds CYCLE_LIMIT imposed by the brainfuck crate"]
    fn add_word_move_long() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.add_word_move((0, 1), (2, 3), [4, 5, 6])?.seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[100, 3, 200, 4],
            0,
            &[0, 0, 44, 7, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[100, 150, 200, 250],
            0,
            &[0, 0, 45, 144, 0, 0, 0],
            0,
        );
        Ok(())
    }

    #[test]
    fn add_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.add_word((0, 1), (2, 3), [4, 5, 6, 7, 8])?.seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[1, 2, 3, 4],
            0,
            &[1, 2, 4, 6, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[1, 100, 3, 200],
            0,
            &[1, 100, 5, 44, 0, 0, 0, 0, 0],
            0,
        );
        Ok(())
    }

    #[test]
    #[ignore = "exceeds CYCLE_LIMIT imposed by the brainfuck crate"]
    fn add_word_long() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.add_word((0, 1), (2, 3), [4, 5, 6, 7, 8])?.seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[100, 3, 200, 4],
            0,
            &[100, 3, 44, 7, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[100, 150, 200, 250],
            0,
            &[100, 150, 45, 144, 0, 0, 0, 0, 0],
            0,
        );
        Ok(())
    }

    #[test]
    fn sub_word_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.sub_word_move((0, 1), (2, 3), [4, 5, 6])?.seek(0)?;

        test::compare_tape(coder.writer(), &[1, 2, 3, 4], 0, &[0, 0, 2, 2, 0, 0, 0], 0);
        test::compare_tape(
            coder.writer(),
            &[1, 4, 3, 2],
            0,
            &[0, 0, 1, 254, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[3, 2, 1, 4],
            0,
            &[0, 0, 254, 2, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[3, 4, 1, 2],
            0,
            &[0, 0, 253, 254, 0, 0, 0],
            0,
        );
        Ok(())
    }

    #[test]
    fn sub_word() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.sub_word((0, 1), (2, 3), [4, 5, 6, 7, 8])?.seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[1, 2, 3, 4],
            0,
            &[1, 2, 2, 2, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[1, 4, 3, 2],
            0,
            &[1, 4, 1, 254, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[3, 2, 1, 4],
            0,
            &[3, 2, 254, 2, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[3, 4, 1, 2],
            0,
            &[3, 4, 253, 254, 0, 0, 0, 0, 0],
            0,
        );
        Ok(())
    }
}
