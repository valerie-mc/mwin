## Features
 - Create windows
    `CreateWindowA()` (but also more stuff)



    `FindWindow()`, `ShowWindow()`
 - Close windows
    `CloseWindow()`
 - Resize windows
    `SetWindowPos()`
 - Move windows
    `SetWindowPos()`
    https://youtu.be/tASfx1C0hE8?list=PLKUl_fMWLdH8Kk4iFnWHhU43xXG2t6bjf&t=226

 - Get window width/height
    `GetWindowRect()`
 - Get window position and depth (x, y, z)
 - Get mouse position within window
    `POINT lastMousePos = {};`
    `GetCursorPos(&lastMousePos{});`


 - Events for when a window is...
   - resized
   - minimized
   - closed

## Extra Features
 - Change border style
   `GetWindowLongPtrA()`, `SetWindowLongPtrA()`
   https://youtu.be/tASfx1C0hE8?list=PLKUl_fMWLdH8Kk4iFnWHhU43xXG2t6bjf&t=160

 - Functionality for Linux distros (bascially same thing we already did, but for Linux)


## Clean-Up Tasks
 - Properly deal with unwraps, expects, panics, and let _ = ...
 - Properly document your code (using actually doucmentation)
    // TODO: Think about how much more efficent it would be to use https://crates.io/crates/phf for getting key code
 - Adding proper derives to structs and enums (https://stackoverflow.com/questions/58044095/what-traits-should-simple-enums-in-rust-derive)

## Credits
 - A lot of the information about how to use the WinAPI to create windows is from https://www.youtube.com/@lowlevelgamedev9330