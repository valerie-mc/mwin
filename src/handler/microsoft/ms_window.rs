use std::{
    sync::mpsc,
    os::windows::ffi::OsStrExt,
    ffi::OsStr,
};

use windows::Win32::{
    Foundation::*,
    Graphics::Gdi::*,
    System::{
        LibraryLoader::GetModuleHandleW,
        SystemServices,
    },
    UI::{
        Input::KeyboardAndMouse::*, 
        Shell,
        WindowsAndMessaging::*,
    },
};
use windows_strings::{HSTRING, PCWSTR};

use crate::{
    events::*,
    handler::microsoft::ms_image_buffer::MSImageBuffer,
    requests::WndRequest,
    traits::{ImageBuffer, Window},
};

// As a note, the situtations in which Window API functions return an error was determined using
// AI (specifically Claude), as such, I don't know if the lists are exhaustive of all error cases.

// Custom window message for when there is a window request
const WM_HANDLE_REQUEST: u32 = WM_USER + 1;

pub struct MSWindowContainer {
    subclass: Box<MSWindow>,
    _class_name_box: Box<[u16]>, // This is just so that class_name has the same lifetime as MSWindowContainer
}

impl MSWindowContainer {
    pub fn new(
        title: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        id: usize,
        req_receiver: mpsc::Receiver<WndRequest>,
        evt_sender: mpsc::Sender<WndEvent>,
    ) -> Result<Self, WindowError> {
        // Converts class name in PCWSTR (because for some reason lpszClassName doesn't accept an HSTRING)
        // I used ChatGPT for this (also, side note, the std::iter::once(0) is so that the string is null terminated)
        let class_name = format!("mwin{}", id);
        let wide: Vec<u16> = OsStr::new(&class_name).encode_wide().chain(std::iter::once(0)).collect();
        let class_name_box: Box<[u16]> = wide.into_boxed_slice();

        // Initalizes window settings
        let wnd_class = unsafe { WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW, // Refresh window on resize
            lpfnWndProc: Some(wnd_proc),    // Function to process window messages
            hInstance: GetModuleHandleW(PCWSTR::null()).unwrap_or_default().into(), // Program instance handle
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),              // Cursor for the window
            lpszClassName: PCWSTR(class_name_box.as_ptr() as *const u16),
            ..Default::default()
        } };


        // Registers settings with window (should only fail if out of memory)
        if unsafe { RegisterClassW(&wnd_class) == 0 } {
            let last_error = unsafe { GetLastError() };

            if last_error == ERROR_NOT_ENOUGH_MEMORY {
                return Err(WindowError::OutOfMemory);
            } else {
                panic!("Failed to register window class. Window error code '{:?}'", last_error);
            }
        }

        // Create window + init image_buffer (should only fail if out of memory)
        let hwnd = unsafe { CreateWindowExW(
            WINDOW_EX_STYLE(0),                // No extended window styles  
            wnd_class.lpszClassName,           // Class name
            &HSTRING::from(title),             // Window title
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,  // Default window
            x, y, width, height,
            None, None, Some(wnd_class.hInstance), None // Other settings
        ).unwrap_or_else(|_s| {
            let last_error = unsafe { GetLastError() };

            if last_error == ERROR_NOT_ENOUGH_MEMORY {
                return Err(WindowError::OutOfMemory);
            } else {
                panic!("Failed to create window. Window error code '{:?}'", last_error);
            }
        }) };

        let mut image_buffer = MSImageBuffer::default();
        image_buffer.init(width, height);

        // Create subclass (dropped when MSWindowContainer is dropped)
        let subclass = Box::new(MSWindow {
            req_receiver,
            evt_sender,
            image_buffer,
            hwnd,
            running: true,
        });

        let subclass_ptr = (&*subclass) as *const MSWindow as usize;

        // Register subclass with id (should only fail if out of memory)
        if unsafe { !Shell::SetWindowSubclass(hwnd, Some(wnd_subclass_proc), id, subclass_ptr).as_bool() } {
            let last_error = unsafe { GetLastError() };

            if last_error == ERROR_NOT_ENOUGH_MEMORY {
                return Err(WindowError::OutOfMemory);
            } else {
                panic!("Failed to set subclass. Window error code '{:?}'", last_error);
            }
        }


        Ok(MSWindowContainer {
            subclass,
            _class_name_box: class_name_box,
        })
    }

    pub fn start(&mut self) {
        self.subclass.run();
    }
}

#[inline]
unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}


// Handles the subclass proc function which sends events to the main thread
struct MSWindow {
    req_receiver: mpsc::Receiver<WndRequest>,
    evt_sender: mpsc::Sender<WndEvent>,
    image_buffer: MSImageBuffer,
    hwnd: HWND,
    running: bool, // Used to keep track of if the window is still active
}

impl MSWindow {
    // * Helper Functions * //

    // Returns the low order word of the lparam
    #[inline]
    fn lp_lo_word(lparam: LPARAM) -> i16 {
        (lparam.0 & 0xFFFF) as i16 
    }

    // Returns the high order word of the lparam
    #[inline]
    fn lp_hi_word(lparam: LPARAM) -> i16 {
        // The lparam is considered 32 bits on Win32 so we don't need to worry if the isize is 64 bits
        ((lparam.0 >> 16) & 0xFFFF) as i16
    }

    // Returns the low order word of the wparam
    #[inline]
    fn wp_lo_word(wparam: WPARAM) -> u16 {
        (wparam.0 & 0xFFFF) as u16
    }

    // Returns the high order word of the wparam
    #[inline]
    fn wp_hi_word(wparam: WPARAM) -> u16 {
        // The wparam is considered 32 bits on Win32 so we don't need to worry if the usize is 64 bits
        ((wparam.0 >> 16) & 0xFFFF) as u16
    }

    #[inline]
    fn get_key_code(wparam: WPARAM) -> Option<KeyCode> {
        let vk_code = VIRTUAL_KEY(MSWindow::wp_lo_word(wparam));

        let key = match vk_code {
            // See `https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes`
            // for all window vk_codes.
            // For readablity, I stack match statements but try to keep the line length < 120.
            
            // Number row
            VK_0 => KeyCode::NUM0, VK_1 => KeyCode::NUM1, VK_2 => KeyCode::NUM2, VK_3 => KeyCode::NUM3,
            VK_4 => KeyCode::NUM4, VK_5 => KeyCode::NUM5, VK_6 => KeyCode::NUM6, VK_7 => KeyCode::NUM7,
            VK_8 => KeyCode::NUM8, VK_9 => KeyCode::NUM9,

            // Letters
            VK_A => KeyCode::A, VK_B => KeyCode::B, VK_C => KeyCode::C, VK_D => KeyCode::D,
            VK_E => KeyCode::E, VK_F => KeyCode::F, VK_G => KeyCode::G, VK_H => KeyCode::H,
            VK_I => KeyCode::I, VK_J => KeyCode::J, VK_K => KeyCode::K, VK_L => KeyCode::L, 
            VK_M => KeyCode::M, VK_N => KeyCode::N, VK_O => KeyCode::O, VK_P => KeyCode::P, 
            VK_Q => KeyCode::Q, VK_R => KeyCode::R, VK_S => KeyCode::S, VK_T => KeyCode::T, 
            VK_U => KeyCode::U, VK_V => KeyCode::V, VK_W => KeyCode::W, VK_X => KeyCode::X, 
            VK_Y => KeyCode::Y, VK_Z => KeyCode::Z,

            // Punctuation (it's messy, but I'm okay with it)
            VK_OEM_3 => KeyCode::Tilde, VK_OEM_MINUS => KeyCode::Minus, VK_OEM_PLUS => KeyCode::Plus,
            VK_OEM_4 => KeyCode::LeftSquareBracket, VK_OEM_6 => KeyCode::RightSquareBracket,
            VK_OEM_5 | VK_OEM_102 => KeyCode::BackSlash, VK_OEM_1 => KeyCode::Semicolon, VK_OEM_7 => KeyCode::Apostrophe,
            VK_OEM_COMMA => KeyCode::Comma, VK_OEM_PERIOD => KeyCode::Period, VK_OEM_2 => KeyCode::Slash,

            // Modifiers (it's messy, but I'm okay with it)
            VK_ESCAPE => KeyCode::Escape, VK_TAB => KeyCode::Tab, VK_BACK => KeyCode::Backspace,
            VK_CAPITAL => KeyCode::CapsLock, VK_SHIFT => KeyCode::Shift, VK_CONTROL => KeyCode::Ctrl,
            VK_MENU | VK_LMENU => KeyCode::Alt, VK_SPACE => KeyCode::Space, VK_RETURN => KeyCode::Enter,
            VK_PRIOR => KeyCode::PageUp, VK_NEXT => KeyCode::PageDown, VK_END => KeyCode::End,
            VK_HOME => KeyCode::Home, VK_INSERT => KeyCode::Insert, VK_DELETE => KeyCode::Delete,

            // Arrows
            VK_UP => KeyCode::Up, VK_DOWN => KeyCode::Down, VK_LEFT => KeyCode::Left, VK_RIGHT => KeyCode::Right,

            // Numpad
            VK_NUMPAD0 => KeyCode::NUMPAD0, VK_NUMPAD1 => KeyCode::NUMPAD1, VK_NUMPAD2 => KeyCode::NUMPAD2,
            VK_NUMPAD3 => KeyCode::NUMPAD3, VK_NUMPAD4 => KeyCode::NUMPAD4, VK_NUMPAD5 => KeyCode::NUMPAD5,
            VK_NUMPAD6 => KeyCode::NUMPAD6, VK_NUMPAD7 => KeyCode::NUMPAD7, VK_NUMPAD8 => KeyCode::NUMPAD8,
            VK_NUMPAD9 => KeyCode::NUMPAD9,

            // Numpad operators
            VK_ADD => KeyCode::Add, VK_SUBTRACT => KeyCode::Subtract, VK_MULTIPLY => KeyCode::Multiply,
            VK_DIVIDE => KeyCode::Divide, VK_DECIMAL => KeyCode::Decimal, VK_NUMLOCK => KeyCode::Numlock,

            // Function keys
            VK_F1 => KeyCode::F1,   VK_F2 => KeyCode::F2,   VK_F3 => KeyCode::F3,   VK_F4 => KeyCode::F4,
            VK_F5 => KeyCode::F5,   VK_F6 => KeyCode::F6,   VK_F7 => KeyCode::F7,   VK_F8 => KeyCode::F8,
            VK_F9 => KeyCode::F9,   VK_F10 => KeyCode::F10, VK_F11 => KeyCode::F11, VK_F12 => KeyCode::F12,
            VK_F13 => KeyCode::F13, VK_F14 => KeyCode::F14, VK_F15 => KeyCode::F15, VK_F16 => KeyCode::F16,
            VK_F17 => KeyCode::F17, VK_F18 => KeyCode::F18, VK_F19 => KeyCode::F19, VK_F20 => KeyCode::F20,
            VK_F21 => KeyCode::F21, VK_F22 => KeyCode::F22, VK_F23 => KeyCode::F23, VK_F24 => KeyCode::F24,

            // Mouse buttons
            VK_LBUTTON => KeyCode::LeftMouse, VK_MBUTTON => KeyCode::MiddleMouse, VK_RBUTTON => KeyCode::RightMouse,
            VK_XBUTTON1 => KeyCode::XMouse1, VK_XBUTTON2 => KeyCode::XMouse2,

            // Unmapped keys (returns early, aka. does nothing)
            _ => return None,
        };

        Some(key)
    }

    #[inline]
    fn get_mouse_modifiers(wparam: WPARAM) -> Modifiers {
        let mut modifiers: Modifiers = Modifiers::empty();

        if wparam.0 as u32 & SystemServices::MK_SHIFT.0 != 0 {
            modifiers.insert(Modifiers::SHIFT);
        }
        if wparam.0 as u32 & SystemServices::MK_CONTROL.0 != 0 {
            modifiers.insert(Modifiers::CTRL);
        }
        if unsafe { GetAsyncKeyState(VK_MENU.0 as i32) } != 0 {
            modifiers.insert(Modifiers::ALT);
        }

        modifiers
    }

    #[inline]
    fn get_mouse_position(lparam: LPARAM) -> (i32, i32) {
        let x = MSWindow::lp_lo_word(lparam) as i32;
        let y = MSWindow::lp_hi_word(lparam) as i32;
        (x, y)
    }


    // * Handling Events * //

    #[inline]
    fn send_event(&self, wnd_event: WndEvent) {
        let _ = self.evt_sender.send(wnd_event);
    }

    fn handle_keyboard_input(&self, state: KeyState, wparam: WPARAM, lparam: LPARAM) {
        // * Get event information
        let Some(key) = MSWindow::get_key_code(wparam) else {
            return
        };
        
        // Get modifiers
        let mut modifiers: Modifiers = Modifiers::empty();
        
        if unsafe { GetAsyncKeyState(VK_SHIFT.0 as i32) } != 0 {
            modifiers.insert(Modifiers::SHIFT);
        }
        if unsafe { GetAsyncKeyState(VK_CONTROL.0 as i32) } != 0 {
            modifiers.insert(Modifiers::CTRL);
        }

        let key_flags = MSWindow::lp_hi_word(lparam) as u32;
        if key_flags & KF_ALTDOWN != 0 {
            modifiers.insert(Modifiers::ALT);
        }

        // * Create and send event
        self.send_event(WndEvent::KeyboardInput { 
            key_event: KeyEvent {
                key,
                state,
                modifiers,
            } 
        });
    }
    
    fn handle_mouse_input(&self, key: KeyCode, state: KeyState, wparam: WPARAM, lparam: LPARAM) {
        self.send_event(WndEvent::MouseInput { 
            mouse_event: MouseEvent {
                key,
                state,
                modifiers: MSWindow::get_mouse_modifiers(wparam),
                position: MSWindow::get_mouse_position(lparam),
            } 
        });
    }

    fn handle_scroll(&self, wparam: WPARAM, lparam: LPARAM) {
        let direction = if MSWindow::wp_hi_word(wparam) as i16 > 0 {
            ScrollDirection::Up
        } else {
            ScrollDirection::Down
        };

        self.send_event(WndEvent::MouseScrolled { 
            scroll_event: ScrollEvent {
                modifiers: MSWindow::get_mouse_modifiers(wparam),
                position: MSWindow::get_mouse_position(lparam),
                direction,
            } 
        });
    }

    fn handle_cursor_move(&self, wparam: WPARAM, lparam: LPARAM) {
        self.send_event(WndEvent::CursorMoved { 
            cursor_event: CursorEvent {
                modifiers: MSWindow::get_mouse_modifiers(wparam),
                position: MSWindow::get_mouse_position(lparam),
            } 
        });
    }
}

impl Window for MSWindow {
    fn run(&mut self) {
        while self.running {
            let mut msg: MSG = Default::default();
            unsafe {
                while PeekMessageW(&mut msg, Some(self.hwnd), 0, 0, PM_REMOVE).as_bool() {
                    let _ = TranslateMessage(&mut msg); // Converts keyboard messages into a WM_CHAR message
                    DispatchMessageW(&mut msg);
                }
            }

            // This is for when there are no window messages
            // Otherwise requests are handled by the subclass proc fn
            if let Ok(req) = self.req_receiver.try_recv() {
                self.handle_request(req);
            }
        }
    }

    // * Getters * //
    fn get_wnd_rect(&self) -> (i32, i32, i32, i32) {
        let mut rect: RECT = Default::default();
        unsafe { GetWindowRect(self.hwnd, &mut rect).expect("Handle should be valid."); }
        (rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top)
    }
    fn get_wnd_size(&self) -> (i32, i32) {
        let mut rect: RECT = Default::default();
        unsafe { GetClientRect(self.hwnd, &mut rect).expect("Handle should be valid."); }
        (rect.right, rect.bottom)
    }

    fn get_cursor_pos(&self) -> (i32, i32) {
        let mut point: POINT = Default::default();
        unsafe { GetCursorPos(&mut point).expect("Point should not be null."); }
        (point.x, point.y)
    }
    fn get_cursor_client_pos(&self) -> (i32, i32) {
        let mut point: POINT = Default::default();
        unsafe { 
            GetCursorPos(&mut point).expect("Point should not be null.");
            let _ = ScreenToClient(self.hwnd, &mut point); // Fails if hwnd is invalid (which shouldn't happen)
        }
        (point.x, point.y)
    }

    fn is_visible(&self) -> bool { 
        unsafe { IsWindowVisible(self.hwnd).as_bool() } 
    }
    fn is_focused(&self) -> bool { 
        unsafe { GetFocus() == self.hwnd }
    }
    
    // * Setters * //

    // These all fail if hwnd is invalid (which shouldn't happen)
    fn set_wnd_pos(&self, x: i32, y: i32) {
        unsafe { let _ = SetWindowPos(self.hwnd, None, x, y, 0, 0, SWP_NOSIZE); }
    }
    fn set_wnd_size(&self, width: i32, height: i32) {
        unsafe { let _ = SetWindowPos(self.hwnd, None, 0, 0, width, height, SWP_NOMOVE); }
    }
    fn set_wnd_pos_and_size(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { let _ = SetWindowPos(self.hwnd, None, x, y, width, height, SET_WINDOW_POS_FLAGS(0)); }
    }

    fn set_visibility(&self, visible: bool) {
        if visible {
            unsafe { let _ = ShowWindow(self.hwnd, SW_SHOW); }
        } else {
            unsafe { let _ = ShowWindow(self.hwnd, SW_HIDE); }
        }
    }
    fn minimize(&self) {
        unsafe { let _ = ShowWindow(self.hwnd, SW_MINIMIZE); }
    }
    fn maximize(&self) {
        unsafe { let _ = ShowWindow(self.hwnd, SW_MAXIMIZE); }
    }
    fn close(&self) {
        unsafe { let _ = PostMessageW(Some(self.hwnd), WM_CLOSE, WPARAM(0), LPARAM(0)); }
    }

    // * Drawing * //

    fn draw_buffer(&self) {
        unsafe { let _ = PostMessageW(Some(self.hwnd), WM_PAINT, WPARAM(0), LPARAM(0)); }
    }
    
    fn resize_buffer(&mut self, width: i32, height: i32) {
        self.image_buffer.resize_buffer(width, height);
    }
    fn clear_buffer(&mut self) {
        self.image_buffer.clear_buffer();
    }
    
    fn set_buffer(&mut self, buffer: Vec<u8>) {
        self.image_buffer.set_buffer(buffer);
    }
    fn set_buffer_direct(&mut self, buffer: Vec<u8>) {
        self.image_buffer.set_buffer_direct(buffer);
    }
}

impl Drop for MSWindow {
    fn drop(&mut self) {
        self.close();
    }
}

unsafe extern "system" fn wnd_subclass_proc(
    hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM, id: usize, dw_ref_data: usize
) -> LRESULT {
    // Convert dw_ref_data to subclass pointer
    let subclass = unsafe { (dw_ref_data as *mut MSWindow).as_mut().expect("dw_ref_data is null") };

    // Handles messages (this is in the subclass proc because the window enters a modal loop when moved/resized)
    if let Ok(req) = subclass.req_receiver.try_recv() {
        subclass.handle_request(req);
        // Posting a message just ensures that the proc fn is called again (in case there is another request)
        unsafe { let _ = PostMessageW(Some(subclass.hwnd), WM_HANDLE_REQUEST, WPARAM(0), LPARAM(0)); }
    }

    match msg {
        // - Keyboard input - //
        WM_KEYDOWN | WM_SYSKEYDOWN => subclass.handle_keyboard_input(KeyState::Pressed, wparam, lparam),
        WM_KEYUP | WM_SYSKEYUP => subclass.handle_keyboard_input(KeyState::Released, wparam, lparam),

        // - Mouse input - //
        WM_LBUTTONDOWN => subclass.handle_mouse_input(KeyCode::LeftMouse, KeyState::Pressed, wparam, lparam),
        WM_LBUTTONUP => subclass.handle_mouse_input(KeyCode::LeftMouse, KeyState::Released, wparam, lparam),
        WM_MBUTTONDOWN => subclass.handle_mouse_input(KeyCode::MiddleMouse, KeyState::Pressed, wparam, lparam),
        WM_MBUTTONUP => subclass.handle_mouse_input(KeyCode::MiddleMouse, KeyState::Released, wparam, lparam),
        WM_RBUTTONDOWN => subclass.handle_mouse_input(KeyCode::RightMouse, KeyState::Pressed, wparam, lparam),
        WM_RBUTTONUP => subclass.handle_mouse_input(KeyCode::RightMouse, KeyState::Released, wparam, lparam),
        WM_XBUTTONDOWN => {
            if MSWindow::wp_hi_word(wparam) == XBUTTON1 {
                subclass.handle_mouse_input(KeyCode::XMouse1, KeyState::Pressed, wparam, lparam)
            } else {
                subclass.handle_mouse_input(KeyCode::XMouse2, KeyState::Pressed, wparam, lparam)
            }
        }
        WM_XBUTTONUP => {
            if MSWindow::wp_hi_word(wparam) == XBUTTON1 {
                subclass.handle_mouse_input(KeyCode::XMouse1, KeyState::Released, wparam, lparam)
            } else {
                subclass.handle_mouse_input(KeyCode::XMouse2, KeyState::Released, wparam, lparam)
            }
        }

        WM_MOUSEWHEEL | WM_MOUSEHWHEEL => subclass.handle_scroll(wparam, lparam),

        WM_MOUSEMOVE => subclass.handle_cursor_move( wparam, lparam),

        // - Changes to window state - //
        WM_SIZE => {
            match wparam.0 as u32 {
                SIZE_MINIMIZED => subclass.send_event(WndEvent::WindowMinimized),
                SIZE_MAXIMIZED => subclass.send_event(WndEvent::WindowMaximized { 
                    width: MSWindow::lp_lo_word(lparam) as i32,
                    height: MSWindow::lp_hi_word(lparam) as i32
                }),
                _ => (),
            }
        }

        WM_ENTERSIZEMOVE => subclass.send_event(WndEvent::WindowPosChanging),
        WM_EXITSIZEMOVE => {
            let (x, y, width, height) = subclass.get_wnd_rect();
            subclass.send_event(WndEvent::WindowPosChanged { x, y, width, height });
        }

        WM_ACTIVATE => {
            let wp_lo_word = MSWindow::wp_lo_word(wparam) as u32;
            if wp_lo_word == WA_INACTIVE {
                subclass.send_event(WndEvent::WindowUnfocused)
            } else {
                subclass.send_event(WndEvent::WindowFocused)
            }
        }
        WM_CLOSE   => subclass.send_event(WndEvent::WindowClosed),
        WM_DESTROY => subclass.send_event(WndEvent::WindowDestroyed),

        //  Drawing to window - //
        WM_PAINT => {
            let mut paint: PAINTSTRUCT = Default::default();

            let _device_context: HDC = unsafe { BeginPaint(hwnd, &mut paint) };
            let hdc: HDC = unsafe { GetDC(Some(hwnd)) };

            unsafe {
                StretchDIBits(
                    hdc,
                    0, 0, subclass.image_buffer.width, subclass.image_buffer.height,
                    0, 0, subclass.image_buffer.width, subclass.image_buffer.height,
                    Some(subclass.image_buffer.buffer.as_ptr() as *const std::ffi::c_void),
                    &subclass.image_buffer.bitmap_info,
                    DIB_RGB_COLORS,
                    SRCCOPY,
                );

                ReleaseDC(Some(hwnd), hdc);

                let _ = EndPaint(hwnd, &mut paint);
            }
        }

        // - Remove subclass on destroy - //
        WM_NCDESTROY => {
            subclass.running = false;
            unsafe { let _ = Shell::RemoveWindowSubclass(hwnd, Some(wnd_subclass_proc), id); }
        }

        _ => (),
    }

    unsafe { Shell::DefSubclassProc(hwnd, msg, wparam, lparam) }
}
