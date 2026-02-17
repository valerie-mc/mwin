pub mod errors;
pub mod handler;
pub mod messaging;

use crate::{handler::*, messaging::events::*};
// use core::time;
// use std::thread;

fn main() {
    let mut wnd = WindowHandler::new("My Window").unwrap();

    let mut running = true;

    while running {
        while let Some(wnd_event) = wnd.pop_wnd_event() {
            match wnd_event {
                WndEvent::KeyboardInput { event } => handle_keyboard_input(event, &mut running),

                // TODO: These aren't working (taking a break for now)
                WndEvent::WindowMoved => println!("window moved"),
                WndEvent::WindowResized => println!("window resized"),
                WndEvent::WindowMaximized => println!("window maximized"),
                WndEvent::WindowMinimzed => println!("window minimized"),
                WndEvent::WindowClosed => println!("window closed"),
                _ => (),
            }
        }
    }
}

fn handle_keyboard_input(event: KeyEvent, running: &mut bool) {
    match event.key {
        KeyCode::A => println!("A was {:?}", event.state),
        KeyCode::Q => *running = false,
        _ => (),
    }
}
