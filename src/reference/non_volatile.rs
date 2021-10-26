// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::marker::PhantomData;

use crate::{Put, Write};

/// A write-only **reference** with **dropping non-volatile** write access.
pub struct WriteOnlyRef<'a, T: 'a> {
    data: *mut T,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: 'a> WriteOnlyRef<'a, T> {
    /// Forms a write-only reference from a pointer.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// * `data` must be [valid](http://doc.rust-lang.org/core/ptr/index.html#safety) for reads for `len * mem::size_of::<T>()` many bytes,
    ///   and it must be properly aligned. This means in particular:
    ///
    ///     * `data` must be non-null and aligned. One reason for this is that enum
    ///       layout optimizations may rely on references being aligned and non-null
    ///       to distinguish them from other data.
    ///
    /// * The memory referenced by the returned reference must not be mutated for the duration
    ///   of lifetime `'a`, except inside an `UnsafeCell`.
    ///
    /// # Caveat
    ///
    /// The lifetime for the returned reference is inferred from its usage. To
    /// prevent accidental misuse, it's suggested to tie the lifetime to whichever
    /// source lifetime is safe in the context, such as by providing a helper
    /// function taking the lifetime of a host guard for the reference, or by explicit
    /// annotation.
    #[inline]
    pub unsafe fn from_ptr(data: *mut T) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: 'a> Put<T> for WriteOnlyRef<'a, T> {
    #[inline]
    fn put(&mut self, guard: T) {
        unsafe {
            *self.data = guard;
        }
    }
}

impl<'a, T: 'a> Write<T> for WriteOnlyRef<'a, T> {
    #[inline]
    fn write(&mut self, guard: T) {
        unsafe {
            self.data.write(guard);
        }
    }
}

impl<'a, T: 'a> From<&'a mut T> for WriteOnlyRef<'a, T> {
    #[inline]
    fn from(borrow: &'a mut T) -> Self {
        unsafe { Self::from_ptr(borrow as *mut T) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use droptest::prelude::*;

    #[test]
    fn from_ptr() {
        let registry = DropRegistry::default();
        let (id, mut guard) = registry.new_guard_for(1).by_id();

        let reference = unsafe { WriteOnlyRef::from_ptr(&mut guard) };

        std::mem::drop(reference);

        assert_no_drop!(registry, id);
    }

    #[test]
    fn from() {
        let registry = DropRegistry::default();
        let (id, mut guard) = registry.new_guard_for(1).by_id();

        let reference = WriteOnlyRef::from(&mut guard);

        std::mem::drop(reference);

        assert_no_drop!(registry, id);
    }

    #[test]
    fn put() {
        let registry = DropRegistry::default();
        let (old_id, mut guard) = registry.new_guard_for(1).by_id();
        let (new_id, new_guard) = registry.new_guard_for(2).by_id();

        let mut reference = WriteOnlyRef::from(&mut guard);
        reference.put(new_guard);

        assert_eq!(guard.value(), &2);

        assert_drop!(registry, old_id);
        assert_no_drop!(registry, new_id);

        std::mem::drop(guard);

        assert_drop!(registry, old_id);
        assert_drop!(registry, new_id);
    }

    #[test]
    fn write() {
        let registry = DropRegistry::default();
        let (old_id, mut guard) = registry.new_guard_for(1).by_id();
        let (new_id, new_guard) = registry.new_guard_for(2).by_id();

        let mut reference = WriteOnlyRef::from(&mut guard);
        reference.write(new_guard);

        assert_eq!(guard.value(), &2);

        assert_no_drop!(registry, old_id);
        assert_no_drop!(registry, new_id);

        std::mem::drop(guard);

        assert_no_drop!(registry, old_id);
        assert_drop!(registry, new_id);
    }
}
