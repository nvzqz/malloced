#![no_std]

use core::{
    ffi::c_void,
    fmt,
    ptr::{self, NonNull},
};

mod sys;

/// A pointer type for `malloc`-ed heap allocation.
#[repr(transparent)]
pub struct Malloced<T: ?Sized> {
    ptr: NonNull<T>,
}

unsafe impl<T: ?Sized + Send> Send for Malloced<T> {}
unsafe impl<T: ?Sized + Sync> Sync for Malloced<T> {}

impl<T: ?Sized> Drop for Malloced<T> {
    #[inline]
    fn drop(&mut self) {
        let ptr = self.ptr.as_ptr();
        unsafe {
            ptr::drop_in_place(ptr);
            sys::free(ptr as *mut c_void);
        }
    }
}

impl<T: ?Sized> core::ops::Deref for Malloced<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> core::ops::DerefMut for Malloced<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized> AsRef<T> for Malloced<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: ?Sized> AsMut<T> for Malloced<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: ?Sized> core::borrow::Borrow<T> for Malloced<T> {
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}

impl<T: ?Sized> core::borrow::BorrowMut<T> for Malloced<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Malloced<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        T::fmt(self, f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for Malloced<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        T::fmt(self, f)
    }
}

impl<T: ?Sized> fmt::Pointer for Malloced<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ptr.fmt(f)
    }
}

impl<T: ?Sized + PartialEq> PartialEq for Malloced<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        T::eq(self, other)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        T::ne(self, other)
    }
}

impl<T: ?Sized + Eq> Eq for Malloced<T> {}

impl<T: ?Sized + PartialOrd> PartialOrd for Malloced<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        T::partial_cmp(self, other)
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        T::lt(self, other)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        T::le(self, other)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        T::ge(self, other)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        T::gt(self, other)
    }
}

impl<T: ?Sized + Ord> Ord for Malloced<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        T::cmp(self, other)
    }
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
