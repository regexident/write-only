#![cfg_attr(not(feature = "std"), no_std)]

mod reference;
mod slice;

pub use reference::{Put, VolatileWriteOnlyRef, Write, WriteOnlyRef};
pub use slice::{
    PutAt, PutFromSliceAt, VolatileWriteOnlySlice, WriteAt, WriteFromSliceAt, WriteOnlySlice,
};

pub mod prelude {
    pub use crate::reference::{Put as _, VolatileWriteOnlyRef, Write as _, WriteOnlyRef};
    pub use crate::slice::{
        PutAt as _, PutFromSliceAt as _, VolatileWriteOnlySlice, WriteAt as _,
        WriteFromSliceAt as _, WriteOnlySlice,
    };
}
