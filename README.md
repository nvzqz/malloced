# Malloced

A `malloc`-ed box pointer type, brought to you by
[@NikolaiVazquez](https://twitter.com/NikolaiVazquez)!

## Table of Contents

1. [Donate](#donate)
2. [Usage](#usage)
3. [MSRV](#msrv)
4. [FFI Safety](#ffi-safety)
5. [Alternatives](#alternatives)
6. [License](#license)

## Donate

If this project is useful to you, please consider
[sponsoring me](https://github.com/sponsors/nvzqz) or
[donating directly](https://www.paypal.me/nvzqz)!

Doing so enables me to create high-quality open source software like this. ❤️

## Usage

This library is available [on crates.io](https://crates.io/crates/malloced) and
can be used by adding the following to your project's
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

```toml
[dependencies]
malloced = "1.2.0"
```

The star of the show is [`Malloced`], [`Box`]-like pointer that calls `free` on
[`Drop`]:

```rust
use malloced::Malloced;
```

## MSRV

This library's minimum supported Rust version (MSRV) is 1.64. A new version
requirement would result in a minor version update.

## FFI Safety

`Malloced<T>` is a `#[repr(transparent)]` wrapper over `NonNull<T>`, so it can
be safely used in C FFI. For example, the following is safe and even compiles
with the `improper_ctypes` lint enabled:

```rust
#[deny(improper_ctypes)]
extern "C" {
    fn my_array_malloc() -> Malloced<[u8; 32]>;
}
```

## Alternatives

- [`malloc_buf`](https://docs.rs/malloc_buf)
- [`mbox`](https://docs.rs/mbox)

## License

This project is released under either
[MIT License](https://github.com/nvzqz/malloced/blob/master/LICENSE-MIT) or
[Apache License (Version 2.0)](https://github.com/nvzqz/malloced/blob/master/LICENSE-APACHE)
at your choosing.

[`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
[`Malloced`]: https://docs.rs/malloced/1.2.0/malloced/struct.Malloced.html
