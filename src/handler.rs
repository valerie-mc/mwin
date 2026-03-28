mod microsoft;
mod linux;

use std::sync::{atomic, mpsc};

use crate::{
    errors::WindowError,
    events::WndEvent,
    requests::WndRequest,
    handler::microsoft::ms_window,
};

// TODO: This is where you should add documentation (actually, maybe just in WindowHandler)

/// A handler for a window.
/// 
/// When initalized, the [`WindowHandler`] creates a concrete window struct
/// (which is dependent on the operating system) in a new thread. The
/// [`WindowHandler`] can be used to send [`WndRequest`]s to the window
/// (eg. [`get_client_rect`] or [`set_wnd_pos`]) and receive [`WndEvent`]s from
/// the window (eg. [`KeyboardInput`] or [`WindowMinimized`]).
/// 
/// Currently, the only supported operating system is Windows. Trying to create
/// a [`WindowHandler`] on an unsupported operating system will return
/// [`WindowError::UnsupportedOS`].
/// 
/// Note: If the [`WindowHandler`] is dropped, the associated window will be
/// closed.
/// 
/// # Examples
/// ```
/// use mwin::handler::WindowHandler;
/// 
/// 
/// let mut window = WindowHandler::new("My Window", 0, 0, 500, 500)
///     .expect("Operating system is unsupported.");
/// 
/// window.maximize()
/// ```
pub struct WindowHandler {
    req_sender: mpsc::Sender<WndRequest>,
    evt_receiver: mpsc::Receiver<WndEvent>,
}

impl WindowHandler {
    pub fn new(title: &str, x: i32, y: i32, width: i32, height: i32) -> Result<Self, WindowError> {
        let title = title.to_string();

        static ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
        let id = ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed);

        let (req_sender, req_receiver) = mpsc::channel::<WndRequest>();
        let (evt_sender, evt_receiver) = mpsc::channel::<WndEvent>();

        match std::env::consts::OS {
            "windows" => {
                std::thread::spawn(move || {
                    ms_window::MSWindowContainer::new(
                        title, x, y, width, height, 
                        id, req_receiver, evt_sender
                    ).run()
                })
            },
            // "linux" => thread::spawn(move || { WindowLinux::new(title, req_receiver).run() }),
            _ => return Err(WindowError::UnsupportedOS),
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

    pub fn get_wnd_event(&mut self) -> Option<WndEvent> {
        // TODO: Probably could just return a result
        if let Ok(event) = self.evt_receiver.try_recv() {
            return Some(event);
        } else {
            return None;
        }
    }

    // * Requests * //
    #[inline]
    fn send_request<T>(&self, req: WndRequest, recv: mpsc::Receiver<T>) -> Result<T, WindowError> {
        self.req_sender.send(req).map_err(|_e| WindowError::WindowClosed)?;
        // recv.recv().map_err(|_e| WindowError::WindowClosed);

        match recv.recv() {
            Ok(res) => Ok(res),
            Err(_e) => Err(WindowError::WindowClosed),
        }
    }

    // * Getters * //
    // X pos, Y pos, Width, Height (includes header and border)
    pub fn get_wnd_rect(&self) -> Result<(i32, i32, i32, i32), WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetWndRect { rtrn };
        self.send_request(req, recv)
    }
    // X pos, Y pos, Width, Height (only includes client area)
    pub fn get_client_rect(&self) -> Result<(i32, i32, i32, i32), WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetClientRect { rtrn };
        self.send_request(req, recv)
    }

    pub fn get_cursor_pos(&self) -> Result<(i32, i32), WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetCursorPos { rtrn };
        self.send_request(req, recv)
    }
    pub fn get_cursor_client_pos(&self) -> Result<(i32, i32), WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetCursorClientPos { rtrn };
        self.send_request(req, recv)
    }

    pub fn is_visible(&self) -> Result<bool, WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::IsVisible { rtrn };
        self.send_request(req, recv)
    }
    pub fn is_focused(&self) -> Result<bool, WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::IsFocused { rtrn };
        self.send_request(req, recv)
    }

    // * Setters * //
    pub fn set_wnd_pos(&self, x: i32, y: i32) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetWndPos { args: (x, y), rtrn };
        self.send_request(req, recv).err()
    }
    pub fn set_wnd_size(&self, width: i32, height: i32) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetWndSize { args: (width, height), rtrn };
        self.send_request(req, recv).err()
    }
    // TODO: Remove
    pub fn set_wnd_pos_and_size(&self, x: i32, y: i32, width: i32, height: i32) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetWndPosAndSize { args: (x, y, width, height), rtrn };
        self.send_request(req, recv).err()
    }

    pub fn set_visibility(&self, visible: bool) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetVisibility { args: visible, rtrn };
        self.send_request(req, recv).err()
    }
    pub fn minimize(&self) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::Minimize { rtrn };
        self.send_request(req, recv).err()
    }
    pub fn maximize(&self) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::Maximize { rtrn };
        self.send_request(req, recv).err()
    }
    pub fn close(&self) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::Close { rtrn };
        self.send_request(req, recv).err()
    }

    // * Drawing * //
    pub fn draw_buffer(&self) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::DrawBuffer { rtrn };
        self.send_request(req, recv).err()
    }

    pub fn resize_buffer(&self, width: i32, height: i32) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::ResizeBuffer { args: (width, height), rtrn };
        self.send_request(req, recv).err()
    }
    pub fn clear_buffer(&self) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::ClearBuffer { rtrn };
        self.send_request(req, recv).err()
    }

    // Expects a buffer of [r_1, g_1, b_2, r_2, g_2, b_2, ...]
    pub fn set_buffer(&self, buffer: Vec<u8>) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetBuffer { args: buffer, rtrn };
        self.send_request(req, recv).err()
    }
    // Directly sets the image buffer, this is os dependent, but faster
    pub fn set_buffer_direct(&self, buffer: Vec<u8>) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetBufferDirect { args: buffer, rtrn };
        self.send_request(req, recv).err()
    }
}

// Closes window when dropped
impl Drop for WindowHandler {
    fn drop(&mut self) {
        self.close();
    }
}
