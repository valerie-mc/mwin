pub mod errors;
pub mod handler;
pub mod messaging;

use crate::{handler::*}; // messaging::events::*

use std::thread;
use std::time;

fn main() {
    let mut wnd_1 = WindowHandler::new("My Window 1").unwrap();

    thread::sleep(time::Duration::from_secs(5));

    println!("making second window");
    let mut wnd_2 = WindowHandler::new("My Window 2").unwrap();
    
    thread::sleep(time::Duration::from_secs(5));
    println!("closing window 1");
    wnd_1.close();
    thread::sleep(time::Duration::from_secs(5));
    wnd_2.close();
}
