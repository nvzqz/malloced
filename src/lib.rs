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
