// use std::sync::mpsc;

// use crate::window_handler::{Window, Message};

// pub struct WindowLinux {
//     receiver: mpsc::Receiver<Message>,
// }

// impl WindowLinux {
//     pub fn new(title: String, receiver: mpsc::Receiver<Message>) -> Self {
//         WindowLinux {
//             receiver,
//         }
//     }

//     pub fn run(&mut self) {
//         loop {
//             // // Message from handler
//             // while let Ok(msg) = self.receiver.try_recv() {
//             //     self.handle_message(msg);
//             // }
//         }
//     }
// }

// impl Window for WindowLinux {
//     fn get_wnd_pos(&self) -> (i32, i32) {
//         todo!()
//     }

//     fn get_wnd_size(&self) -> (i32, i32) {
//         todo!()
//     }

//     fn get_wnd_pos_and_size(&self) -> (i32, i32, i32, i32) {
//         todo!()
//     }

//     fn get_cursor_pos(&self) -> (i32, i32) {
//         todo!()
//     }

//     fn get_cursor_client_pos(&self) -> (i32, i32) {
//         todo!()
//     }

//     fn is_visible(&self) -> bool {
//         todo!()
//     }

//     fn is_focused(&self) -> bool {
//         todo!()
//     }

//     fn set_wnd_pos(&self, x: i32, y: i32) {
//         todo!()
//     }

//     fn set_wnd_size(&self, width: i32, height: i32) {
//         todo!()
//     }

//     fn set_wnd_pos_and_size(&self, x: i32, y: i32, width: i32, height: i32) {
//         todo!()
//     }

//     fn set_visibility(&self, visible: bool) {
//         todo!()
//     }

//     fn minimize(&self) {
//         todo!()
//     }

//     fn maximize(&self) {
//         todo!()
//     }

//     fn close(&mut self) {
//         todo!()
//     }
// }

// impl Drop for WindowLinux {
//     fn drop(&mut self) {
//         self.close();
//     }
// }
