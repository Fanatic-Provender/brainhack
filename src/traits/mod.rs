pub mod arith;
pub mod branch;
pub mod core;
pub mod logic;
pub mod memory;
pub mod seek;

pub mod prelude {
    pub use super::{
        arith::{word, Arith, Word},
        branch::Branch,
        core::{Core, CoreExt},
        logic::Logic,
        memory::{m_pos, Memory},
        seek::{pos, Pos, Seek},
    };
}
