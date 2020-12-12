#![no_std]

use core::{
    ffi::c_void,
    ptr::{self, NonNull},
};

/// A pointer type for `malloc`-ed heap allocation.
#[repr(transparent)]
pub struct Malloced<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T: ?Sized> Drop for Malloced<T> {
    #[inline]
    fn drop(&mut self) {
        extern "C" {
            fn free(ptr: *mut c_void);
        }

        let ptr = self.ptr.as_ptr();
        unsafe {
            ptr::drop_in_place(ptr);
            free(ptr as *mut c_void);
        }
    }
}
