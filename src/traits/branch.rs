use crate::traits::seek::{Pos, Seek};

pub trait Branch: Seek {
    fn if_move<F>(&mut self, cond: Pos, f: F) -> anyhow::Result<&mut Self>
    where
        F: FnOnce(&mut Self) -> anyhow::Result<&mut Self>,
    {
        self.while_(cond, |s| f(s.clear_val()?))
    }
    fn if_else_move<F, G>(&mut self, cond: Pos, temp: Pos, f: F, g: G) -> anyhow::Result<&mut Self>
    where
        F: FnOnce(&mut Self) -> anyhow::Result<&mut Self>,
        G: FnOnce(&mut Self) -> anyhow::Result<&mut Self>,
    {
        self.seek(temp)?
            .inc_val()?
            .if_move(cond, |s| f(s)?.seek(temp)?.dec_val())?
            .if_move(temp, g)
    }
}
impl<T: Seek> Branch for T {}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{coder::Coder, test, traits::core::CoreExt},
    };

    #[test]
    fn if_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .if_move(0, |c| c.seek(2)?.inc_val_by(3))?
            .if_move(0, |c| c.seek(1)?.inc_val_by(4))?;

        test::compare_tape(coder.writer(), &[5], 0, &[0, 0, 3], 0);
        Ok(())
    }

    #[test]
    fn if_else_move() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .if_else_move(
                0,
                1,
                |c| c.seek(2)?.inc_val_by(2),
                |c| c.seek(3)?.inc_val_by(7),
            )?
            .if_else_move(
                0,
                1,
                |c| c.seek(2)?.inc_val_by(1),
                |c| c.seek(3)?.inc_val_by(8),
            )?
            .seek(0)?;

        test::compare_tape(coder.writer(), &[5], 0, &[0, 0, 2, 8], 0);
        Ok(())
    }
}
