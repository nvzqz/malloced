#![no_std]

use core::ptr::NonNull;

/// A pointer type for `malloc`-ed heap allocation.
#[repr(transparent)]
pub struct Malloced<T: ?Sized> {
    ptr: NonNull<T>,
}
