//! A `malloc`-ed box pointer type, brought to you by
//! [@NikolaiVazquez](https://twitter.com/NikolaiVazquez)!
//!
//! # Table of Contents
//!
//! 1. [Donate](#donate)
//! 2. [Usage](#usage)
//! 3. [MSRV](#msrv)
//! 4. [FFI Safety](#ffi-safety)
//! 5. [Alternatives](#alternatives)
//! 6. [License](#license)
//!
//! # Donate
//!
//! If this project is useful to you, please consider
//! [sponsoring me](https://github.com/sponsors/nvzqz) or
//! [donating directly](https://www.paypal.me/nvzqz)!
//!
//! Doing so enables me to create high-quality open source software like this. ❤️
//!
//! # Usage
//!
//! This library is available [on crates.io](https://crates.io/crates/malloced) and
//! can be used by adding the following to your project's
//! [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):
//!
//! ```toml
//! [dependencies]
//! malloced = "1.0.0"
//! ```
//!
//! The star of the show is [`Malloced`], [`Box`]-like pointer that calls `free` on
//! [`Drop`]:
//!
//! ```rust
//! use malloced::Malloced;
//! ```
//!
//! # MSRV
//!
//! This library's minimum supported Rust version (MSRV) is 1.30. A new version
//! requirement would result in a minor version update.
//!
//! If the `pin` feature is enabled, the MSRV is 1.33.
//!
//! # FFI Safety
//!
//! `Malloced<T>` is a `#[repr(transparent)]` wrapper over `NonNull<T>`, so it can
//! be safely used in C FFI. For example, the following is safe and even compiles
//! with the `improper_ctypes` lint enabled:
//!
//! ```rust
//! # use malloced::Malloced;
//! #[deny(improper_ctypes)]
//! extern "C" {
//!     fn my_array_malloc() -> Malloced<[u8; 32]>;
//! }
//! ```
//!
//! # Alternatives
//!
//! - [`malloc_buf`](https://docs.rs/malloc_buf)
//! - [`mbox`](https://docs.rs/mbox)
//!
//! # License
//!
//! This project is released under either
//! [MIT License](https://github.com/nvzqz/malloced/blob/master/LICENSE-MIT) or
//! [Apache License (Version 2.0)](https://github.com/nvzqz/malloced/blob/master/LICENSE-APACHE)
//! at your choosing.
//!
//! [`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
//! [`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
//! [`Malloced`]: struct.Malloced.html

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std as core;

use core::{any::Any, mem::ManuallyDrop, ptr::NonNull};

#[cfg(feature = "pin")]
use core::pin::Pin;

mod impls;
mod sys;

/// A pointer type for `malloc`-ed heap allocation.
///
/// # Memory layout
///
/// So long as `T: Sized`, a `Malloced<T>` is guaranteed to be represented as a
/// single pointer and is also ABI-compatible with C pointers (i.e. the C type
/// `T*`). This means that if you have extern "C" Rust functions that will be
/// called from C, you can define those Rust functions using `Malloced<T>`
/// types, and use `T*` as corresponding type on the C side.
///
/// Regardless if `T: Sized`, a `Malloced<T>` is guaranteed to be ABI-compatible
/// with [`NonNull<T>`](https://doc.rust-lang.org/std/ptr/struct.NonNull.html).
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
        Self::leak(this)
    }

    /// Converts a `Malloced<T>` into a `Pin<Malloced<T>>`
    ///
    /// This conversion does not allocate on the heap and happens in place.
    ///
    /// This is also available via
    /// [`From`](https://doc.rust-lang.org/std/convert/trait.From.html).
    #[inline]
    #[cfg(feature = "pin")]
    pub fn into_pin(this: Self) -> Pin<Malloced<T>> {
        // SAFETY: It's not possible to move or replace the insides of a
        // `Pin<Malloced<T>>` when `T: !Unpin`, so it's safe to pin it directly
        // without any additional requirements.
        unsafe { Pin::new_unchecked(this) }
    }

    /// Consumes and leaks the instance, returning a mutable reference,
    /// `&'a mut T`.
    ///
    /// Note that the type `T` must outlive the chosen lifetime `'a`. If the
    /// type has only static references, or none at all, then this may be chosen
    /// to be `'static`.
    ///
    /// This function is mainly useful for data that lives for the remainder of
    /// the program's life. Dropping the returned reference will cause a memory
    /// leak. If this is not acceptable, the reference should first be wrapped
    /// with the [`Malloced::from_raw`](#method.from_raw) function producing a
    /// `Malloced`. This `Malloced` can then be dropped which will properly
    /// destroy `T` and `free` the allocated memory.
    ///
    /// Note: this is an associated function, which means that you have to call
    /// it as `Malloced::leak(this)` instead of `this.leak()`. This is so that
    /// there is no conflict with a method on the inner type.
    #[inline]
    pub fn leak<'a>(this: Self) -> &'a mut T
    where
        T: 'a,
    {
        unsafe { &mut *ManuallyDrop::new(this).ptr.as_ptr() }
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

impl Malloced<dyn Any> {
    /// Attempt to downcast the instance to a concrete type.
    #[inline]
    pub fn downcast<T: Any>(self) -> Result<Malloced<T>, Self> {
        if self.is::<T>() {
            let raw: *mut dyn Any = Malloced::into_raw(self);
            Ok(unsafe { Malloced::from_raw(raw as *mut T) })
        } else {
            Err(self)
        }
    }
}

impl Malloced<dyn Any + Send> {
    /// Attempt to downcast the instance to a concrete type.
    #[inline]
    pub fn downcast<T: Any>(self) -> Result<Malloced<T>, Self> {
        if self.is::<T>() {
            let raw: *mut (dyn Any + Send) = Malloced::into_raw(self);
            Ok(unsafe { Malloced::from_raw(raw as *mut T) })
        } else {
            Err(self)
        }
    }
}
