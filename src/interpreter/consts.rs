pub const REGISTER_BUFFER: usize = 19;
pub const SCREEN: usize = 24576;
pub const RAM: usize = 49152;
pub const KBD: usize = 3;
pub const TAPE_SIZE: usize = REGISTER_BUFFER + RAM + SCREEN + KBD;


// Au Al T0 Du Dl T1 Mu Ml T2
// Pu Pl T3 Qu Ql T4 Ru Rl T5
// Fu Fl T6 Vu Vl T7 Wu Wl T8
// S0 S1 G0 S2 S3 G1 S4 S5 G2
// S6 S7 G3 S8 S9 G4 ...