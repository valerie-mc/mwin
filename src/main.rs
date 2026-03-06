pub mod errors;
pub mod handler;
pub mod messaging;
pub mod traits;

use crate::messaging::events::{KeyCode, WndEvent};
use crate::handler::WindowHandler;

// use std::thread;
// use std::time;


const DIRECT: bool = true;

fn main() {
    let mut wnd = WindowHandler::new("Window", 500, 100, 500, 500).unwrap();

    // TODO: Test what happens with keyboard inputs when two windows are open (none thingy)

    let mut running = true;

    draw_pixels(&wnd, 500, 500, 500, 500);
    wnd.draw_buffer();

    // TODO: Why is drawing while moving so slow (is it because my drawing code is slow lol?)

    while running {
        for event in wnd.get_wnd_events() {
            match event {
                WndEvent::KeyboardInput { event } => {
                    match event.key {
                        KeyCode::Q => running = false,
                        _ => (),
                    }
                }
                WndEvent::WindowClosed => running = false,
                WndEvent::WindowPosChanged { x, y, width, height } => {
                    draw_pixels(&wnd, x, y, width, height);
                }
                _ => ()
            }
        }
    }
}

fn draw_pixels(wnd: &WindowHandler, c_x: i32, c_y: i32, w: i32, h: i32) {
    // let (c_x, c_y, w, h) = wnd.get_wnd_rect();
    println!("{:?}", (c_x, c_y, w, h));
    // return;

    wnd.resize_buffer(w, h);


    if DIRECT {
        let mut temp_buffer = vec![0; (4 * w * h) as usize];

        for y in 0..h {
            for x in 0..w {
                temp_buffer[(4 * (w * y + x) + 2) as usize] = ((c_x + x) % 255) as u8; // r
                temp_buffer[(4 * (w * y + x) + 1) as usize] = ((c_y + y) % 255) as u8; // g
                temp_buffer[(4 * (w * y + x)) as usize]     = 200; // b
            }
        }

        wnd.set_buffer_direct(temp_buffer);
        wnd.draw_buffer();
    } else {
        let mut temp_buffer = vec![0; (3 * w * h) as usize];

        for y in 0..h {
            for x in 0..w {
                temp_buffer[3 * (w * y + x) as usize]     = ((c_x + x) % 255) as u8; // r
                temp_buffer[3 * (w * y + x) as usize + 1] = ((c_y + y) % 255) as u8; // g
                temp_buffer[3 * (w * y + x) as usize + 2] = 200;  // b
            }
        }
        wnd.set_buffer(temp_buffer);
        // wnd.draw_buffer();
    }
}