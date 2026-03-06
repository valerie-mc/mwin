mod microsoft;
mod linux;

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{self, Receiver, Sender}
    },
    thread
};

use crate::{
    errors::WindowError,
    traits::window::Window,
    handler::microsoft::ms_window::MSWindow,
    messaging::{events::*, requests::WndRequest}
};

// TODO: This is where you should add documentation (actually, maybe just in WindowHandler)

pub struct WindowHandler {
    req_sender: Sender<WndRequest>,
    evt_receiver: Receiver<WndEvent>,
}

impl WindowHandler {
    pub fn new(title: &str, x: i32, y: i32, width: i32, height: i32) -> Result<Self, WindowError> {
        let title = title.to_string();

        static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        let (req_sender, req_receiver) = mpsc::channel::<WndRequest>();
        let (evt_sender, evt_receiver) = mpsc::channel::<WndEvent>();

        match std::env::consts::OS {
            "windows" => {
                thread::spawn(move || {
                    MSWindow::new(
                        title, x, y, width, height, 
                        id, req_receiver, evt_sender
                    ).run()
                })
            },
            // "linux" => thread::spawn(move || { WindowLinux::new(title, req_receiver).run() }),
            _ => return Err(WindowError::ERROR_UNSUPPORTED_OS),
        };

        Ok(WindowHandler {
            req_sender,
            evt_receiver,
        })
    }

    // * Events * //
    #[inline]
    pub fn get_wnd_events(&mut self) -> Vec<WndEvent> {
        self.evt_receiver.try_iter().collect()
    }

    // * Requests * //
    #[inline]
    fn send_request<T>(&self, req: WndRequest, recv: Receiver<T>) -> T {
        let _ = self.req_sender.send(req);
        recv.recv().unwrap() // If the sender is dropped, this returns an error, do I have to make all of these a result/option now?
    }

    // * Getters * //
    // X pos, Y pos, Width, Height (includes header and border)
    pub fn get_wnd_rect(&self) -> (i32, i32, i32, i32) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetWndRect { rtrn };
        self.send_request(req, recv)
    }
    // X pos, Y pos, Width, Height (only includes client area)
    pub fn get_client_rect(&self) -> (i32, i32, i32, i32) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetClientRect { rtrn };
        self.send_request(req, recv)
    }

    pub fn get_cursor_pos(&self) -> (i32, i32) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetCursorPos { rtrn };
        self.send_request(req, recv)
    }
    pub fn get_cursor_client_pos(&self) -> (i32, i32) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetCursorClientPos { rtrn };
        self.send_request(req, recv)
    }

    pub fn is_visible(&self) -> bool {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::IsVisible { rtrn };
        self.send_request(req, recv)
    }
    pub fn is_focused(&self) -> bool {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::IsFocused { rtrn };
        self.send_request(req, recv)
    }

    // * Setters * //
    pub fn set_wnd_pos(&self, x: i32, y: i32) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetWndPos { args: (x, y), rtrn };
        self.send_request(req, recv)
    }
    pub fn set_wnd_size(&self, width: i32, height: i32) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetWndSize { args: (width, height), rtrn };
        self.send_request(req, recv)
    }
    pub fn set_wnd_pos_and_size(&self, x: i32, y: i32, width: i32, height: i32) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetWndPosAndSize { args: (x, y, width, height), rtrn };
        self.send_request(req, recv)
    }

    pub fn set_visibility(&self, visible: bool) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetVisibility { args: visible, rtrn };
        self.send_request(req, recv)
    }
    pub fn minimize(&self) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::Minimize { rtrn };
        self.send_request(req, recv)
    }
    pub fn maximize(&self) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::Maximize { rtrn };
        self.send_request(req, recv)
    }
    pub fn close(&self) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::Close { rtrn };
        self.send_request(req, recv)
    }

    // * Drawing * //
    pub fn draw_buffer(&self) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::DrawBuffer { rtrn };
        self.send_request(req, recv)
    }

    // * NOTE Only resizes if the buffer would be a different size
    pub fn resize_buffer(&self, width: i32, height: i32) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::ResizeBuffer { args: (width, height), rtrn };
        self.send_request(req, recv)
    }
    pub fn clear_buffer(&self) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::ClearBuffer { rtrn };
        self.send_request(req, recv)
    }

    // Expects a buffer of [r_1, g_1, b_2, r_2, g_2, b_2, ...]
    pub fn set_buffer(&self, buffer: Vec<u8>) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetBuffer { args: buffer, rtrn };
        self.send_request(req, recv)
    }
    // Directly sets the image buffer, this is os dependent, but faster
    pub fn set_buffer_direct(&self, buffer: Vec<u8>) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetBufferDirect { args: buffer, rtrn };
        self.send_request(req, recv)
    }
}
