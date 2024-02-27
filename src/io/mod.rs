//! Copy from [serde-rs/json](https://github.com/serde-rs/json/blob/0de7b516f504b5c1df1774425eef18efb132ed93/src/io/mod.rs)

//! A tiny, `no_std`-friendly facade around `std::io`.
//! Reexports types from `std` when available; otherwise reimplements and
//! provides some of the core logic.
//!
//! The main reason that `std::io` hasn't found itself reexported as part of
//! the `core` crate is the `std::io::{Read, Write}` traits' reliance on
//! `std::io::Error`, which may contain internally a heap-allocated `Box<Error>`
//! and/or now relying on OS-specific `std::backtrace::Backtrace`.

pub use self::imp::{Result, Write};

#[cfg(not(feature = "std"))]
#[path = "core.rs"]
mod imp;

#[cfg(feature = "std")]
use std::io as imp;