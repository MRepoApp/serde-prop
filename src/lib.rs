mod de;
mod error;
mod read;
mod ser;

pub use crate::de::{from_slice, from_str, Deserializer};
pub use crate::error::{Error, Result};
pub use crate::ser::{to_string, to_vec, to_writer, Serializer};
