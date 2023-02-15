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
    fn add_val(&mut self, n: u8) -> anyhow::Result<&mut Self> {
        for _ in 0..n {
            self.inc_val()?;
        }
        Ok(self)
    }
    fn set_val(&mut self, n: u8) -> anyhow::Result<&mut Self> {
        self.clear_val()?.add_val(n)
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
            .add_val(30)?
            .inc_ptr()?
            .set_val(42)?;

        test::compare_tape(coder.writer(), &[27, 18, 28], 0, &[0, 48, 42], 2);
        Ok(())
    }
}
