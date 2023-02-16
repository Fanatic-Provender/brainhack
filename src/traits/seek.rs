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

pub trait Seek {
    fn seek(&mut self, pos: Pos) -> anyhow::Result<&mut Self>;
}
