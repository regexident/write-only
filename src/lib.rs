// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! References/slices that provide write-access, but no read-access.
//!
//! # Examples
//!
//! Write-only reference:
//! ```
//! use write_only::{prelude::*, Put};
//!
//! fn write<T: Put<u8>>(slice: &mut T) {
//!     slice.put(42u8);
//! }
//!
//! let mut value: u8 = 0;
//!
//! let mut reference = WriteOnlyRef::from(&mut value);
//! write(&mut reference);
//!
//! assert_eq!(value, 42);
//!```
//!
//! Write-only slice:
//! ```
//! use write_only::{prelude::*, PutAt};
//!
//! fn write<T: PutAt<u8>>(slice: &mut T) {
//!     slice.put_at(2, 42u8);
//! }
//!
//! let mut values: Vec<u8> = (0..10).collect();
//!
//! let mut slice = WriteOnlySlice::from(&mut values[..]);
//! write(&mut slice);
//!
//! assert_eq!(values[2], 42u8);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod reference;
mod slice;

pub use reference::{Put, VolatileWriteOnlyRef, Write, WriteOnlyRef};
pub use slice::{
    PutAt, PutFromSliceAt, VolatileWriteOnlySlice, WriteAt, WriteFromSliceAt, WriteOnlySlice,
};

/// The crate's prelude.
pub mod prelude {
    pub use crate::reference::{Put as _, VolatileWriteOnlyRef, Write as _, WriteOnlyRef};
    pub use crate::slice::{
        PutAt as _, PutFromSliceAt as _, VolatileWriteOnlySlice, WriteAt as _,
        WriteFromSliceAt as _, WriteOnlySlice,
    };
}
