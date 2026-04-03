use std::sync::mpsc;

use crate::{
    events::*,
    handler::linux::linux_image_buffer::LinuxImageBuffer,
    requests::WndRequest,
    traits::{ImageBuffer, Window}
};

pub struct LinuxWindow {
    req_receiver: mpsc::Receiver<WndRequest>,
    evt_sender: mpsc::Sender<WndEvent>,
    image_buffer: LinuxImageBuffer,
    running: bool,
}

impl LinuxWindow {
    pub fn new(
        title: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        id: usize,
        req_receiver: mpsc::Receiver<WndRequest>,
        evt_sender: mpsc::Sender<WndEvent>,
    ) -> Self {
    

        let mut image_buffer = LinuxImageBuffer::default();
        image_buffer.init(width, height);

        LinuxWindow {
            req_receiver,
            evt_sender,
            image_buffer,
            running: true
        }
    }

    pub fn start(&mut self) {
        self.run();
    }
}

impl Window for LinuxWindow {
    fn run(&mut self) {
        todo!()
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

impl Drop for LinuxWindow {
    fn drop(&mut self) {
        self.close();
    }
}
