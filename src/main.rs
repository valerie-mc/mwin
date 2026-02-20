pub mod errors;
pub mod handler;
pub mod messaging;

use crate::{handler::*}; // messaging::events::*

use std::thread;
use std::time;

const WND_COUNT: i32 = 1000;

const W: i32 = 100;
const H: i32 = 100;

const X: i32 = 2560;

fn main() {
    let mut wnds: Vec<WindowHandler> = Vec::with_capacity(WND_COUNT as usize);

    // let wnd = WindowHandler::new("Window", 500, 500).unwrap();
    // thread::sleep(time::Duration::from_secs(3));

    for i in 0..WND_COUNT {
        let x_pos: i32 = (i*W) % X;
        let y_pos: i32 = (i*W) / X * H;
        
        let wnd = WindowHandler::new(&i.to_string(), x_pos, y_pos, W, H).unwrap();
        wnds.push(wnd);
        // thread::sleep(time::Duration::from_millis(1));
    }
    println!("Created windows!");
    thread::sleep(time::Duration::from_secs(25));
}
