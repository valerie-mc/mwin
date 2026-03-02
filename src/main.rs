pub mod errors;
pub mod handler;
pub mod messaging;
pub mod traits;

use crate::{handler::*}; // messaging::events::*

use std::thread;
use std::time;


fn main() {
    let wnd = WindowHandler::new("Window", 500, 500, 1000, 1000).unwrap();

    thread::sleep(time::Duration::from_secs(3));
    
    // TODO: Doesn't do anything </3
    for x in 0..999 {
        for y in 0..999 {
            wnd.set_pixel(x, y, (x % 255) as u8, (y % 255) as u8, 200);
        }
    }

    println!("done setting pixels");

    thread::sleep(time::Duration::from_secs(25));
}
