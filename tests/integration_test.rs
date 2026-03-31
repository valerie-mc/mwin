use std::{thread, time::Duration};
use mwin::{errors, events, WindowHandler};

// TODO: These tests are integration tests, you should add unit tests
// TODO: See: https://doc.rust-lang.org/book/ch11-03-test-organization.html

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
fn test_fn() {
    use std::{thread, time::Duration};

    // Creates a new window with the title "My Window", at (0, 0) with a size of 500 by 500.
    let window = WindowHandler::new("My Window", 0, 0, 500, 500)
        .expect("Current operating system is unsupported.");

    // Creates a buffer of 500 * 500 white pixels
    let buffer: Vec<u8> = vec![255; 3 * (500 * 500)];

    window.set_buffer(buffer);

    // This stops the window from closing when the WindowHandler is dropped.
    thread::sleep(Duration::from_secs(5));
}



#[test]
fn test_fn_2() {
    // use mwin::{events, WindowHandler};
    let wnd = WindowHandler::new("Window", 0, 250, 500, 750)
        .expect("Current operating system is unsupported.");

    sleep(3000);

    // Exact size of the boarder depends on the os, this represents Windows' border
    assert_eq!((482, 706), wnd.get_wnd_size().expect("Window shouldn't be closed."));
}



// Constructor test
#[test]
fn constructor_one_window_test() {
    let _wnd1 = create_window();
}

#[test]
fn constructor_two_windows_test() {
    let _wnd1 = create_window();
    let _wnd2 = create_window();
}

// Get Wnd Events test
#[test]
fn get_wnd_events_test() {
    // let wnd = create_window();

    // wnd.minimize();
    // wnd.maximize();
    // wnd.

    // sleep(100);
}


// Get Wnd Event test

    // Get wnd rect


// get client rect

// get curous post

// get_cursour client pos

// is visible
#[test]
fn is_visible_test() {
    let wnd = create_window();
    assert!(wnd.is_visible().unwrap());

    wnd.set_visibility(true);
    assert!(wnd.is_visible().unwrap());

    wnd.set_visibility(false);
    assert!(!wnd.is_visible().unwrap());
}

// is focused
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

// set wnd pos

// set wnd size

// TODO: Decide if you will keep set_wnd_pos_and_size

// set_visibility (tested implicitly)

// minimize

// maximize

// close
#[test]
fn close_test() {
    let wnd = create_window();
    wnd.close();
    assert_eq!(Some(errors::WindowError::WindowClosed), wnd.close());
}

// Testing drawing? (this might be better done with unit tests as
//                   I'm not sure how I would validate the results)