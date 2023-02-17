use crate::traits::{branch::Branch, seek::Pos};

pub trait Logic: Branch {
    fn logical_not_move(&mut self, src: Pos, dest: Pos) -> anyhow::Result<&mut Self> {
        self.seek(dest)?
            .inc_val()?
            .if_move(src, |s| s.seek(dest)?.dec_val())
    }
    fn logical_not(&mut self, src: Pos, dest: Pos, temp: Pos) -> anyhow::Result<&mut Self> {
        self.copy_cell(src, &[temp], dest)?
            .logical_not_move(temp, dest)
    }

    fn logical_or_move(&mut self, a: Pos, b: Pos, dest: Pos) -> anyhow::Result<&mut Self> {
        self.if_move(a, |s| s.seek(dest)?.inc_val())?
            .if_move(b, |s| s.seek(dest)?.inc_val())
    }
    fn logical_or(
        &mut self,
        a: Pos,
        b: Pos,
        dest: Pos,
        temp_1: Pos,
        temp_2: Pos,
    ) -> anyhow::Result<&mut Self> {
        self.copy_cell(a, &[temp_1], dest)?
            .copy_cell(b, &[temp_2], dest)?
            .logical_or_move(temp_1, temp_2, dest)
    }
}
impl<T: Branch> Logic for T {}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{coder::Coder, test, traits::seek::Seek},
    };

    #[test]
    fn logical_not_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .logical_not_move(0, 2)?
            .logical_not_move(1, 3)?
            .seek(0)?;

        test::compare_tape(coder.writer(), &[5, 0], 0, &[0, 0, 0, 1], 0);
        Ok(())
    }

    #[test]
    fn logical_not() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.logical_not(0, 2, 4)?.logical_not(1, 3, 4)?.seek(0)?;

        test::compare_tape(coder.writer(), &[5, 0], 0, &[5, 0, 0, 1, 0], 0);
        Ok(())
    }

    #[test]
    fn logical_or_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.logical_or_move(0, 1, 2)?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[3, 0], 0, &[0, 0, 1], 0);
        test::compare_tape(coder.writer(), &[0, 5], 0, &[0, 0, 1], 0);
        test::compare_tape(coder.writer(), &[4, 6], 0, &[0, 0, 2], 0);
        Ok(())
    }

    #[test]
    fn logical_or() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.logical_or(0, 1, 2, 3, 4)?.seek(0)?;

        test::compare_tape(coder.writer(), &[0, 0], 0, &[0, 0, 0, 0, 0], 0);
        test::compare_tape(coder.writer(), &[3, 0], 0, &[3, 0, 1, 0, 0], 0);
        test::compare_tape(coder.writer(), &[0, 5], 0, &[0, 5, 1, 0, 0], 0);
        test::compare_tape(coder.writer(), &[4, 6], 0, &[4, 6, 2, 0, 0], 0);
        Ok(())
    }
}
