use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt;
use core::fmt::{Debug, Display, Formatter};
use core::result;

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    err: Box<ErrorImpl>,
}

impl Error {
    pub(crate) fn msg(msg: String) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                msg: msg.into_boxed_str(),
            }),
        }
    }
}

struct ErrorImpl {
    msg: Box<str>,
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error({:?})", self.err.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.err, f)
    }
}

impl serde::de::StdError for Error {}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::msg(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::msg(msg.to_string())
    }
}
