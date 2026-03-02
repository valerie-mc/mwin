mod microsoft;
mod linux;

use std::{
    collections::vec_deque::{Iter, VecDeque},
    sync::{
        atomic::{AtomicUsize, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex
    },
    thread
};

use crate::{
    errors::*,
    traits::{image_buffer::ImageBuffer, window::Window},
    handler::microsoft::{ms_image_buffer::MSImageBuffer, ms_window::MSWindow},
    messaging::{events::*, requests::*}
};

// TODO: This is where you should add documentation (actually, maybe just in WindowHandler)

pub struct WindowHandler {
    req_sender: Sender<WndRequest>,
    evt_receiver: Receiver<WndEvent>,
    window_events: VecDeque<WndEvent>,
    image_buffer: Arc<Mutex<dyn ImageBuffer + Send>>,
}

impl WindowHandler {
    pub fn new(title: &str, x: i32, y: i32, width: i32, height: i32) -> Result<Self, WindowError> {
        let title = title.to_string();

        static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        let (req_sender, req_receiver) = mpsc::channel::<WndRequest>();
        let (evt_sender, evt_receiver) = mpsc::channel::<WndEvent>();

        let image_buffer: Arc<Mutex<dyn ImageBuffer + Send>>;

        match std::env::consts::OS {
            "windows" => {
                image_buffer = Arc::new(Mutex::new(MSImageBuffer::default()));
                let shared_image_buffer = image_buffer.clone();

                thread::spawn(move || {
                    MSWindow::new(
                        title, x, y, width, height, 
                        id, evt_sender, req_receiver, shared_image_buffer
                    ).run()
                })
            },
            // "linux" => thread::spawn(move || { WindowLinux::new(title, req_receiver).run() }),
            _ => return Err(WindowError::ERROR_UNSUPPORTED_OS),
        };

        Ok(WindowHandler {
            req_sender,
            evt_receiver,
            window_events: VecDeque::new(),
            image_buffer,
        })
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
    pub fn is_mouse_captured(&self) -> bool {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::IsMouseCaptured { rtrn };
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

    pub fn capture_mouse(&self) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::CaptureMouse { rtrn };
        self.send_request(req, recv)
    }
    pub fn release_mouse(&self) {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::ReleaseMouse { rtrn };
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

    // * Events * //
    #[inline]
    fn poll_window_events(&mut self) {
        for wnd_event in self.evt_receiver.try_iter() {
            self.window_events.push_back(wnd_event);
        }
    }
    
    pub fn pop_wnd_event(&mut self) -> Option<WndEvent> {
        self.poll_window_events();
        self.window_events.pop_front()
    }
    pub fn wnd_event_iter(&mut self) -> Iter<'_, WndEvent> {
        self.poll_window_events();
        self.window_events.iter()
    }

    // * Image Buffer * //
    pub fn resize_buffer(&self) {
        let (_, _, width, height) = self.get_client_rect();

        let mut image_buffer = self.image_buffer.lock().unwrap();
        image_buffer.resize_buffer(width, height);
    }

    pub fn clear_buffer(&self) {
        let mut image_buffer = self.image_buffer.lock().unwrap();
        image_buffer.clear_buffer();
    }

    pub fn set_pixel(&self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        let mut image_buffer = self.image_buffer.lock().unwrap();
        image_buffer.set_pixel(x, y, r, g, b);
    }

    // TODO: Fn to draw buffer on screen
}
