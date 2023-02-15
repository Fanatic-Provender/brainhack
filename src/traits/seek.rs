pub type Pos = isize;

pub trait Seek {
    fn seek(&mut self, pos: Pos) -> anyhow::Result<&mut Self>;
}
