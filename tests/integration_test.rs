use std::{thread, time::Duration};
use mwin::{WindowError, events, WindowHandler};


// Helper functions
fn create_window() -> WindowHandler {
    let wnd = WindowHandler::new("Window", 0, 0, 500, 500)
        .expect("Tests can only be run on a supported os.");

    // Gives the os time to create the window
    thread::sleep(Duration::from_millis(100));
    wnd
}

fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}


#[test]
fn constructor_one_window_test() {
    let _wnd1 = create_window();
}

#[test]
fn constructor_two_windows_test() {
    let _wnd1 = create_window();
    let _wnd2 = create_window();
}

#[test]
fn get_wnd_rect_test() {
    let wnd = create_window();
    assert_eq!(Ok((0, 0, 500, 500)), wnd.get_wnd_rect());
}

#[test]
fn get_wnd_size_test() {
    let wnd = create_window();
    assert_eq!(Ok((482, 456)), wnd.get_wnd_size());
}

#[test]
fn is_visible_test() {
    let wnd = create_window();
    assert!(wnd.is_visible().unwrap());

    wnd.set_visibility(true);
    assert!(wnd.is_visible().unwrap());

    wnd.set_visibility(false);
    assert!(!wnd.is_visible().unwrap());
}

// * NOTE: This test cannot be run at the same time as other test, otherwise it will fail
#[test]
fn is_focused_test() {
    let wnd1 = create_window();
    assert!(wnd1.is_focused().unwrap());

    let wnd2 = create_window();
    assert!(!wnd1.is_focused().unwrap());
    assert_eq!(events::WndEvent::WindowUnfocused, wnd1.get_wnd_event().unwrap());

    wnd2.close();
    sleep(100);
    assert!(wnd1.is_focused().unwrap());
    assert_eq!(events::WndEvent::WindowFocused, wnd1.get_wnd_event().unwrap());
}

#[test]
fn set_wnd_pos_test() {
    let wnd = create_window();
    wnd.set_wnd_pos(300, 400);
    assert_eq!(Ok((300, 400, 500, 500)), wnd.get_wnd_rect());
}

#[test]
fn set_wnd_size_test() {
    let wnd = create_window();
    wnd.set_wnd_size(750, 1000);
    assert_eq!(Ok((0, 0, 750, 1000)), wnd.get_wnd_rect());
}

#[test]
fn set_wnd_pos_and_size_test() {
    let wnd = create_window();
    wnd.set_wnd_pos_and_size(300, 400, 750, 1000);
    assert_eq!(Ok((300, 400, 750, 1000)), wnd.get_wnd_rect());
}

#[test]
fn minimize_test()  {
    let wnd = create_window();
    wnd.minimize();
    assert_eq!(Some(events::WndEvent::WindowMinimized), wnd.get_wnd_event());
}
#[test]
fn maximize_test()  {
    let wnd = create_window();
    wnd.maximize();
    let (width, height) = wnd.get_wnd_size().expect("Window shouldn't be closed.");
    assert_eq!(Some(events::WndEvent::WindowMaximized { width, height }), wnd.get_wnd_event());
}

#[test]
fn close_test() {
    let wnd = create_window();
    wnd.close();
    sleep(100);
    assert_eq!(Some(events::WndEvent::WindowClosed), wnd.get_wnd_event());
    assert_eq!(Some(WindowError::WindowClosed), wnd.close());
}
