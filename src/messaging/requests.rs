use std::sync::mpsc::Sender;

// @args indicates the argument types (if any)
// @rtrn indicates the return types (in a Sender)
pub enum WndRequest {
    // Getters
    GetWndRect { rtrn: Sender<(i32, i32, i32, i32)> },
    GetClientRect { rtrn: Sender<(i32, i32, i32, i32)> },

    GetCursorPos { rtrn: Sender<(i32, i32)> },
    GetCursorClientPos { rtrn: Sender<(i32, i32)> },

    IsVisible { rtrn: Sender<bool> },
    IsFocused { rtrn: Sender<bool> },

    // Setters
    SetWndPos { args: (i32, i32), rtrn: Sender<()> },
    SetWndSize { args: (i32, i32), rtrn: Sender<()> },
    SetWndPosAndSize { args: (i32, i32, i32, i32), rtrn: Sender<()> },

    SetVisibility { args: bool, rtrn: Sender<()> },
    Minimize { rtrn: Sender<()> },
    Maximize { rtrn: Sender<()> },
    Close { rtrn: Sender<()> },

    // Drawing
    DrawBuffer { rtrn: Sender<()> },

    ResizeBuffer { args: (i32, i32), rtrn: Sender<()> },
    ClearBuffer { rtrn: Sender<()> },

    SetBuffer { args: Vec<u8>, rtrn: Sender<()> },
    SetPixel { args: (i32, i32, u8, u8, u8), rtrn: Sender<()> }
}
