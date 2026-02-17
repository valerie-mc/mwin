use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum WindowError {
    ERROR_UNSUPPORTED_OS,
}

impl Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WindowError::ERROR_UNSUPPORTED_OS => write!(f, "Cannot create a window on '{}' because it is an unsupported os.", std::env::consts::OS),
        }
    }
}

impl Error for WindowError {}
