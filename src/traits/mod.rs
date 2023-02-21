pub mod arith;
pub mod branch;
pub mod core;
pub mod logic;
pub mod seek;

pub mod prelude {
    pub use super::{
        arith::{Arith, Word},
        branch::Branch,
        core::{Core, CoreExt},
        logic::Logic,
        seek::{Pos, Seek},
    };
}
