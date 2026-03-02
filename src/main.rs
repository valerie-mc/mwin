pub mod errors;
pub mod handler;
pub mod messaging;
pub mod traits;

use crate::messaging::events::{KeyCode, WndEvent};
use crate::handler::WindowHandler;

// use std::thread;
// use std::time;


const W: i32 = 256 * 4;
const H: i32 = 256 * 4;

fn main() {

    let mut wnd = WindowHandler::new("Window", 500, 500, W, H).unwrap();

    let mut running = true;

    // TODO: There is a bug where sometimes the window just doesn't even open
    // TODO: Window freezes when I press a key (could be something with my changes to events)

    println!("running");

    let mut temp_buffer = vec![0; (4 * W * H) as usize];

    println!("drawing pixels");
    for x in 0..W-1 {
        for y in 0..H-1 {
            // wnd.set_pixel(x, y, (x % 255) as u8, (y % 255) as u8, 200);
            temp_buffer[4 * (W * y + x) as usize + 2] = (x % 255) as u8;
            temp_buffer[4 * (W * y + x) as usize + 1] = (y % 255) as u8;
            temp_buffer[4 * (W * y + x) as usize] = 200;
        }
    }
    wnd.set_buffer(temp_buffer);
    wnd.draw_buffer();
    println!("finished drawing pixels");

    while running {
        for event in wnd.get_wnd_events() {
            println!("{:?}", event);
            match event {
                WndEvent::KeyboardInput { event } => {
                    match event.key {
                        KeyCode::D => draw_pixels(&wnd),
                        KeyCode::E => wnd.clear_buffer(),
                        KeyCode::Q => running = false,
                        _ => ()
                    }
                }
                _ => ()
            }
        }
    }
}

fn draw_pixels(wnd: &WindowHandler) {
    println!("drawing pixels");
    for x in 0..W-1 {
        for y in 0..H-1 {
            wnd.set_pixel(x, y, (x % 255) as u8, (y % 255) as u8, 200);
        }
    }
    wnd.draw_buffer();
    println!("finished drawing pixels");
}