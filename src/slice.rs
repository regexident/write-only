// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Slices that only provide write-access, no read.

mod non_volatile;
mod volatile;

pub use non_volatile::WriteOnlySlice;
pub use volatile::VolatileWriteOnlySlice;

/// A trait for objects which provide **dropping indexed** write access to their values.
pub trait PutAt<T> {
    /// Puts the value at `index` to the given value, dropping the old value.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    fn put_at(&mut self, index: usize, value: T);

    /// Puts the value at `index` to the given value, dropping the old value without checking bounds.
    ///
    /// For a safe alternative see [`PutAt::put_at`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior.
    unsafe fn put_at_unchecked(&mut self, index: usize, value: T);
}

/// A trait for objects which provide **dropping indexed** write access to their values from a slice.
pub trait PutFromSliceAt<T>: PutAt<T> {
    /// Clones the elements from `src` into self, starting at `offset`, dropping the old values.
    ///
    /// The length of `src` must be less than `self.len - offset`.
    ///
    /// # Panics
    ///
    /// This function will panic if the length of `src` is greater than `self.len - offset`.
    fn put_cloning_from_slice_at(&mut self, src: &[T], offset: usize)
    where
        T: Clone;
}

/// A trait for objects which provide **non-dropping indexed** write access to their values.
pub trait WriteAt<T> {
    /// Performs a write of a memory location with the given value without reading or dropping the old value.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    fn write_at(&mut self, index: usize, value: T);

    /// Performs a write of a memory location with the given value without reading or dropping the old value.
    ///
    /// For a safe alternative see [`WriteAt::write_at`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior.
    unsafe fn write_at_unchecked(&mut self, index: usize, value: T);
}

/// A trait for objects which provide **non-dropping indexed** write access to their values from a slice.
pub trait WriteFromSliceAt<T>: WriteAt<T> {
    /// Copies the elements from `src` into `self`.
    ///
    /// The length of `src` must be less than `self.len - offset`.
    ///
    /// If `T` implements `Copy`, it can be more performant to use
    /// [`WriteFromSliceAt::write_copying_from_slice_at`].
    ///
    /// # Panics
    ///
    /// This function will panic if the length of `src` is greater than `self.len - offset`.
    fn write_cloning_from_slice_at(&mut self, src: &[T], offset: usize)
    where
        T: Clone;

    /// Copies all elements from `src` into `self`, using a memcpy.
    ///
    /// The length of `src` must be less than `self.len - offset`.
    ///
    /// If `T` does not implement `Copy`, use [`WriteFromSliceAt::write_cloning_from_slice_at`].
    ///
    /// # Panics
    ///
    /// This function will panic if the length of `src` is greater than `self.len - offset`.
    fn write_copying_from_slice_at(&mut self, src: &[T], offset: usize)
    where
        T: Copy;
}
