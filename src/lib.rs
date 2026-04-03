//! # mwin
//!
//! This crate is a minimal window manager (`mwin`) which focuses on ease of use.
//! It is specifically developed for prototyping Rust applications or for simplying
//! playing around with windows in Rust through a simple and convenient interface.
//! 
//! To get started, you can take a look at the [`WindowHandler`] documentation for
//! some examples.
//! 
//! If you are interested in more technical, but advanced window creation and
//! management libraries, I recommend checking out [winit] and similar crates.
//! 
//! [winit]: https://docs.rs/winit/latest/winit/

mod requests;
mod traits;

/// A module of the events that can occur in a window.
/// 
/// The [`crate::events::WndEvent`] enum represents any event a window can have.
/// If you're looking for information about a event, I would recommend looking
/// there first.
pub mod events;
/// A handler for a window.
pub mod handler;

#[doc(hidden)]
pub use crate::handler::WindowHandler;

/// The error type for window requests.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum WindowError {
    /// Error due to a [`crate::WindowHandler`] being created on an usupported os.
    UnsupportedOS,
    /// Error due to a window request being made on a closed window.
    WindowClosed,
}

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WindowError::UnsupportedOS => write!(f, "Cannot create a window on '{}' because it is an unsupported os.", std::env::consts::OS),
            WindowError::WindowClosed => write!(f, "The window has been closed and/or dropped."),
        }
    }
}
