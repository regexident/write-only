// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! References that only provide write-access, no read.

mod non_volatile;
mod volatile;

pub use non_volatile::WriteOnlyRef;
pub use volatile::VolatileWriteOnlyRef;

/// A trait for objects which provide **dropping** write access to their value.
pub trait Put<T> {
    /// Puts the value the given value, dropping the old value.
    fn put(&mut self, value: T);
}

/// A trait for objects which provide **non-dropping** write access to their value.
pub trait Write<T> {
    /// Writes the value the given value without dropping the old value.
    fn write(&mut self, value: T);
}
