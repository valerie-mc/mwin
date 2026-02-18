pub mod errors;
pub mod handler;
pub mod messaging;

use crate::{handler::*}; // messaging::events::*

use std::thread;
use std::time;

fn main() {
    let mut wnd = WindowHandler::new("My Window").unwrap();
    wnd.close();
}
