use crate::requests::WndRequest;

pub trait Window {
    fn run(&mut self);
    fn handle_request(&mut self, req: WndRequest) {
        match req {
            // Getters
            WndRequest::GetWndRect { rtrn } => { let _ = rtrn.send(self.get_wnd_rect()); }
            WndRequest::GetClientRect { rtrn } => { let _ = rtrn.send(self.get_client_rect()); }

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

            // Drawing
            WndRequest::DrawBuffer { rtrn } => { let _ = rtrn.send(self.draw_buffer()); }

            WndRequest::ResizeBuffer { args, rtrn } => { let _ = rtrn.send(self.resize_buffer(args.0, args.1)); }
            WndRequest::ClearBuffer { rtrn } => { let _ = rtrn.send(self.clear_buffer()); }

            WndRequest::SetBuffer { args, rtrn } => { let _ = rtrn.send(self.set_buffer(args)); }
            WndRequest::SetBufferDirect { args, rtrn } => { let _ = rtrn.send(self.set_buffer_direct(args)); }
        }
    }

    // * Getters * //
    fn get_wnd_rect(&self) -> (i32, i32, i32, i32);
    fn get_client_rect(&self) -> (i32, i32, i32, i32);

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
    fn close(&self);

    // * Drawing * //
    fn draw_buffer(&self);
    
    fn resize_buffer(&mut self, width: i32, height: i32);
    fn clear_buffer(&mut self);

    fn set_buffer(&mut self, buffer: Vec<u8>);
    fn set_buffer_direct(&mut self, buffer: Vec<u8>);
}

pub trait ImageBuffer {
    fn init(&mut self, width: i32, height: i32);

    fn resize_buffer(&mut self, width: i32, height: i32);
    fn clear_buffer(&mut self);

    fn set_buffer(&mut self, buffer: Vec<u8>);
    fn set_buffer_direct(&mut self, buffer: Vec<u8>);
}
