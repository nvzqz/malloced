use crate::{sys, Malloced};
use core::{
    ffi::c_void,
    fmt,
    hash::{Hash, Hasher},
    ptr,
};

#[cfg(feature = "pin")]
use core::pin::Pin;

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

impl From<Malloced<str>> for Malloced<[u8]> {
    #[inline]
    fn from(m: Malloced<str>) -> Self {
        unsafe { Self::from_raw(Malloced::into_raw(m) as *mut [u8]) }
    }
}

#[cfg(feature = "pin")]
impl<T: ?Sized> From<Malloced<T>> for Pin<Malloced<T>> {
    #[inline]
    fn from(m: Malloced<T>) -> Self {
        Malloced::into_pin(m)
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

impl<T: ?Sized + Hash> Hash for Malloced<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::hash(self, state);
    }
}

impl<T: ?Sized + Hasher> Hasher for Malloced<T> {
    #[inline]
    fn finish(&self) -> u64 {
        T::finish(self)
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        T::write(self, bytes)
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        T::write_u8(self, i)
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        T::write_u16(self, i)
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        T::write_u32(self, i)
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        T::write_u64(self, i)
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        T::write_u128(self, i)
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        T::write_usize(self, i)
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        T::write_i8(self, i)
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        T::write_i16(self, i)
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        T::write_i32(self, i)
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        T::write_i64(self, i)
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        T::write_i128(self, i)
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        T::write_isize(self, i)
    }
}
