use std::cmp::Ordering;

pub trait Core {
    fn inc_val(&mut self) -> anyhow::Result<&mut Self>;
    fn dec_val(&mut self) -> anyhow::Result<&mut Self>;
    fn inc_ptr(&mut self) -> anyhow::Result<&mut Self>;
    fn dec_ptr(&mut self) -> anyhow::Result<&mut Self>;
    fn start_loop(&mut self) -> anyhow::Result<&mut Self>;
    fn end_loop(&mut self) -> anyhow::Result<&mut Self>;
}

pub trait CoreExt: Core {
    fn loop_<F>(&mut self, f: F) -> anyhow::Result<&mut Self>
    where
        F: FnOnce(&mut Self) -> anyhow::Result<&mut Self>,
    {
        self.start_loop()?;
        f(self)?;
        self.end_loop()
    }
    fn clear_val(&mut self) -> anyhow::Result<&mut Self> {
        self.loop_(|s| s.dec_val())
    }
    fn set_val(&mut self, n: u8) -> anyhow::Result<&mut Self> {
        self.clear_val()?.inc_val_by(n)
    }
    fn inc_val_by(&mut self, n: u8) -> anyhow::Result<&mut Self> {
        for _ in 0..n {
            self.inc_val()?;
        }
        Ok(self)
    }
    fn dec_val_by(&mut self, n: u8) -> anyhow::Result<&mut Self> {
        for _ in 0..n {
            self.dec_val()?;
        }
        Ok(self)
    }
    fn inc_ptr_by(&mut self, n: usize) -> anyhow::Result<&mut Self> {
        for _ in 0..n {
            self.inc_ptr()?;
        }
        Ok(self)
    }
    fn dec_ptr_by(&mut self, n: usize) -> anyhow::Result<&mut Self> {
        for _ in 0..n {
            self.dec_ptr()?;
        }
        Ok(self)
    }
    fn change_ptr_by(&mut self, delta: isize) -> anyhow::Result<&mut Self> {
        let n = delta.unsigned_abs();

        match delta.cmp(&0) {
            Ordering::Less => self.dec_ptr_by(n),
            Ordering::Equal => Ok(self),
            Ordering::Greater => self.inc_ptr_by(n),
        }
    }
}
impl<T: Core> CoreExt for T {}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{coder::Coder, test},
    };

    #[test]
    fn loop_() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.loop_(|c| c.dec_val())?;

        test::compare_tape(coder.writer(), &[42], 0, &[0], 0);
        Ok(())
    }

    #[test]
    fn set_val() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .clear_val()?
            .inc_ptr()?
            .set_val(30)?
            .inc_ptr()?
            .set_val(42)?;

        test::compare_tape(coder.writer(), &[27, 0, 28], 0, &[0, 30, 42], 2);
        Ok(())
    }

    #[test]
    fn change_val() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .inc_val_by(27)?
            .inc_ptr()?
            .dec_val_by(18)?
            .inc_ptr()?
            .inc_val_by(28)?;

        test::compare_tape(coder.writer(), &[31, 41, 59], 0, &[58, 23, 87], 2);
        Ok(())
    }

    #[test]
    fn change_ptr() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .inc_ptr_by(3)?
            .inc_val_by(2)?
            .dec_ptr_by(1)?
            .dec_val_by(7)?
            .change_ptr_by(4)?
            .inc_val_by(1)?
            .change_ptr_by(-1)?
            .set_val(8)?
            .change_ptr_by(0)?
            .dec_val_by(2)?;

        test::compare_tape(coder.writer(), &[], 0, &[0, 0, 249, 2, 0, 6, 1], 5);
        Ok(())
    }
}
