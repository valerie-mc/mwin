use std::thread;
use windows::Win32::{
    System::LibraryLoader::GetModuleHandleW,
    Foundation::*,
    UI::{Input::KeyboardAndMouse::GetFocus, WindowsAndMessaging::*},
};
use windows_strings::{w, HSTRING, PCWSTR};





// * Note: Currently, this struct assumes we are running on windows (see wnd_handle as evidence)
pub struct Window {
    title: String,
    // input: Input,
    wnd_handle: HWND,
}

impl Window {
    // * Constructor
    // Opens window by default
    pub fn new(title: &str) -> Self {
        let mut wnd = Window {
            title: title.to_string(),
            wnd_handle: Default::default(),
        };
        
        unsafe { 
            init_wnd(&mut wnd);
            run_wnd(&wnd); // TODO: THREADY
        }

        wnd
    }

    // * Getters
    // Returns the positions of the windows four edges (left, top, right, bottom)
    pub fn get_wnd_pos(&self) -> (i32, i32, i32, i32) {
        let mut rect: RECT = Default::default();
        unsafe { GetClientRect(self.wnd_handle, &mut rect).unwrap() };
        (rect.left, rect.top, rect.right, rect.bottom)
    }

    pub fn get_cursor_pos(&self) -> (i32, i32) {
        let mut point: POINT = Default::default();
        unsafe { GetCursorPos(&mut point).unwrap() };
        (point.x, point.y)
    }

    pub fn get_client_cursor_pos(&self) -> (i32, i32) {
        let mut point: POINT = Default::default();
        unsafe { 
            GetCursorPos(&mut point).unwrap();
            // ScreenToClient(&self.wnd_handle, &mut point);
        }
        (point.x, point.y)
    }

    // Getting mouse pos
    // let mut point: POINT;
    // GetCursorPos(&point);
    // ScreenToClient(wnd_handle, &point);
    // point.x and point.y

    pub fn visible(&self) -> bool { 
        unsafe { IsWindowVisible(self.wnd_handle).as_bool() } 
    }
    pub fn focused(&self) -> bool { 
        unsafe { GetFocus()  == self.wnd_handle }
    }
    
    // * Setters
    // Position is relative to the top left corner in client coordinates, size does not change
    pub fn set_pos(&self, x: i32, y: i32) {
        unsafe { let _ = SetWindowPos(self.wnd_handle, None, x, y, 0, 0, SWP_NOSIZE); }
    }
    // Width and height in pixels, position does not change
    pub fn set_size(&self, width: i32, height: i32) {
        unsafe { let _ = SetWindowPos(self.wnd_handle, None, 0, 0, width, height, SWP_NOMOVE); }
    }
    // Position is relative to the top left corner in client coordinates, width and height in pixels
    pub fn set_rect(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { let _ = SetWindowPos(self.wnd_handle, None, x, y, width, height, SET_WINDOW_POS_FLAGS(0)); }
    }

    pub fn set_visibility(&self, visible: bool) {
        if visible {
            unsafe { let _ = ShowWindow(self.wnd_handle, SW_SHOW); }
        } else {
            unsafe { let _ = ShowWindow(self.wnd_handle, SW_HIDE); }
        }
    }

    // * Window functionality
    pub fn minimize(&self) {
        unsafe { let _ = ShowWindow(self.wnd_handle, SW_MINIMIZE); }
    }
    pub fn maximize(&self) {
        unsafe { let _ = ShowWindow(self.wnd_handle, SW_MAXIMIZE); }
    }
    // Closes an open window, does nothing if the window is already closed
    pub fn close(&mut self) {
        unsafe { CloseWindow(self.wnd_handle).unwrap() };
    }

    // * Events (both window and inputs)
    // Figure out how informing the user of events should work
}

// Closes window when dropped
impl Drop for Window {
    fn drop(&mut self) {
        self.close();
    }
}


// The majority of this code is unsafe so it is just going in one unsafe block
unsafe fn init_wnd(wnd: &mut Window) {
    unsafe {
        // Initalizes window settings
        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,      // Refresh window on resize
            lpfnWndProc: Some(process_wnd_msgs), // Function to process window messages
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
        wnd.wnd_handle = CreateWindowExW(
            WINDOW_EX_STYLE(0),                // No extended window styles  
            wnd_class.lpszClassName,           // Class name
            &HSTRING::from(wnd.title.clone()), // Window title
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,  // Default window
            CW_USEDEFAULT,                     // x (position)
            CW_USEDEFAULT,                     // y (position)
            CW_USEDEFAULT,                     // Width
            CW_USEDEFAULT,                     // Height
            None, None, Some(wnd_class.hInstance), None // Other settings
        ).unwrap_or_default();
    }
}

unsafe fn run_wnd(wnd: &Window) {
    unsafe {
        loop {
            let mut msg: MSG = Default::default();
            // Returns a negative number if it fails (note, most message go directly to the callback)
            while PeekMessageW(&mut msg, Some(wnd.wnd_handle), 0, 0, PM_REMOVE).as_bool() {
                TranslateMessage(&mut msg).as_bool(); // Converts keyboard messages into a WM_CHAR message
                DispatchMessageW(&mut msg); // Calls our window callback

                // TODO: This is where I can get messages to the wnd
            }
        }
    }
}


// Processes all of the window's messages
unsafe extern "system" fn process_wnd_msgs(wnd_handle: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    // LRESULT to be returned (some messages require a return value)
    let mut result: LRESULT = Default::default();

    match msg {
        // WM_CLOSE => wnd.running = false,
        // WM_SIZE => resize
        // 

        // Uses the default process for a window message because there are too many to handle manually (8000+)
        _ => result = unsafe { DefWindowProcW(wnd_handle, msg, wp, lp) },
    }

    result
}