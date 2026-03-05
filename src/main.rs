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

    let mut running = true;

    draw_pixels(&wnd);

    while running {
        for event in wnd.get_wnd_events() {
            match event {
                WndEvent::KeyboardInput { event } => {
                    match event.key {
                        KeyCode::D => draw_pixels(&wnd),
                        KeyCode::E => {
                            wnd.clear_buffer();
                            wnd.draw_buffer();
                        }
                        KeyCode::Q => running = false,
                        _ => ()
                    }
                }
                // TODO: Don't get updates when the windows is currently being resized
                // TODO: Idk how important this really is, but it would be nice
                WndEvent::WindowResized { width: _, height: _ } => {
                    draw_pixels(&wnd);
                }
                WndEvent::WindowClosed => running = false,
                _ => ()
            }
        }
    }
}

fn draw_pixels(wnd: &WindowHandler) {
    let (_, _, w, h) = wnd.get_client_rect();
    
    wnd.resize_buffer(w, h);
    
    if DIRECT {
        let mut temp_buffer = vec![0; (4 * w * h) as usize];

        for y in 0..h {
            for x in 0..w {
                temp_buffer[(4 * (w * y + x) + 2) as usize] = (x % 255) as u8; // r
                temp_buffer[(4 * (w * y + x) + 1) as usize] = (y % 255) as u8; // g
                temp_buffer[(4 * (w * y + x)) as usize] = 200; // b
            }
        }
        wnd.set_buffer_direct(temp_buffer);
        wnd.draw_buffer();
    } else {
        let mut temp_buffer = vec![0; (3 * w * h) as usize];

        for y in 0..h {
            for x in 0..w {
                temp_buffer[3 * (w * y + x) as usize] = (x % 255) as u8; // r
                temp_buffer[3 * (w * y + x) as usize + 1] = (y % 255) as u8; // g
                temp_buffer[3 * (w * y + x) as usize + 2] = 200;  // b
            }
        }
        wnd.set_buffer(temp_buffer);
        wnd.draw_buffer();
    }
}