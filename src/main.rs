pub mod window;
pub mod input;

use crate::window::*;


fn main() {
    let mut wnd = Window::new("My Window");

    let mut x = 10;

    wnd.set_pos(x, 12);
    wnd.close();

    x += 1;

    println!("{x}");
}


