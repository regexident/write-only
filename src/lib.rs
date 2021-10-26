#![cfg_attr(not(feature = "std"), no_std)]

mod reference;
pub use reference::{Put, VolatileWriteOnlyRef, Write, WriteOnlyRef};

pub mod prelude {
    pub use crate::reference::{Put as _, VolatileWriteOnlyRef, Write as _, WriteOnlyRef};
}
