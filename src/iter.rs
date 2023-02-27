use crate::sys;
use core::{
    marker::PhantomData,
    mem,
    ptr::{self, NonNull},
    slice,
};

/// An iterator over a
/// <code>[Malloced](crate::Malloced)<[\[T\]](prim@slice)></code>.
pub struct SliceIter<T> {
    pub(super) buf: NonNull<T>,
    // Marks ownership of an instance of T.
    pub(super) marker: PhantomData<T>,
    pub(super) ptr: *mut T,
    pub(super) end: *mut T,
}

impl<T> SliceIter<T> {
    #[inline]
    fn as_raw_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len()) }
    }
}

impl<T> Drop for SliceIter<T> {
    #[inline]
    fn drop(&mut self) {
        struct DeallocGuard<'a, T: 'a>(&'a mut SliceIter<T>);

        impl<'a, T> Drop for DeallocGuard<'a, T> {
            #[inline]
            fn drop(&mut self) {
                unsafe {
                    sys::free(self.0.buf.as_ptr() as _);
                }
            }
        }

        // Deallocates the memory slice's on drop. If dropping the elements
        // panics, the memory will still be deallocated.
        let guard = DeallocGuard(self);

        // Drop remaining elements.
        unsafe {
            ptr::drop_in_place(guard.0.as_raw_mut_slice());
        }
    }
}

impl<T> Iterator for SliceIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else if mem::size_of::<T>() == 0 {
            // Purposefully don't use `ptr.offset` because for slices with
            // 0-size elements this would return the same pointer.
            self.ptr = (self.ptr as *mut i8).wrapping_add(1) as *mut T;

            // Make up a value of this ZST.
            Some(unsafe { mem::zeroed() })
        } else {
            let old = self.ptr;
            self.ptr = unsafe { self.ptr.offset(1) };

            Some(unsafe { old.read() })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<T> DoubleEndedIterator for SliceIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end == self.ptr {
            None
        } else if mem::size_of::<T>() == 0 {
            // Purposefully don't use `ptr.offset` because for slices with
            // 0-size elements this would return the same pointer.
            self.ptr = (self.ptr as *mut i8).wrapping_sub(1) as *mut T;

            // Make up a value of this ZST.
            Some(unsafe { mem::zeroed() })
        } else {
            self.end = unsafe { self.end.offset(-1) };

            Some(unsafe { self.end.read() })
        }
    }
}

impl<T> ExactSizeIterator for SliceIter<T> {
    #[inline]
    fn len(&self) -> usize {
        let diff = (self.end as usize).wrapping_sub(self.ptr as usize);

        match diff.checked_div(mem::size_of::<T>()) {
            Some(len) => len,

            // ZST
            None => diff,
        }
    }
}

impl<T> core::iter::FusedIterator for SliceIter<T> {}

#[cfg(test)]
mod tests {
    use crate::Malloced;

    mod len {
        use super::*;

        #[track_caller]
        fn test(slice: &[impl Copy]) {
            let iter = Malloced::alloc(slice).unwrap().into_iter();
            assert_eq!(iter.len(), slice.len());
        }

        #[test]
        fn zst() {
            test(&[()]);
            test(&[(), ()]);
        }

        #[test]
        fn u8() {
            test(&[1u8]);
            test(&[1u8, 2u8]);
        }

        #[test]
        fn u16() {
            test(&[1u16]);
            test(&[1u16, 2u16]);
        }

        #[test]
        fn usize() {
            test(&[1usize]);
            test(&[1usize, 2usize]);
        }
    }
}
