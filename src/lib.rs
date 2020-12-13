#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std as core;

use core::{mem, ptr::NonNull};

mod impls;
mod sys;

/// A pointer type for `malloc`-ed heap allocation.
#[repr(transparent)]
pub struct Malloced<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T: ?Sized> Malloced<T> {
    /// Constructs an instance from a raw `malloc`-ed pointer.
    ///
    /// # Safety
    ///
    /// The data referenced by `ptr` must be valid and must have been allocated
    /// by `malloc` so that it can be `free`-d on
    /// [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html).
    #[inline]
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self {
            ptr: NonNull::new_unchecked(ptr),
        }
    }

    /// Consumes the instance, returning a wrapped raw pointer.
    ///
    /// The pointer will be properly aligned and non-null.
    #[inline]
    pub fn into_raw(this: Self) -> *mut T {
        let ptr = this.ptr.as_ptr();
        mem::forget(this);
        ptr
    }

    /// Returns an immutable raw pointer to the data.
    #[inline]
    pub fn as_ptr(this: &Self) -> *const T {
        this.ptr.as_ptr()
    }

    /// Returns a mutable raw pointer to the data.
    #[inline]
    pub fn as_mut_ptr(this: &mut Self) -> *mut T {
        this.ptr.as_ptr()
    }
}

impl<T> Malloced<[T]> {
    /// Constructs an instance for a slice from a pointer and a length.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// - `data` must have been allocated by `malloc` so that it can be `free`-d
    ///   on [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html).
    ///
    /// - `data` must be
    ///   [valid](https://doc.rust-lang.org/std/ptr/index.html#safety) for both
    ///   reads and writes for `len * mem::size_of::<T>()` many bytes, and it
    ///   must be properly aligned. This means in particular:
    ///
    ///     - The entire memory range of this slice must be contained within a
    ///       single allocated object! Slices can never span across multiple
    ///       allocated objects.
    ///
    ///     - `data` must be non-null and aligned even for zero-length slices.
    ///       One reason for this is that enum layout optimizations may rely on
    ///       references (including slices of any length) being aligned and
    ///       non-null to distinguish them from other data. You can obtain a
    ///       pointer that is usable as `data` for zero-length slices using
    ///       [`NonNull::dangling()`](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.dangling).
    ///
    /// - `data` must point to `len` consecutive properly initialized values of
    ///   type `T`.
    ///
    /// - The total size `len * mem::size_of::<T>()` of the slice must be no
    ///   larger than `isize::MAX`. See the safety documentation of
    ///   [`pointer::offset`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset).
    ///
    /// See
    /// [`slice::from_raw_parts_mut`](https://doc.rust-lang.org/std/slice/fn.from_raw_parts_mut.html)
    /// for details.
    #[inline]
    pub unsafe fn slice_from_raw_parts(data: *mut T, len: usize) -> Self {
        Self::from_raw(core::slice::from_raw_parts_mut(data, len))
    }
}
