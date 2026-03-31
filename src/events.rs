/// An enum for all the possible events that can occur in a window.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WndEvent {
    // * Input * //
    /// Occurs when a user presses a button on their keyboard.
    /// 
    /// The [`KeyEvent`] struct contains information about the event.
    KeyboardInput { key_event: KeyEvent },
    /// Occurs when a user presses a button on their mouse.
    /// 
    /// The [`MouseEvent`] struct contains information about the event.
    MouseInput { mouse_event: MouseEvent },
    /// Occurs when a user scrolls their mouse wheel.
    /// 
    /// The [`ScrollEvent`] struct contains information about the event.
    MouseScrolled { scroll_event: ScrollEvent },
    /// Occurs when a user moves their mouse.
    /// 
    /// The [`CursorEvent`] struct contains information about the event.
    CursorMoved { cursor_event: CursorEvent },

    // * Window state * //
    /// Occurs when a user begins to move or resize the window.
    WindowPosChanging,
    /// Occurs after a user has moved or resized the window.
    /// 
    /// The `x`, `y`, `width`, and `height` values represent the new position
    /// and size of the window.
    WindowPosChanged { x: i32, y: i32, width: i32, height: i32 },
    /// Occurs when a user minimizes the window.
    WindowMinimized,
    /// Occurs when a user maximizes the window.
    /// 
    /// The `width`, and `height` values represent the new size of the window.
    WindowMaximized { width: i32, height: i32 },
    /// Occurs when the window is focused by a user.
    WindowFocused,
    /// Occurs when the window is unfocused by a user.
    WindowUnfocused,
    /// Occurs when the window is closed by a user.
    WindowClosed,
    /// Occurs when the window is destroyed.
    WindowDestroyed,
}

/// Contains information about a [`WndEvent::KeyboardInput`] event.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct KeyEvent {
    pub key: KeyCode,
    pub state: KeyState,
    pub modifiers: Modifiers,
}

/// Contains information about a [`WndEvent::MouseInput`] event.
/// 
/// The position represents the `(x, y)` coordinates of the cursor at the time
/// of this event.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MouseEvent {
    pub key: KeyCode,
    pub state: KeyState,
    pub modifiers: Modifiers,
    pub position: (i32, i32),
}

/// Contains information about a [`WndEvent::MouseScrolled`] event.
/// 
/// The position represents the `(x, y)` coordinates of the cursor at the time
/// of this event.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ScrollEvent {
    pub modifiers: Modifiers,
    pub position: (i32, i32),
    pub direction: ScrollDirection,
}

/// Contains information about a [`WndEvent::CursorMoved`] event.
/// 
/// The position represents the `(x, y)` coordinates of the cursor at the time
/// of this event.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CursorEvent {
    pub modifiers: Modifiers,
    pub position: (i32, i32),
}

// * Enums and Constants * //

/// An enum of all buttons (both keyboard and mouse).
/// 
/// [`KeyCode::XMouse1`] and [`KeyCode::XMouse2`] correspond to the two addtional
/// buttons that some mice may have.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum KeyCode {
    NUM0, NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9, // Number row numbers
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, // Letters
    Tilde, Minus, Plus, LeftSquareBracket, RightSquareBracket, BackSlash, Semicolon, Apostrophe, Comma, Period, Slash, // Punctuation
    Escape, Tab, Backspace, CapsLock, Shift, Ctrl, Alt, Space, Enter, PageUp, PageDown, End, Home, Insert, Delete, // Modifiers
    Up, Down, Left, Right, // Arrow keys
    NUMPAD0, NUMPAD1, NUMPAD2, NUMPAD3, NUMPAD4, NUMPAD5, NUMPAD6, NUMPAD7, NUMPAD8, NUMPAD9, // Numpad numbers
    Add, Subtract, Multiply, Divide, Decimal, Numlock, // Numpad operators
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, // Function keys
    LeftMouse, MiddleMouse, RightMouse, XMouse1, XMouse2 // Mouse buttons (X1 and X2 are the two additional buttons on mouses)
}

/// An enum representing the state of a key (either Pressed or Released).
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum KeyState {
    Pressed,
    Released
}

/// An enum representing the direction the mouse wheel was scrolled.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ScrollDirection {
    Up,
    Down
}

bitflags::bitflags! {
    /// Lists which modifiers were pressed during the given event.
    /// 
    /// Use `modifiers.contains(events::Modifiers::...)` to determine if a
    /// specific modifies was pressed during the event.
    /// 
    /// Eg. `if key_event.modifiers.contains(events::Modifiers::SHIFT) { ... }`
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub struct Modifiers: u8 {
        const SHIFT = 1 << 0;
        const CTRL  = 1 << 1;
        const ALT   = 1 << 2;
    }
}

