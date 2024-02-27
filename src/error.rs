use alloc::boxed::Box;
use alloc::string::ToString;
use core::fmt;
use core::result;

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    err: Box<ErrorImpl>,
}

impl Error {
    pub(crate) fn msg<T: fmt::Display>(msg: T) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                msg: Box::new(msg.to_string()),
            }),
        }
    }
}

struct ErrorImpl {
    msg: Box<dyn fmt::Display>,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.err, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Debug for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.msg.to_string())
    }
}

impl fmt::Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl serde::de::StdError for Error {}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::msg(msg)
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::msg(msg)
    }
}
