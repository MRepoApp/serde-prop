#![allow(unused)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use crate::de::{Deserializer, from_slice, from_str};
pub use crate::error::{Error, Result};
pub use crate::ser::{Serializer, to_string, to_vec, to_writer};

mod de;
mod error;
mod read;
mod ser;
mod io;
