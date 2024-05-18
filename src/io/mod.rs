pub use self::io::{Result, Write};

#[cfg(not(feature = "std"))]
#[path = "core.rs"]
mod io;

#[cfg(feature = "std")]
use std::io;
