pub const REGISTER_BUFFER: usize = 27;
pub const SCREEN: usize = 8192 * 3;
pub const RAM: usize = 16384 * 3;
pub const KBD: usize = 3;
pub const TAPE_SIZE: usize = REGISTER_BUFFER + RAM + SCREEN + KBD;
