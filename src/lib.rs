pub mod coder;
pub mod hackfuck;
pub mod traits;

#[cfg(test)]
pub mod test;

pub mod prelude {
    pub use super::{coder::Coder, traits::prelude::*};
}
