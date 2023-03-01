use crate::prelude::*;

pub trait Binary: Arith {
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
}
impl<T: Arith> Binary for T {}

#[cfg(test)]
mod tests {
    use {super::*, crate::test};

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
}
