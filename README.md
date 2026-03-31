mwin
====

This crate is a minimal window manager (`mwin`) which focuses on ease of use.
It is specifically developed for prototyping Rust applications or for simplying
playing around with windows in Rust through a simple and convienent interface.

If you are interested in more technical, but advanced window creation and
management libraries, I recommend checking out [winit] and similar crates.

[winit]: https://docs.rs/winit/latest/winit/

## Platform Support
Mwin currently only supports Windows; however, support for Linux is coming soon.

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
mwin = "0.1.0"
```
and then:
```rust
use mwin::WindowHandler;

let window = WindowHander::new("My Window", 0, 0, 500, 500).expect();
```

## License

Licensed under MIT license (https://opensource.org/licenses/MIT).
