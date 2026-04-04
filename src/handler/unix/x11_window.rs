use std::sync::mpsc;

use std::error::Error;

use x11rb::connection::Connection;
use x11rb::errors::{ConnectionError, ReplyError, ReplyOrIdError};
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::COPY_DEPTH_FROM_PARENT;


use crate::{
    WindowError,
    events::*,
    handler::unix::x11_image_buffer::X11ImageBuffer,
    requests::WndRequest,
    traits::{ImageBuffer, Window}
};

pub struct X11Window {
    req_receiver: mpsc::Receiver<WndRequest>,
    evt_sender: mpsc::Sender<WndEvent>,
    image_buffer: X11ImageBuffer,
    running: bool,
}

// Some code has been adapted from https://github.com/psychon/x11rb/blob/master/x11rb/examples/tutorial.rs

impl X11Window {
    pub fn new(
        title: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        req_receiver: mpsc::Receiver<WndRequest>,
        evt_sender: mpsc::Sender<WndEvent>,
    ) -> Result<Self, WindowError> {
        // Open the connection to the X server. Use the DISPLAY environment variable.
        let (conn, screen_num) = x11rb::connect(None).unwrap();

        // Get the screen from the screen_num
        let screen = &conn.setup().roots[screen_num];

        // Ask for our window's Id
        let win = conn.generate_id().unwrap();

        // Create the window
        conn.create_window(
            COPY_DEPTH_FROM_PARENT,    // depth (same as root)
            win,                       // window Id
            screen.root,               // parent window
            0,                         // x
            0,                         // y
            150,                       // width
            150,                       // height
            10,                        // border width
            WindowClass::INPUT_OUTPUT, // class
            screen.root_visual,        // visual
            &Default::default(),
        ).unwrap(); // masks, not used yet

        // Map the window on the screen
        conn.map_window(win).unwrap();

        // Make sure commands are sent before the sleep, so window is shown
        conn.flush().unwrap();

        std::thread::sleep(std::time::Duration::from_secs(10));

        let mut image_buffer = X11ImageBuffer::default();
        image_buffer.init(width, height);

        Ok(X11Window {
            req_receiver,
            evt_sender,
            image_buffer,
            running: true
        })
    }

    pub fn start(&mut self) {
        self.run();
    }
}

impl Window for X11Window {
    fn run(&mut self) {
        while self.running {
            if let Ok(req) = self.req_receiver.try_recv() {
                self.handle_request(req);
            }
        }
    }

    fn get_wnd_rect(&self) -> (i32, i32, i32, i32) {
        todo!()
    }

    fn get_wnd_size(&self) -> (i32, i32) {
        todo!()
    }

    fn get_cursor_pos(&self) -> (i32, i32) {
        todo!()
    }

    fn get_cursor_client_pos(&self) -> (i32, i32) {
        todo!()
    }

    fn is_visible(&self) -> bool {
        todo!()
    }

    fn is_focused(&self) -> bool {
        todo!()
    }

    fn set_wnd_pos(&self, x: i32, y: i32) {
        todo!()
    }

    fn set_wnd_size(&self, width: i32, height: i32) {
        todo!()
    }

    fn set_wnd_pos_and_size(&self, x: i32, y: i32, width: i32, height: i32) {
        todo!()
    }

    fn set_visibility(&self, visible: bool) {
        todo!()
    }

    fn minimize(&self) {
        todo!()
    }

    fn maximize(&self) {
        todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn draw_buffer(&self) {
        todo!()
    }

    fn resize_buffer(&mut self, width: i32, height: i32) {
        todo!()
    }

    fn clear_buffer(&mut self) {
        todo!()
    }

    fn set_buffer(&mut self, buffer: Vec<u8>) {
        todo!()
    }

    fn set_buffer_direct(&mut self, buffer: Vec<u8>) {
        todo!()
    }
}

impl Drop for X11Window {
    fn drop(&mut self) {
        self.close();
    }
}
