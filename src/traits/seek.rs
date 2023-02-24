use crate::prelude::*;

pub type Pos = isize;

pub mod pos {
    use super::Pos;

    pub const AU: Pos = 0;
    pub const AL: Pos = 1;
    pub const T0: Pos = 2;
    pub const DU: Pos = 3;
    pub const DL: Pos = 4;
    pub const T1: Pos = 5;
    pub const MU: Pos = 6;
    pub const ML: Pos = 7;
    pub const T2: Pos = 8;
    pub const RU: Pos = 9;
    pub const RL: Pos = 10;
    pub const T3: Pos = 11;
    pub const F: Pos = 12;
    pub const T4: Pos = 13;
    pub const T5: Pos = 14;
    pub const T6: Pos = 15;
    pub const T7: Pos = 16;
    pub const T8: Pos = 17;
}

pub trait Seek: CoreExt {
    fn seek(&mut self, pos: Pos) -> anyhow::Result<&mut Self>;

    fn while_<F>(&mut self, cond: Pos, f: F) -> anyhow::Result<&mut Self>
    where
        F: FnOnce(&mut Self) -> anyhow::Result<&mut Self>,
    {
        self.seek(cond)?.loop_(|s| f(s)?.seek(cond))
    }

    fn clear_cell(&mut self, cells: &[Pos]) -> anyhow::Result<&mut Self> {
        for &cell in cells {
            self.seek(cell)?.clear_val()?;
        }
        Ok(self)
    }
    fn move_cell(&mut self, src: Pos, dests: &[Pos]) -> anyhow::Result<&mut Self> {
        self.while_(src, |s| {
            s.dec_val()?;
            for &dest in dests {
                s.seek(dest)?.inc_val()?;
            }
            Ok(s)
        })
    }
    fn move_cell_overwrite(&mut self, src: Pos, dests: &[Pos]) -> anyhow::Result<&mut Self> {
        self.clear_cell(dests)?.move_cell(src, dests)
    }
    fn copy_cell(&mut self, src: Pos, dests: &[Pos], temp: Pos) -> anyhow::Result<&mut Self> {
        let mut dests_temp = dests.to_vec();
        dests_temp.push(temp);

        self.move_cell(src, &dests_temp)?.move_cell(temp, &[src])
    }
    fn copy_cell_overwrite(
        &mut self,
        src: Pos,
        dests: &[Pos],
        temp: Pos,
    ) -> anyhow::Result<&mut Self> {
        self.clear_cell(dests)?.copy_cell(src, dests, temp)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{coder::Coder, test, traits::core::Core},
    };

    #[test]
    fn while_() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.while_(0, |c| c.dec_val()?.seek(1)?.inc_val_by(3))?;

        test::compare_tape(coder.writer(), &[5], 0, &[0, 15], 0);
        Ok(())
    }

    #[test]
    fn clear_cell() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.clear_cell(&[0, 1, 3, 4])?.seek(0)?;

        test::compare_tape(coder.writer(), &[3, 1, 4, 1, 5], 0, &[0, 0, 4, 0, 0], 0);
        Ok(())
    }

    #[test]
    fn add_move_cell() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.move_cell(2, &[0, 1, 3, 4])?;

        test::compare_tape(coder.writer(), &[3, 1, 4, 1, 5], 0, &[7, 5, 0, 5, 9], 2);
        Ok(())
    }

    #[test]
    fn move_cell() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.move_cell_overwrite(2, &[0, 1, 3, 4])?;

        test::compare_tape(coder.writer(), &[3, 1, 4, 1, 5], 0, &[4, 4, 0, 4, 4], 2);
        Ok(())
    }

    #[test]
    fn add_copy_cell() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.copy_cell(2, &[0, 1, 3, 4], 5)?;

        test::compare_tape(coder.writer(), &[3, 1, 4, 1, 5], 0, &[7, 5, 4, 5, 9], 5);
        Ok(())
    }

    #[test]
    fn copy_cell() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.copy_cell_overwrite(2, &[0, 1, 3, 4], 5)?;

        test::compare_tape(coder.writer(), &[3, 1, 4, 1, 5], 0, &[4, 4, 4, 4, 4], 5);
        Ok(())
    }
}
