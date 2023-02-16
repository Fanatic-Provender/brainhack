use {
    crate::traits::{
        core::{Core, CoreExt},
        seek::{Pos, Seek},
    },
    std::io::Write,
};

#[derive(Debug)]
pub struct Coder<W: Write> {
    writer: W,
    location: isize,
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

impl<W: Write> Seek for Coder<W> {
    fn seek(&mut self, pos: Pos) -> anyhow::Result<&mut Self> {
        let delta = pos - self.location;
        self.change_ptr_by(delta)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::test};

    #[test]
    fn core() -> anyhow::Result<()> {
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

    #[test]
    fn seek() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder
            .seek(3)?
            .inc_val_by(2)?
            .dec_ptr_by(1)?
            .inc_val_by(7)?
            .seek(-4)?
            .inc_val_by(1)?
            .seek(1)?
            .inc_val_by(8)?;

        test::compare_tape(coder.writer(), &[], 4, &[1, 0, 0, 0, 0, 8, 7, 2], 5);
        Ok(())
    }
}
