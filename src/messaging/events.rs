use bitflags::bitflags;

#[derive(Debug)]
pub enum WndEvent {
    KeyboardInput { event: KeyEvent },

    MouseInput { event: MouseEvent },
    MouseScrolled { event: ScrollEvent },
    CursorMoved { event: CursorEvent },

    WindowMoved { x: i32, y: i32 },
    WindowResized { width: i32, height: i32 },
    WindowPosChanged { x: i32, y: i32, width: i32, height: i32 },
    WindowMinimized,
    WindowMaximized { width: i32, height: i32 },
    WindowFocused,
    WindowUnfocused,
    WindowClosed,
    WindowDestroyed,
}

#[derive(Debug)]
pub struct KeyEvent {
    pub key: KeyCode,
    pub state: KeyState,
    pub modifiers: Modifiers,
}

#[derive(Debug)]
pub struct MouseEvent {
    pub key: KeyCode,
    pub state: KeyState,
    pub modifiers: Modifiers,
    pub position: (i32, i32),
}

#[derive(Debug)]
pub struct ScrollEvent {
    pub modifiers: Modifiers,
    pub position: (i32, i32),
    pub direction: ScrollDirection,
}

#[derive(Debug)]
pub struct CursorEvent {
    pub modifiers: Modifiers,
    pub position: (i32, i32),
}

// * Enums and Constants * //

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyCode {
    NUM0, NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9, // Number row numbers
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, // Letters
    Tilde, Minus, Plus, LeftSquareBracket, RightSquareBracket, BackSlash, Semicolon, Apostrophe, Comma, Period, Slash, // Punctuation
    Escape, Tab, Backspace, CapsLock, Shift, Ctrl, Alt, Space, Enter, PageUp, PageDown, End, Home, Insert, Delete, // Modifiers
    Up, Down, Left, Right, // Arrow keys
    NUMPAD0, NUMPAD1, NUMPAD2, NUMPAD3, NUMPAD4, NUMPAD5, NUMPAD6, NUMPAD7, NUMPAD8, NUMPAD9, // Numpad numbers
    Add, Subtract, Multiply, Divide, Decimal, Numlock, // Numpad operators
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, // Function keys
    LeftMouse, MiddleMouse, RightMouse, XMouse1, XMouse2, // Mouse buttons (X1 and X2 are the two additional buttons on mouses)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyState {
    Pressed,
    Released
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScrollDirection {
    Up,
    Down
}

bitflags! {
    #[derive(Debug)]
    pub struct Modifiers: u8 {
        const SHIFT = 1 << 0;
        const CTRL  = 1 << 1;
        const ALT   = 1 << 2;
    }
}
