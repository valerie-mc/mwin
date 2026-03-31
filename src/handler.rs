mod microsoft;
mod linux;

use std::sync::{atomic, mpsc};

use crate::{
    WindowError,
    events::WndEvent,
    requests::WndRequest,
    handler::microsoft::ms_window,
};

/// A handler for a window.
/// 
/// When initalized, the [`WindowHandler`] creates a concrete window struct
/// (which is dependent on the operating system) in a new thread. The
/// [`WindowHandler`] can be used to send requests to the window
/// (eg. `get_client_rect` or `set_wnd_pos`) and receive [`WndEvent`]s from
/// the window (eg. [`WndEvent::KeyboardInput`] or [`WndEvent::WindowMinimized`]).
/// 
/// Currently, the only supported operating system is Windows. Trying to create
/// a [`WindowHandler`] on an unsupported operating system will return
/// [`WindowError::UnsupportedOS`]. All [`WindowHandler`] methods, except `new`,
/// will return [`WindowError::WindowClosed`] if associated window was closed.
/// 
/// Note: If the [`WindowHandler`] is dropped, the associated window will be
/// closed.
/// 
/// # Examples
/// ```
/// use mwin::{events, WindowHandler};
/// 
/// // Creates a new window with the title "My Window", at (0, 0) with a size of 500 by 500.
/// let window = WindowHandler::new("My Window", 0, 0, 500, 500)
///     .expect("Current operating system is unsupported.");
/// 
/// let mut run = true;
/// while run {
///     // Iterates through a Vec<WndEvent>.
///     for wnd_event in window.get_wnd_events() {
///         match wnd_event {
///             // Stops running when the window is closed.
///             events::WndEvent::WindowClosed => run = false,
///             events::WndEvent::KeyboardInput { key_event } => {
///                 match key_event.key {
///                     // Closes the window and stops running.
///                     events::KeyCode::Q => {
///                         run = false;
///                         window.close();
///                     }
///                     events::KeyCode::H => {
///                         // Prints "Hello World" only when 'H' is Pressed and 'Shift' is held.
///                         if key_event.state == events::KeyState::Pressed && 
///                            key_event.modifiers.contains(events::Modifiers::SHIFT) {
///                             println!("Hello World");
///                         }
///                     }
///                     _ => (),
///                 }
///             }
///             _ => (),
///         }
///     }
/// }
/// ```
/// The [`WindowHandler`] can also be used to draw to the window.
/// ```
/// use std::{thread, time::Duration};
/// use mwin::WindowHandler;
/// 
/// // Creates a new window with the title "My Window", at (0, 0) with a size of 500 by 500.
/// let window = WindowHandler::new("My Window", 0, 0, 500, 500)
///     .expect("Current operating system is unsupported.");
/// 
/// // Creates a buffer of 500 x 500 white pixels. 
/// let buffer: Vec<u8> = vec![255; 3 * (500 * 500)];
/// 
/// window.set_buffer(buffer);
/// window.draw_buffer();
/// 
/// // Prevents the WindowHandler from being dropped immediately and closing the window.
/// thread::sleep(Duration::from_secs(5));
/// ```
/// To note, [`Self::set_buffer`] can safely be used on any supported os, but is slower
/// than using [`Self::set_buffer_direct`] because it converts the given buffer to the
/// format expected by the current os. See [`Self::set_buffer_direct`] for more information.
#[derive(Debug)]
pub struct WindowHandler {
    req_sender: mpsc::Sender<WndRequest>,
    evt_receiver: mpsc::Receiver<WndEvent>,
}

impl WindowHandler {
    /// Creates a new [`WindowHandler`].
    /// 
    /// When created, the [`WindowHandler`] creates a window in a new thread with
    /// the given title, position, and size. Returns [`WindowError::UnsupportedOS`]
    /// error if the current os is unsupported.
    /// 
    /// Note: The image buffer of the window is set to the given size of the window
    /// by default.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("My Window", 100, 100, 1000, 500)
    ///     .expect("Current operating system is unsupported.");
    /// ```
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
            _ => return Err(WindowError::UnsupportedOS),
        };

        Ok(WindowHandler {
            req_sender,
            evt_receiver,
        })
    }

    // * Events * //
    /// Returns a [`Vec<WndEvent>`] that the [`WindowHandler`] has recieved from 
    /// the window.
    /// 
    /// [`WndEvent`]s are not saved by the [`WindowHandler`], as such, when 
    /// [`Self::get_wnd_events`] is called, the returned [`WndEvent`]s are removed
    /// from the [`WindowHandler`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::{events::WndEvent, WindowHandler};
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// for event in wnd.get_wnd_events() {
    ///     match event {
    ///         WndEvent::WindowClosed => println!("Window closed."),
    ///         _ => (),
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn get_wnd_events(&self) -> Vec<WndEvent> {
        self.evt_receiver.try_iter().collect()
    }

    /// Returns the first [`WndEvent`] that the [`WindowHandler`] has recieved from 
    /// the window, or [`None`] if there are no [`WndEvent`]s.
    /// 
    /// [`WndEvent`]s are not saved by the [`WindowHandler`], as such, when 
    /// [`Self::get_wnd_event`] is called, the returned [`WndEvent`] is removed
    /// from the [`WindowHandler`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::{events::WndEvent, WindowHandler};
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// if let Some(event) = wnd.get_wnd_event() {
    ///     match event {
    ///         WndEvent::WindowClosed => println!("Window closed."),
    ///         _ => (),
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn get_wnd_event(&self) -> Option<WndEvent> {
        self.evt_receiver.try_recv().ok()
    }

    // * Requests * //
    #[inline]
    fn send_request<T>(&self, req: WndRequest, recv: mpsc::Receiver<T>) -> Result<T, WindowError> {
        self.req_sender.send(req).map_err(|_e| WindowError::WindowClosed)?;
        recv.recv().map_err(|_e| WindowError::WindowClosed)
    }

    // * Getters * //
    /// Returns the window's rect.
    /// 
    /// Returns the (x_pos, y_pos, width, height) of the full window (including
    /// the top window bar and border), where (x_pos, y_pos) is the position of 
    /// the top left corner of the window relative to the top left corner of the
    /// screen. Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 250, 500, 750)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// assert_eq!((0, 250, 500, 750), wnd.get_wnd_rect().expect("Window shouldn't be closed."));
    /// ```
    pub fn get_wnd_rect(&self) -> Result<(i32, i32, i32, i32), WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetWndRect { rtrn };
        self.send_request(req, recv)
    }
    
    /// Returns the size of the window's client area.
    /// 
    /// Returns the (width, height) of the client area of the window, this
    /// excludes the top window bar and border. Returns [`WindowError::WindowClosed`]
    /// if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 250, 500, 750)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// // Exact size of the boarder depends on the os, this represents Windows' border.
    /// assert_eq!((482, 706), wnd.get_wnd_size().expect("Window shouldn't be closed."));
    /// ```
    pub fn get_wnd_size(&self) -> Result<(i32, i32), WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetWndSize { rtrn };
        self.send_request(req, recv)
    }

    /// Returns the cursor's position relative to the screen.
    /// 
    /// Returns the (x_pos, y_pos) of the cursor's position or [`WindowError::WindowClosed`]
    /// if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// let (x, y) = wnd.get_cursor_pos().expect("Window shouldn't be closed.");
    /// ```
    pub fn get_cursor_pos(&self) -> Result<(i32, i32), WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetCursorPos { rtrn };
        self.send_request(req, recv)
    }
    
    /// Returns the cursor's position relative to the window.
    /// 
    /// Returns the (x_pos, y_pos) of the cursor's position relative to the
    /// client rect of the window or [`WindowError::WindowClosed`] if the window
    /// was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// let (x, y) = wnd.get_cursor_client_pos().expect("Window shouldn't be closed.");
    /// ```
    pub fn get_cursor_client_pos(&self) -> Result<(i32, i32), WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::GetCursorClientPos { rtrn };
        self.send_request(req, recv)
    }

    /// Returns `true` if the window is visible or [`WindowError::WindowClosed`]
    /// if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// assert!(wnd.is_visible().expect("Window shouldn't be closed."));
    /// ```
    pub fn is_visible(&self) -> Result<bool, WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::IsVisible { rtrn };
        self.send_request(req, recv)
    }
    
    /// Returns `true` if the window is focused or [`WindowError::WindowClosed`]
    /// if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// assert!(wnd.is_focused().expect("Window shouldn't be closed."));
    /// ```
    pub fn is_focused(&self) -> Result<bool, WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::IsFocused { rtrn };
        self.send_request(req, recv)
    }

    // * Setters * //
    /// Sets the position of the window.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// wnd.set_wnd_pos(100, 100);
    /// assert_eq!((100, 100, 500, 500), wnd.get_wnd_rect().expect("Window shouldn't be closed."));
    /// ```
    pub fn set_wnd_pos(&self, x: i32, y: i32) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetWndPos { args: (x, y), rtrn };
        self.send_request(req, recv).err()
    }
    /// Sets the size of the window.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// wnd.set_wnd_size(750, 750);
    /// assert_eq!((0, 0, 750, 750), wnd.get_wnd_rect().expect("Window shouldn't be closed."));
    /// ```
    pub fn set_wnd_size(&self, width: i32, height: i32) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetWndSize { args: (width, height), rtrn };
        self.send_request(req, recv).err()
    }
    /// Sets the position and size of the window.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// wnd.set_wnd_pos_and_size(100, 100, 750, 750);
    /// assert_eq!((100, 100, 750, 750), wnd.get_wnd_rect().expect("Window shouldn't be closed."));
    /// ```
    pub fn set_wnd_pos_and_size(&self, x: i32, y: i32, width: i32, height: i32) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetWndPosAndSize { args: (x, y, width, height), rtrn };
        self.send_request(req, recv).err()
    }

    /// Sets the visibility of the window.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// wnd.set_visibility(false);
    /// assert!(!wnd.is_visible().expect("Window shouldn't be closed."));
    /// ```
    pub fn set_visibility(&self, visible: bool) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetVisibility { args: visible, rtrn };
        self.send_request(req, recv).err()
    }
    
    /// Minimizes the window.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::{events::WndEvent, WindowHandler};
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// wnd.minimize();
    /// assert_eq!(Some(WndEvent::WindowMinimized), wnd.get_wnd_event());
    /// ```
    pub fn minimize(&self) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::Minimize { rtrn };
        self.send_request(req, recv).err()
    }
    
    /// Maximizes the window.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::{events::WndEvent, WindowHandler};
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// wnd.maximize();
    /// let (width, height) = wnd.get_wnd_size().expect("Window shouldn't be closed.");
    /// assert_eq!(Some(WndEvent::WindowMaximized { width, height }), wnd.get_wnd_event());
    /// ```
    pub fn maximize(&self) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::Maximize { rtrn };
        self.send_request(req, recv).err()
    }
    
    /// Closes the window.
    /// 
    /// For clarification, a "closed" window is the same thing as a terminated 
    /// or destroyed window.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::{thread, time::Duration};
    /// use mwin::{events::WndEvent, WindowHandler};
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// wnd.close();
    /// thread::sleep(Duration::from_millis(100)); // Gives the WindowHandler time to recieve the event.
    /// assert_eq!(Some(WndEvent::WindowClosed), wnd.get_wnd_event());
    /// ```
    pub fn close(&self) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::Close { rtrn };
        self.send_request(req, recv).err()
    }

    // * Drawing * //
    /// Draws the image buffer to the client portion of the window.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// Note: On Windows, the window cannot be drawn to while it is being moved
    /// or resized (this is due to Windows entering a modal loop when the user
    /// moves or resizes the window).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// // Creates a buffer of 500 x 500 white pixels. 
    /// let buffer: Vec<u8> = vec![255; 3 * (500 * 500)];
    /// 
    /// wnd.set_buffer(buffer);
    /// wnd.draw_buffer();
    /// ```
    pub fn draw_buffer(&self) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::DrawBuffer { rtrn };
        self.send_request(req, recv).err()
    }

    /// Resizes the image buffer.
    /// 
    /// This is most useful for resizing the image buffer in response to the
    /// user resizing the window.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// Note: The image buffer is set to the given size of the window by default.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// // Creates a buffer of 500 x 500 white pixels. 
    /// let buffer: Vec<u8> = vec![255; 3 * (500 * 500)];
    /// 
    /// wnd.set_buffer(buffer);
    /// wnd.draw_buffer();
    /// 
    /// // Simulates a user resizing the window.
    /// wnd.set_wnd_size(750, 750);
    /// 
    /// let (width, height) = wnd.get_wnd_size().expect("Window shouldn't be closed.");
    /// wnd.resize_buffer(width, height);
    /// wnd.draw_buffer();
    /// ```
    pub fn resize_buffer(&self, width: i32, height: i32) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::ResizeBuffer { args: (width, height), rtrn };
        self.send_request(req, recv).err()
    }
    
    /// Clears the image buffer.
    /// 
    /// This sets the image buffer to be a vector a black pixels, while still
    /// retaining the previous size.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// Note: The image buffer is set to the given size of the window by default.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// // Creates a buffer of 500 x 500 white pixels. 
    /// let buffer: Vec<u8> = vec![255; 3 * (500 * 500)];
    /// 
    /// wnd.set_buffer(buffer);
    /// wnd.draw_buffer();
    /// 
    /// wnd.clear_buffer(); // The image buffer is now a buffer of 500 x 500 black pixels.
    /// wnd.draw_buffer();
    /// ```
    pub fn clear_buffer(&self) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::ClearBuffer { rtrn };
        self.send_request(req, recv).err()
    }

    /// Sets the image buffer to the given buffer.
    /// 
    /// This function expects the given buffer to be in the following format:
    ///     `[r1, g1, b1, r2, g2, b2, r3, g3, b3 ...]`
    /// with a size of `3 * width * height` where `r`, `g`, and `b` refer to the
    /// red, green, and blue colour values of each pixel.
    /// 
    /// This function converts the given buffer to the format expected by the os,
    /// as such, it works on all supported os's, but its overhead is slight higher.
    /// If performance is an issue, use [`Self::set_buffer_direct`] instead.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// // Creates a buffer of 500 x 500 pink pixels.
    /// let mut buffer: Vec<u8> = vec![0; 3 * (500 * 500)];
    /// 
    /// for pixel in 0..(500 * 500) {
    ///     buffer[3 * pixel]     = 225; // Red
    ///     buffer[3 * pixel + 1] = 150; // Green
    ///     buffer[3 * pixel + 2] = 240; // Blue
    /// }
    /// 
    /// wnd.set_buffer(buffer);
    /// wnd.draw_buffer();
    /// ```
    pub fn set_buffer(&self, buffer: Vec<u8>) -> Option<WindowError> {
        let (rtrn, recv) = mpsc::channel();
        let req = WndRequest::SetBuffer { args: buffer, rtrn };
        self.send_request(req, recv).err()
    }
    
    /// Sets the image buffer to the given buffer.
    /// 
    /// Unlike [`Self::set_buffer`], this function directly sets the image buffer, as
    /// such, this function is faster than [`Self::set_buffer`], but the given buffer
    /// must be in the format expected by the os.
    /// 
    /// On Windows, the expected format is:
    ///     `[b1, g1, r1, 0, b2, g2, r2, 0, b3, g3, r3, 0 ...]`
    /// with a size of `4 * width * height` where `r`, `g`, and `b` refer to the
    /// red, green, and blue colour values of each pixel.
    /// 
    /// Returns [`WindowError::WindowClosed`] if the window was closed.
    /// 
    /// # Examples
    /// 
    /// Setting the buffer directly on Windows.
    /// ```
    /// use mwin::WindowHandler;
    /// let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
    ///     .expect("Current operating system is unsupported.");
    /// 
    /// // Creates a buffer of 500 x 500 pink pixels (on Windows).
    /// let mut buffer: Vec<u8> = vec![0; 4 * (500 * 500)];
    /// 
    /// for pixel in 0..(500 * 500) {
    ///     buffer[4 * pixel]     = 240; // Blue
    ///     buffer[4 * pixel + 1] = 150; // Green
    ///     buffer[4 * pixel + 2] = 225; // Red
    ///     buffer[4 * pixel + 3] = 0;   // Padding (although it is already set to 0)
    /// }
    /// 
    /// wnd.set_buffer_direct(buffer);
    /// wnd.draw_buffer();
    /// ```
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
