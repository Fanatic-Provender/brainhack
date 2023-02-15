pub trait Core {
    fn inc_val(&mut self) -> anyhow::Result<&mut Self>;
    fn dec_val(&mut self) -> anyhow::Result<&mut Self>;
    fn inc_ptr(&mut self) -> anyhow::Result<&mut Self>;
    fn dec_ptr(&mut self) -> anyhow::Result<&mut Self>;
    fn start_loop(&mut self) -> anyhow::Result<&mut Self>;
    fn end_loop(&mut self) -> anyhow::Result<&mut Self>;
}

trait CoreExt: Core {
    fn loop_<F>(&mut self, f: F) -> anyhow::Result<&mut Self>
    where
        F: FnOnce(&mut Self) -> anyhow::Result<&mut Self>,
    {
        self.start_loop()?;
        f(self)?;
        self.end_loop()
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
}
