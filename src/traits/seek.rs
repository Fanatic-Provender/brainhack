use crate::traits::core::CoreExt;

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
}
