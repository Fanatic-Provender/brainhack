use {crate::traits::core::Core, std::io::Write};

#[derive(Debug)]
pub struct Coder<W: Write> {
    writer: W,
    location: usize,
}

impl<W: Write> Coder<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            location: 0,
        }
    }
    pub fn writer(&self) -> &W {
        &self.writer
    }
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }
    pub fn into_writer(self) -> W {
        self.writer
    }

    pub fn write(&mut self, code: &str) -> anyhow::Result<&mut Self> {
        self.writer.write_all(code.as_bytes())?;
        Ok(self)
    }
}

impl<W: Write> Core for Coder<W> {
    fn inc_val(&mut self) -> anyhow::Result<&mut Self> {
        self.write("+")
    }
    fn dec_val(&mut self) -> anyhow::Result<&mut Self> {
        self.write("-")
    }
    fn inc_ptr(&mut self) -> anyhow::Result<&mut Self> {
        self.location += 1;
        self.write(">")
    }
    fn dec_ptr(&mut self) -> anyhow::Result<&mut Self> {
        self.location -= 1;
        self.write("<")
    }
    fn start_loop(&mut self) -> anyhow::Result<&mut Self> {
        self.write("[")
    }
    fn end_loop(&mut self) -> anyhow::Result<&mut Self> {
        self.write("]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coder() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);

        coder
            .inc_val()?
            .dec_val()?
            .inc_ptr()?
            .dec_ptr()?
            .start_loop()?
            .end_loop()?;

        assert_eq!(coder.writer, b"+-><[]");

        Ok(())
    }
}
