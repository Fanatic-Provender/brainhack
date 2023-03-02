use crate::prelude::*;

pub trait Binary: Arith {
    fn mul_two_move_cell(&mut self, src: Pos, dest: Pos) -> anyhow::Result<&mut Self> {
        self.while_(src, |s| s.seek(src)?.dec_val()?.seek(dest)?.inc_val_by(2))
    }

    // (src, dest) = (src % 2, src / 2)
    fn div_mod_two_cell(
        &mut self,
        src: Pos,
        dest: Pos,
        temp: [Pos; 4],
    ) -> anyhow::Result<&mut Self> {
        self.while_cond(
            temp[0],
            |s| {
                s.copy_cell(src, &[temp[1]], temp[3])?
                    .seek(src)?
                    .dec_val()?
                    .copy_cell(src, &[temp[2]], temp[3])?
                    .seek(src)?
                    .inc_val()?
                    .clear_cell(&[temp[0]])?
                    .logical_and_move(temp[1], temp[2], temp[0])
            },
            |s| s.seek(src)?.dec_val_by(2)?.seek(dest)?.inc_val(),
        )
    }

    fn binary_not_move(
        &mut self,
        src: Word,
        dest: Word,
        temp: [Pos; 3],
    ) -> anyhow::Result<&mut Self> {
        self.dec_word(dest, [temp[0], temp[1]])?
            .sub_word_move(src, dest, temp)
    }
    fn binary_not(&mut self, src: Word, dest: Word, temp: [Pos; 5]) -> anyhow::Result<&mut Self> {
        self.copy_word(src, &[(temp[0], temp[1])], temp[2])?
            .binary_not_move((temp[0], temp[1]), dest, [temp[2], temp[3], temp[4]])
    }

    fn binary_and_cell_move(
        &mut self,
        a: Pos,
        b: Pos,
        dest: Pos,
        temp: [Pos; 7],
    ) -> anyhow::Result<&mut Self> {
        let mul = temp[0];
        self.seek(mul)?.inc_val()?;
        for _ in 0..8 {
            self.div_mod_two_cell(a, temp[1], [temp[3], temp[4], temp[5], temp[6]])?
                .div_mod_two_cell(b, temp[2], [temp[3], temp[4], temp[5], temp[6]])?
                .logical_and_move(a, b, temp[3])?
                .move_cell(temp[1], &[a])?
                .move_cell(temp[2], &[b])?
                .if_move(temp[3], |s| {
                    // relies on adding semantics of copy_cell
                    s.copy_cell(mul, &[dest], temp[4])
                })?
                .mul_two_move_cell(mul, temp[3])?
                .move_cell(temp[3], &[mul])?;
        }
        self.clear_cell(&[mul])
    }
    fn binary_and_cell(
        &mut self,
        a: Pos,
        b: Pos,
        dest: Pos,
        temp: [Pos; 9],
    ) -> anyhow::Result<&mut Self> {
        self.copy_cell(a, &[temp[0]], temp[2])?
            .copy_cell(b, &[temp[1]], temp[3])?
            .binary_and_cell_move(
                temp[0],
                temp[1],
                dest,
                [
                    temp[2], temp[3], temp[4], temp[5], temp[6], temp[7], temp[8],
                ],
            )
    }
    fn binary_and_move(
        &mut self,
        a: Word,
        b: Word,
        dest: Word,
        temp: [Pos; 7],
    ) -> anyhow::Result<&mut Self> {
        self.binary_and_cell_move(a.0, b.0, dest.0, temp)?
            .binary_and_cell_move(a.1, b.1, dest.1, temp)
    }
    fn binary_and(
        &mut self,
        a: Word,
        b: Word,
        dest: Word,
        temp: [Pos; 9],
    ) -> anyhow::Result<&mut Self> {
        self.binary_and_cell(a.0, b.0, dest.0, temp)?
            .binary_and_cell(a.1, b.1, dest.1, temp)
    }

    fn binary_or_cell_move(
        &mut self,
        a: Pos,
        b: Pos,
        dest: Pos,
        temp: [Pos; 7],
    ) -> anyhow::Result<&mut Self> {
        let mul = temp[0];
        self.seek(mul)?.inc_val()?;
        for _ in 0..8 {
            self.div_mod_two_cell(a, temp[1], [temp[3], temp[4], temp[5], temp[6]])?
                .div_mod_two_cell(b, temp[2], [temp[3], temp[4], temp[5], temp[6]])?
                .logical_or_move(a, b, temp[3])?
                .move_cell(temp[1], &[a])?
                .move_cell(temp[2], &[b])?
                .if_move(temp[3], |s| {
                    // relies on adding semantics of copy_cell
                    s.copy_cell(mul, &[dest], temp[4])
                })?
                .mul_two_move_cell(mul, temp[3])?
                .move_cell(temp[3], &[mul])?;
        }
        self.clear_cell(&[mul])
    }
    fn binary_or_cell(
        &mut self,
        a: Pos,
        b: Pos,
        dest: Pos,
        temp: [Pos; 9],
    ) -> anyhow::Result<&mut Self> {
        self.copy_cell(a, &[temp[0]], temp[2])?
            .copy_cell(b, &[temp[1]], temp[3])?
            .binary_or_cell_move(
                temp[0],
                temp[1],
                dest,
                [
                    temp[2], temp[3], temp[4], temp[5], temp[6], temp[7], temp[8],
                ],
            )
    }
    fn binary_or_move(
        &mut self,
        a: Word,
        b: Word,
        dest: Word,
        temp: [Pos; 7],
    ) -> anyhow::Result<&mut Self> {
        self.binary_or_cell_move(a.0, b.0, dest.0, temp)?
            .binary_or_cell_move(a.1, b.1, dest.1, temp)
    }
    fn binary_or(
        &mut self,
        a: Word,
        b: Word,
        dest: Word,
        temp: [Pos; 9],
    ) -> anyhow::Result<&mut Self> {
        self.binary_or_cell(a.0, b.0, dest.0, temp)?
            .binary_or_cell(a.1, b.1, dest.1, temp)
    }
}
impl<T: Arith> Binary for T {}

#[cfg(test)]
mod tests {
    use {super::*, crate::test};

    #[test]
    fn mul_two_move_cell() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.mul_two_move_cell(0, 1)?.seek(0)?;

        test::compare_tape(coder.writer(), &[0], 0, &[0, 0], 0);
        test::compare_tape(coder.writer(), &[1], 0, &[0, 2], 0);
        test::compare_tape(coder.writer(), &[3], 0, &[0, 6], 0);
        test::compare_tape(coder.writer(), &[42], 0, &[0, 84], 0);
        test::compare_tape(coder.writer(), &[128], 0, &[0, 0], 0);
        test::compare_tape(coder.writer(), &[255], 0, &[0, 254], 0);

        Ok(())
    }

    #[test]
    fn div_mod_two_cell() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.div_mod_two_cell(0, 1, [2, 3, 4, 5])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0], 0, &[0, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[1], 0, &[1, 0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[3], 0, &[1, 1, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[42], 0, &[0, 21, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[255], 0, &[1, 127, 0, 0, 0, 0], 0);

        Ok(())
    }

    #[test]
    fn binary_not_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.binary_not_move((0, 1), (2, 3), [4, 5, 6])?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 255, 255, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 255], 0, &[0, 0, 255, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[3, 41], 0, &[0, 0, 252, 214, 0, 0, 0], 0);

        Ok(())
    }

    #[test]
    fn binary_not() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.binary_not((0, 1), (2, 3), [4, 5, 6, 7, 8])?.seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[0, 0],
            0,
            &[0, 0, 255, 255, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[0, 255],
            0,
            &[0, 255, 255, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[3, 41],
            0,
            &[3, 41, 252, 214, 0, 0, 0, 0, 0],
            0,
        );

        Ok(())
    }

    #[test]
    fn binary_and_cell_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .binary_and_cell_move(0, 1, 2, [3, 4, 5, 6, 7, 8, 9])?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[0, 0],
            0,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[0, 42],
            0,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[31, 41],
            0,
            &[0, 0, 31 & 41, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[215, 148],
            0,
            &[0, 0, 215 & 148, 0, 0, 0, 0, 0, 0, 0],
            0,
        );

        Ok(())
    }

    #[test]
    fn binary_and_cell() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .binary_and_cell(0, 1, 2, [3, 4, 5, 6, 7, 8, 9, 10, 11])?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[0, 0],
            0,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[0, 42],
            0,
            &[0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[31, 41],
            0,
            &[31, 41, 31 & 41, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[215, 148],
            0,
            &[215, 148, 215 & 148, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );

        Ok(())
    }

    #[test]
    fn binary_and_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .binary_and_move((0, 1), (2, 3), (4, 5), [6, 7, 8, 9, 10, 11, 12])?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[0, 0, 0, 0],
            0,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[31, 41, 59, 26],
            0,
            &[0, 0, 0, 0, 31 & 59, 41 & 26, 0, 0, 0, 0, 0, 0],
            0,
        );

        Ok(())
    }

    #[test]
    fn binary_and() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .binary_and((0, 1), (2, 3), (4, 5), [6, 7, 8, 9, 10, 11, 12, 13, 14])?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[0, 0, 0, 0],
            0,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[31, 41, 59, 26],
            0,
            &[31, 41, 59, 26, 31 & 59, 41 & 26, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );

        Ok(())
    }

    #[test]
    fn binary_or_cell_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .binary_or_cell_move(0, 1, 2, [3, 4, 5, 6, 7, 8, 9])?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[0, 0],
            0,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[0, 42],
            0,
            &[0, 0, 42, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[31, 41],
            0,
            &[0, 0, 31 | 41, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[215, 148],
            0,
            &[0, 0, 215 | 148, 0, 0, 0, 0, 0, 0, 0],
            0,
        );

        Ok(())
    }

    #[test]
    fn binary_or_cell() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .binary_or_cell(0, 1, 2, [3, 4, 5, 6, 7, 8, 9, 10, 11])?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[0, 0],
            0,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[0, 42],
            0,
            &[0, 42, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[31, 41],
            0,
            &[31, 41, 31 | 41, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[215, 148],
            0,
            &[215, 148, 215 | 148, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );

        Ok(())
    }

    #[test]
    fn binary_or_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .binary_or_move((0, 1), (2, 3), (4, 5), [6, 7, 8, 9, 10, 11, 12])?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[0, 0, 0, 0],
            0,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[31, 41, 59, 26],
            0,
            &[0, 0, 0, 0, 31 | 59, 41 | 26, 0, 0, 0, 0, 0, 0],
            0,
        );

        Ok(())
    }

    #[test]
    fn binary_or() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .binary_or((0, 1), (2, 3), (4, 5), [6, 7, 8, 9, 10, 11, 12, 13, 14])?
            .seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[0, 0, 0, 0],
            0,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );
        test::compare_tape(
            coder.writer(),
            &[31, 41, 59, 26],
            0,
            &[31, 41, 59, 26, 31 | 59, 41 | 26, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
        );

        Ok(())
    }
}
