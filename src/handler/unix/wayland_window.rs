use std::sync::mpsc;

use crate::{
    WindowError,
    events::*,
    handler::unix::wayland_image_buffer::WaylandImageBuffer,
    requests::WndRequest,
    traits::{ImageBuffer, Window}
};

pub struct WaylandWindow {
    req_receiver: mpsc::Receiver<WndRequest>,
    evt_sender: mpsc::Sender<WndEvent>,
    image_buffer: WaylandImageBuffer,
    running: bool,
}

impl WaylandWindow {
    pub fn new(
        title: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        req_receiver: mpsc::Receiver<WndRequest>,
        evt_sender: mpsc::Sender<WndEvent>,
    ) -> Result<Self, WindowError> {
        let mut image_buffer = WaylandImageBuffer::default();
        image_buffer.init(width, height);

        Ok(WaylandWindow {
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

impl Window for WaylandWindow {
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

impl Drop for WaylandWindow {
    fn drop(&mut self) {
        self.close();
    }
}
