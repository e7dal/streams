//! Implementation of command traits for calculating the size for output buffer in Wrap operation.

/// Message size counting context.
#[derive(Debug)]
pub struct Context<TW, F> {
    /// The current message size in trits.
    size: usize,
    _phantom: std::marker::PhantomData<(TW, F)>,
}

impl<TW, F> Context<TW, F> {
    /// Creates a new Context<TW, F>.
    pub fn new() -> Self {
        Self {
            size: 0,
            _phantom: std::marker::PhantomData,
        }
    }
    /// Returns calculated message size.
    pub fn get_size(&self) -> usize {
        self.size
    }
}

mod absorb;
mod absorb_external;
mod commit;
mod dump;
mod fork;
mod join;
mod mask;
mod repeated;
mod skip;
mod squeeze;
mod squeeze_external;

mod mssig;
mod ntrukem;

pub use absorb::*;
pub use absorb_external::*;
pub use commit::*;
pub use dump::*;
pub use fork::*;
pub use join::*;
pub use mask::*;
pub use repeated::*;
pub use skip::*;
pub use squeeze::*;
pub use squeeze_external::*;

pub use mssig::*;
pub use ntrukem::*;
