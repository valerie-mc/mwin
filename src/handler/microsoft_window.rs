use std::sync::mpsc::{Sender, Receiver};
use windows::Win32::{
    Foundation::*,
    Graphics::Gdi::ScreenToClient,
    System::LibraryLoader::GetModuleHandleW,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};
use windows_strings::{w, HSTRING, PCWSTR};

use crate::{
    messaging::{events::*, requests::*},
    handler::window_trait::*,
};


pub struct WindowMC {
    evt_sender: Sender<WndEvent>,
    req_receiver: Receiver<WndRequest>,
    handle: HWND,
    running: bool,
}

impl WindowMC {
    #[inline]
    fn send_event(&self, wnd_event: WndEvent) {
        self.evt_sender.send(wnd_event).unwrap();
    }

    fn handle_window_messages(&self, &msg: &MSG) {
        // Checking for messages here bc the WndProc function can't access this struct's data
        match msg.message {
            // Keyboard input
            WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP => 
                self.handle_keyboard_input(&msg),

            // Mouse input
            // TODO

            // Changes to window state
            WM_MOVE | WM_MOVING => self.send_event(WndEvent::WindowMoved),
            WM_SIZING => self.send_event(WndEvent::WindowResized),
            WM_SIZE => match msg.wParam.0 as u32 {
                SIZE_RESTORED  => self.send_event(WndEvent::WindowResized),
                SIZE_MINIMIZED => self.send_event(WndEvent::WindowMinimzed),
                SIZE_MAXIMIZED => self.send_event(WndEvent::WindowMaximized),
                _ => (),
            }
            WM_CLOSE   => self.send_event(WndEvent::WindowClosed),
            WM_DESTROY => self.send_event(WndEvent::WindowDestroyed),
            _ => (),
        }

        // TODO: Learn how to draw pixels to the window
        // case WM_PAINT: 
    }

    // TODO: Still can't detect just alt or F10
    // TODO: Also can't detect mouse button inputs
    fn handle_keyboard_input(&self, msg: &MSG) {
        // * Get event information
        // Get key code
        let vk_code = VIRTUAL_KEY((msg.wParam.0 & 0xFFFF) as u16); // The last 16 bits of the wParam are the vkCode
        let key: KeyCode = match vk_code {
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
            _ => return,
        };

        // Get state
        let state: KeyState = match msg.message {
            WM_KEYDOWN | WM_SYSKEYUP => KeyState::Pressed,
            WM_KEYUP  | WM_SYSKEYDOWN => KeyState::Released,
            _ => return, // Should never happen, but will safely return early
        };

        // Get modifiers
        let mut modifiers: Modifiers = Modifiers::empty();

        if unsafe { GetAsyncKeyState(VK_SHIFT.0 as i32) } != 0 {
            modifiers.insert(Modifiers::SHIFT);
        }
        if unsafe { GetAsyncKeyState(VK_CONTROL.0 as i32) } != 0 {
            modifiers.insert(Modifiers::CTRL);
        }

        let key_flags = ((msg.lParam.0 >> 16) & 0xFFFF) as u32; // The first 16 bits of the lParam are the key flags
        if key_flags & KF_ALTDOWN != 0 {
            modifiers.insert(Modifiers::ALT);
        }

        // * Create and send event
        self.send_event(WndEvent::KeyboardInput { 
            event: KeyEvent {
                key,
                state,
                modifiers,
            } 
        });
    }
}

impl Window for WindowMC {
    fn new(title: String, evt_sender: Sender<WndEvent>, req_receiver: Receiver<WndRequest>) -> Self {
        unsafe { 
            // Initalizes window settings
            let wnd_class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,          // Refresh window on resize
                lpfnWndProc: Some(DefWindowProcWExtern), // Function to process window messages
                hInstance: GetModuleHandleW(PCWSTR::null()).unwrap_or_default().into(), // Program instance handle
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),              // Cursor for the window
                lpszClassName: w!("WindowClass"),
                ..Default::default()
            };

            // Registers settings with window, returns zero if the function fails
            if RegisterClassW(&wnd_class) == 0 {
                // TODO: Deal with this panic, idk what to do with it rn
                panic!();
            }

            // Creates window
            let handle = CreateWindowExW(
                WINDOW_EX_STYLE(0),                // No extended window styles  
                wnd_class.lpszClassName,           // Class name
                &HSTRING::from(title),             // Window title
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,  // Default window
                CW_USEDEFAULT,                     // x (position)
                CW_USEDEFAULT,                     // y (position)
                500,                     // Width // TODO: RETURN BACK TO USEDEFAULT
                500,                     // Height // TODO: RETURN BACK TO USEDEFAULT
                None, None, Some(wnd_class.hInstance), None // Other settings
            ).unwrap_or_default();

            WindowMC {
                evt_sender,
                req_receiver,
                handle,
                running: true,
            }
        }
    }

    fn run(&mut self) {
        unsafe {
            while self.running {
                // Message from Microsoft
                let mut msg: MSG = Default::default();
                // Returns a negative number if it fails (note, most message go directly to the callback)
                while PeekMessageW(&mut msg, Some(self.handle), 0, 0, PM_REMOVE).as_bool() {
                    TranslateMessage(&mut msg).as_bool(); // Converts keyboard messages into a WM_CHAR message
                    DispatchMessageW(&mut msg); // Calls our window callback

                    self.handle_window_messages(&msg);
                }

                // Request from handler
                while let Ok(req) = self.req_receiver.try_recv() {
                    self.handle_request(req);
                }
            }
        }
    }

    // * Getters * //
    // Top left corner x, y, width, height
    fn get_wnd_pos(&self) -> (i32, i32) {
        let mut rect: RECT = Default::default();
        unsafe { GetClientRect(self.handle, &mut rect).unwrap(); }
        (rect.left, rect.top)

        // TODO: What about GetWindowRect()
    }
    fn get_wnd_size(&self) -> (i32, i32) {
        let mut rect: RECT = Default::default();
        unsafe { GetClientRect(self.handle, &mut rect).unwrap(); }
        (rect.right - rect.left, rect.bottom - rect.top)
    }
    fn get_wnd_pos_and_size(&self) -> (i32, i32, i32, i32) {
        let mut rect: RECT = Default::default();
        unsafe { GetClientRect(self.handle, &mut rect).unwrap(); }
        (rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top)
    }

    fn get_cursor_pos(&self) -> (i32, i32) {
        let mut point: POINT = Default::default();
        unsafe { GetCursorPos(&mut point).unwrap(); }
        (point.x, point.y)
    }
    fn get_cursor_client_pos(&self) -> (i32, i32) {
        let mut point: POINT = Default::default();
        unsafe { 
            GetCursorPos(&mut point).unwrap();
            let _ = ScreenToClient(self.handle, &mut point);
        }
        (point.x, point.y)
    }

    fn is_visible(&self) -> bool { 
        unsafe { IsWindowVisible(self.handle).as_bool() } 
    }
    fn is_focused(&self) -> bool { 
        unsafe { GetFocus()  == self.handle }
    }
    
    // * Setters * //
    // Position is relative to the top left corner in client coordinates, size does not change
    fn set_wnd_pos(&self, x: i32, y: i32) {
        unsafe { let _ = SetWindowPos(self.handle, None, x, y, 0, 0, SWP_NOSIZE); }
    }
    // Width and height in pixels, position does not change
    fn set_wnd_size(&self, width: i32, height: i32) {
        unsafe { let _ = SetWindowPos(self.handle, None, 0, 0, width, height, SWP_NOMOVE); }
    }
    // Position is relative to the top left corner in client coordinates, width and height in pixels
    fn set_wnd_pos_and_size(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { let _ = SetWindowPos(self.handle, None, x, y, width, height, SET_WINDOW_POS_FLAGS(0)); }
    }

    fn set_visibility(&self, visible: bool) {
        if visible {
            unsafe { let _ = ShowWindow(self.handle, SW_SHOW); }
        } else {
            unsafe { let _ = ShowWindow(self.handle, SW_HIDE); }
        }
    }
    fn minimize(&self) {
        unsafe { let _ = ShowWindow(self.handle, SW_MINIMIZE); }
    }
    fn maximize(&self) {
        unsafe { let _ = ShowWindow(self.handle, SW_MAXIMIZE); }
    }
    fn close(&mut self) {
        // This only minimizes window, it doesn't destroy it
        // unsafe { CloseWindow(self.handle).unwrap() };
        unsafe { DestroyWindow(self.handle).unwrap(); }
        // self.running = false; // TODO: Actually don't do this here, do this in the handle event function
        // TODO: Could have it set all values in events to false.
        // TODO: This would prevent the user from interpreting input when the window is destroyed.
    }
}

// Closes window when dropped
impl Drop for WindowMC {
    fn drop(&mut self) {
        self.close();
    }
}


#[allow(non_snake_case)]
#[allow(unused)]
#[inline(always)]
// Only used to satisfy type requirement for `lpfnWndProc`
unsafe extern "system" fn DefWindowProcWExtern(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
