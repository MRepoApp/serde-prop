#![allow(
    // Deserializer::from_str
    clippy::should_implement_trait,
)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use crate::de::{from_slice, from_str, Deserializer};
pub use crate::error::{Error, Result};
pub use crate::ser::{to_string, to_vec, to_writer, Serializer};

mod de;
mod error;
mod io;
mod read;
mod ser;
