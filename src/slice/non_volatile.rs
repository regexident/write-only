use core::{marker::PhantomData, mem};

use crate::{PutAt, PutFromSliceAt, WriteAt, WriteFromSliceAt};

/// A write-only **slice** with **dropping non-volatile** write access.
pub struct WriteOnlySlice<'a, T: 'a> {
    data: *mut T,
    len: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: 'a> WriteOnlySlice<'a, T> {
    /// Forms a write-only slice from a pointer and a length.
    ///
    /// The `len` argument is the number of **elements**, not the number of bytes.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// * `data` must be [valid](http://doc.rust-lang.org/core/ptr/index.html#safety) for reads for `len * mem::size_of::<T>()` many bytes,
    ///   and it must be properly aligned. This means in particular:
    ///
    ///     * The entire memory range of this slice must be contained within a single allocated object!
    ///       Slices can never span across multiple allocated objects. See [below](#incorrect-usage)
    ///       for an example incorrectly not taking this into account.
    ///     * `data` must be non-null and aligned even for zero-length slices. One
    ///       reason for this is that enum layout optimizations may rely on references
    ///       (including slices of any length) being aligned and non-null to distinguish
    ///       them from other data. You can obtain a pointer that is usable as `data`
    ///       for zero-length slices using [`::core::ptr::NonNull::dangling()`].
    ///
    /// * `data` must point to `len` consecutive properly initialized items of type `T`.
    ///
    /// * The memory referenced by the returned slice must not be mutated for the duration
    ///   of lifetime `'a`, except inside an `UnsafeCell`.
    ///
    /// * The total size `len * mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`.
    ///   See the safety documentation of
    ///   [`pointer::offset`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset).
    ///
    /// # Caveat
    ///
    /// The lifetime for the returned slice is inferred from its usage. To
    /// prevent accidental misuse, it's suggested to tie the lifetime to whichever
    /// source lifetime is safe in the context, such as by providing a helper
    /// function taking the lifetime of a host value for the slice, or by explicit
    /// annotation.
    #[inline]
    pub unsafe fn from_raw_parts(data: *mut T, len: usize) -> Self {
        debug_assert!(
            !data.is_null() && (data.align_offset(mem::align_of::<u16>()) == 0),
            "attempt to create unaligned or null slice"
        );
        debug_assert!(
            mem::size_of::<T>().saturating_mul(len) <= isize::MAX as usize,
            "attempt to create slice covering at least half the address space"
        );
        // SAFETY: the caller must uphold the safety contract for `from_raw_parts`.
        Self {
            data,
            len,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<'a, T: 'a> PutAt<T> for WriteOnlySlice<'a, T> {
    #[inline]
    fn put_at(&mut self, index: usize, value: T) {
        assert!(index < self.len);

        unsafe {
            self.put_at_unchecked(index, value);
        }
    }

    #[inline]
    unsafe fn put_at_unchecked(&mut self, index: usize, value: T) {
        *self.data.add(index) = value;
    }
}

impl<'a, T: 'a> WriteAt<T> for WriteOnlySlice<'a, T> {
    #[inline]
    fn write_at(&mut self, index: usize, value: T) {
        assert!(index < self.len);

        unsafe {
            self.write_at_unchecked(index, value);
        }
    }

    #[inline]
    unsafe fn write_at_unchecked(&mut self, index: usize, value: T) {
        self.data.add(index).write(value);
    }
}

impl<'a, T: 'a> PutFromSliceAt<T> for WriteOnlySlice<'a, T> {
    #[inline]
    fn put_cloning_from_slice_at(&mut self, src: &[T], offset: usize)
    where
        T: Clone,
    {
        assert!(offset + src.len() <= self.len);

        // SAFETY: `self` is valid for `self.len()` elements by definition,
        // and `src` was checked to have a length less than `self.len() - offset`.
        // The slices cannot overlap because mutable references are exclusive.

        for (index, item) in src.iter().enumerate() {
            unsafe {
                *self.data.add(offset + index) = item.clone();
            }
        }
    }
}

impl<'a, T: 'a> WriteFromSliceAt<T> for WriteOnlySlice<'a, T> {
    #[inline]
    fn write_cloning_from_slice_at(&mut self, src: &[T], offset: usize)
    where
        T: Clone,
    {
        assert!(offset + src.len() <= self.len);

        // SAFETY: `self` is valid for `self.len()` elements by definition,
        // and `src` was checked to have a length less than `self.len() - offset`.
        // The slices cannot overlap because mutable references are exclusive.

        for (index, item) in src.iter().enumerate() {
            unsafe {
                self.data.add(offset + index).write(item.clone());
            }
        }
    }

    #[inline]
    fn write_copying_from_slice_at(&mut self, src: &[T], offset: usize)
    where
        T: Copy,
    {
        assert!(src.len() <= self.len - offset);

        // SAFETY: `self` is valid for `self.len()` elements by definition,
        // and `src` was checked to have a length less than `self.len - offset`.
        // The slices cannot overlap because mutable references are exclusive.
        unsafe {
            self.data
                .add(offset)
                .copy_from_nonoverlapping(src.as_ptr(), src.len());
        }
    }
}

impl<'a, T: 'a> From<&'a mut [T]> for WriteOnlySlice<'a, T> {
    #[inline]
    fn from(slice: &'a mut [T]) -> Self {
        unsafe { Self::from_raw_parts(slice.as_mut_ptr(), slice.len()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use droptest::prelude::*;

    #[test]
    fn from_raw_parts() {
        let registry = DropRegistry::default();
        let mut guards: Vec<_> = (0..3).map(|i| registry.new_guard_for(i)).collect();

        let reference = unsafe { WriteOnlySlice::from_raw_parts(&mut guards, 3) };

        std::mem::drop(reference);

        assert_drop_stats!(registry, { created: 3, dropped: 0 });

        std::mem::drop(guards);

        assert_drop_stats!(registry, { created: 3, dropped: 3 });
    }

    #[test]
    fn from() {
        let registry = DropRegistry::default();
        let mut guards: Vec<_> = (0..3).map(|i| registry.new_guard_for(i)).collect();

        let reference = WriteOnlySlice::from(&mut guards[..]);

        std::mem::drop(reference);

        assert_drop_stats!(registry, { created: 3, dropped: 0 });

        std::mem::drop(guards);

        assert_drop_stats!(registry, { created: 3, dropped: 3 });
    }

    #[test]
    fn put_at() {
        let registry = DropRegistry::default();
        let (old_ids, mut guards): (Vec<_>, Vec<_>) =
            (0..3).map(|i| registry.new_guard_for(i).by_id()).unzip();
        let (new_id, new_guard) = registry.new_guard_for(3).by_id();

        let mut slice = WriteOnlySlice::from(&mut guards[..]);
        slice.put_at(1, new_guard);

        assert_eq!(guards[1].id(), new_id);
        assert_eq!(guards[1].value(), &3);

        assert_drop!(registry, old_ids[1]);
        assert_drop_stats!(registry, { created: 4, dropped: 1 });
    }

    #[test]
    #[should_panic]
    fn put_at_out_of_bounds() {
        let registry = DropRegistry::default();
        let mut guards: Vec<_> = (0..3).map(|i| registry.new_guard_for(i)).collect();
        let new_guard = registry.new_guard_for(3);

        let mut slice = WriteOnlySlice::from(&mut guards[..]);
        slice.put_at(10, new_guard);
    }

    #[test]
    fn write_at() {
        let registry = DropRegistry::default();
        let (old_ids, mut guards): (Vec<_>, Vec<_>) =
            (0..3).map(|i| registry.new_guard_for(i).by_id()).unzip();
        let (new_id, new_guard) = registry.new_guard_for(3).by_id();

        let mut slice = WriteOnlySlice::from(&mut guards[..]);
        slice.write_at(1, new_guard);

        assert_eq!(guards[1].id(), new_id);
        assert_eq!(guards[1].value(), &3);

        assert_no_drop!(registry, old_ids[1]);
        assert_drop_stats!(registry, { created: 4, dropped: 0 });
    }

    #[test]
    #[should_panic]
    fn write_at_out_of_bounds() {
        let registry = DropRegistry::default();
        let mut guards: Vec<_> = (0..3).map(|i| registry.new_guard_for(i)).collect();
        let new_guard = registry.new_guard_for(3);

        let mut slice = WriteOnlySlice::from(&mut guards[..]);
        slice.write_at(10, new_guard);
    }

    #[test]
    fn put_cloning_from_slice_at() {
        let registry = DropRegistry::default();
        let (old_ids, mut guards): (Vec<_>, Vec<_>) =
            (0..5).map(|i| registry.new_guard_for(i).by_id()).unzip();
        let new_guards: Vec<_> = (5..8).map(|i| registry.new_guard_for(i)).collect();

        let mut slice = WriteOnlySlice::from(&mut guards[..]);
        slice.put_cloning_from_slice_at(&new_guards[..], 1);

        assert_ne!(guards[1].id(), old_ids[1]);
        assert_eq!(guards[1].value(), &5);
        assert_ne!(guards[2].id(), old_ids[1]);
        assert_eq!(guards[2].value(), &6);
        assert_ne!(guards[3].id(), old_ids[2]);
        assert_eq!(guards[3].value(), &7);

        assert_drop!(registry, old_ids[1]);
        assert_drop!(registry, old_ids[2]);
        assert_drop!(registry, old_ids[3]);
        assert_drop_stats!(registry, { created: 11, dropped: 3 });
    }

    #[test]
    fn write_cloning_from_slice_at() {
        let registry = DropRegistry::default();
        let (old_ids, mut guards): (Vec<_>, Vec<_>) =
            (0..5).map(|i| registry.new_guard_for(i).by_id()).unzip();
        let new_guards: Vec<_> = (5..8).map(|i| registry.new_guard_for(i)).collect();

        let mut slice = WriteOnlySlice::from(&mut guards[..]);
        slice.write_cloning_from_slice_at(&new_guards[..], 1);

        assert_ne!(guards[1].id(), old_ids[1]);
        assert_eq!(guards[1].value(), &5);
        assert_ne!(guards[2].id(), old_ids[1]);
        assert_eq!(guards[2].value(), &6);
        assert_ne!(guards[3].id(), old_ids[2]);
        assert_eq!(guards[3].value(), &7);

        assert_no_drop!(registry, old_ids[1]);
        assert_no_drop!(registry, old_ids[2]);
        assert_no_drop!(registry, old_ids[3]);
        assert_drop_stats!(registry, { created: 11, dropped: 0 });
    }

    #[test]
    fn write_copying_from_slice_at() {
        let mut values: Vec<_> = (0..5).collect();
        let new_values: Vec<_> = (5..8).collect();

        let mut slice = WriteOnlySlice::from(&mut values[..]);
        slice.write_copying_from_slice_at(&new_values[..], 1);

        assert_eq!(values, &[0, 5, 6, 7, 4]);
    }
}
