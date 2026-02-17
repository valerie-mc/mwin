use std::sync::mpsc::{Sender, Receiver};

use crate::messaging::{events::WndEvent, requests::WndRequest};


// TODO: Documentation of what this is

// TODO: Probably want a capture mouse (and release mouse) function, that seems pretty useful

pub trait Window {
    fn new(title: String, evt_sender: Sender<WndEvent>, req_receiver: Receiver<WndRequest>) -> Self;
    /// This is an example of what a default run function would do
    /// while let Ok(req) = self.receiver.try_recv() {
    ///     self.handle_message(req);
    /// }
    fn run(&mut self);
    fn handle_request(&mut self, req: WndRequest) {
        match req {
            // Getters
            WndRequest::GetWndPos { rtrn } => { let _ = rtrn.send(self.get_wnd_pos()); }
            WndRequest::GetWndSize { rtrn } => { let _ = rtrn.send(self.get_wnd_size()); }
            WndRequest::GetWndPosAndSize { rtrn } => { let _ = rtrn.send(self.get_wnd_pos_and_size()); }

            WndRequest::GetCursorPos { rtrn } => { let _ = rtrn.send(self.get_cursor_pos()); }
            WndRequest::GetCursorClientPos { rtrn } => { let _ = rtrn.send(self.get_cursor_client_pos()); }

            WndRequest::IsVisible { rtrn } => { let _ = rtrn.send(self.is_visible()); }
            WndRequest::IsFocused { rtrn } => { let _ = rtrn.send(self.is_focused()); }
            
            // Setters
            WndRequest::SetWndPos { args, rtrn } => { let _ = rtrn.send(self.set_wnd_pos(args.0, args.1)); }
            WndRequest::SetWndSize { args, rtrn } => { let _ = rtrn.send(self.set_wnd_size(args.0, args.1)); }
            WndRequest::SetWndPosAndSize { args, rtrn } => { let _ = rtrn.send(self.set_wnd_pos_and_size(args.0, args.1, args.2, args.3)); }
            
            WndRequest::SetVisibility { args, rtrn } => { let _ = rtrn.send(self.set_visibility(args)); }
            WndRequest::Minimize { rtrn } => { let _ = rtrn.send(self.minimize()); }
            WndRequest::Maximize { rtrn } => { let _ = rtrn.send(self.maximize()); }

            WndRequest::Close { rtrn } => { let _ = rtrn.send(self.close()); }
        }
    }

    // * Getters * //
    fn get_wnd_pos(&self) -> (i32, i32);
    fn get_wnd_size(&self) -> (i32, i32);
    fn get_wnd_pos_and_size(&self) -> (i32, i32, i32, i32);
    fn get_cursor_pos(&self) -> (i32, i32);
    fn get_cursor_client_pos(&self) -> (i32, i32);

    fn is_visible(&self) -> bool;
    fn is_focused(&self) -> bool;
    
    // * Setters * //
    fn set_wnd_pos(&self, x: i32, y: i32);
    fn set_wnd_size(&self, width: i32, height: i32);
    fn set_wnd_pos_and_size(&self, x: i32, y: i32, width: i32, height: i32);

    fn set_visibility(&self, visible: bool);
    fn minimize(&self);
    fn maximize(&self);
    fn close(&mut self);

    // * Events (both window and inputs) * //
    // Figure out how informing the user of events should work
}


