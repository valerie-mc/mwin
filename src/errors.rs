use std::error;
use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WindowError {
    UnsupportedOS,
    WindowClosed,
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            WindowError::UnsupportedOS => write!(f, "Cannot create a window on '{}' because it is an unsupported os.", std::env::consts::OS),
            WindowError::WindowClosed => write!(f, "The window has been closed and/or dropped."),
        }
    }
}

impl error::Error for WindowError {}
