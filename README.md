# WinSafe

[![Crates.io](https://img.shields.io/crates/v/winsafe.svg)](https://crates.io/crates/winsafe)
[![Docs.rs](https://docs.rs/winsafe/badge.svg)](https://docs.rs/winsafe)
[![Lines of code](https://tokei.rs/b1/github/rodrigocfd/winsafe)](https://github.com/rodrigocfd/winsafe)
[![License](https://img.shields.io/crates/l/winsafe.svg)](https://github.com/rodrigocfd/winsafe/blob/master/LICENSE.md)

Win32 GUI and related APIs in safe, idiomatic Rust.

WinSafe has:

* high-level structs to build native Win32 GUI applications;
* low-level Win32 API constants, functions and structs related to GUI.

If you're looking for a comprehensive Win32 coverage, take a look at [winapi](https://crates.io/crates/winapi) or [windows](https://crates.io/crates/windows) crates, which are unsafe, but have everything.

Documentation for the WinSafe `master` branch: [rodrigocfd.github.io/winsafe/winsafe](https://rodrigocfd.github.io/winsafe/winsafe/)

## Current status

- [x] User windows (main, modal and control)
- [ ] Window messages
- [ ] Native controls
- [ ] APIs

Since this crate is *too* incomplete, it's not published to [crates.io](https://crates.io/crates/winsafe) yet. However, you can use this crate simply by cloning this repo.

## Example

WinSafe works with both ordinary windows (created programatically) and resource dialogs (from `.rc` files). Be sure to check the [examples folder](examples/), which is being constantly updated.

Here is an [example](examples/01_button_click/) of an ordinary window with a button. Note how the click event is handled with a closure:

![Example 01](examples/01_button_click/screen.gif)

```rust
#![windows_subsystem = "windows"]

use winsafe::{gui, POINT, SIZE, WinResult};

fn main() {
    let my = MyWindow::new();  // instantiate our main window
    if let Err(e) = my.wnd.run_main(None) { // ... and run it
        eprintln!("{}", e);
    }
}


#[derive(Clone)]
pub struct MyWindow {
    wnd:       gui::WindowMain, // responsible for managing the window
    btn_hello: gui::Button,     // a button
}

impl MyWindow {
    pub fn new() -> MyWindow {
        let wnd = gui::WindowMain::new( // instantiate the window manager
            gui::WindowMainOpts {
                title: "My window title".to_owned(),
                size: SIZE::new(300, 150),
                ..Default::default() // leave all other options as default
            },
        );

        let btn_hello = gui::Button::new(
            &wnd, // the window manager is the parent of our button
            gui::ButtonOpts {
                text: "&Click me".to_owned(),
                position: POINT::new(20, 20),
                ..Default::default()
            },
        );

        let new_self = Self { wnd, btn_hello };
        new_self.events(); // attach our events
        new_self
    }

    fn events(&self) {
        let wnd = self.wnd.clone(); // clone so it can be passed into the closure

        self.btn_hello.on().bn_clicked(move || {
            wnd.hwnd().SetWindowText("Hello, world!").unwrap();
        });
    }
}
```

## License

Licensed under [MIT license](https://opensource.org/licenses/MIT), see [LICENSE.txt](LICENSE.txt) for details.
