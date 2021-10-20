# WinSafe

[![Crates.io](https://img.shields.io/crates/v/winsafe.svg)](https://crates.io/crates/winsafe)
[![Docs.rs](https://docs.rs/winsafe/badge.svg)](https://docs.rs/winsafe)
[![Lines of code](https://tokei.rs/b1/github/rodrigocfd/winsafe)](https://github.com/rodrigocfd/winsafe)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Win32 GUI and related APIs in safe, idiomatic Rust.

WinSafe has:

* high-level structs to build native Win32 GUI applications;
* low-level Win32 API constants, functions and structs related to GUI.

If you're looking for a comprehensive Win32 coverage, take a look at [winapi](https://crates.io/crates/winapi) or [windows](https://crates.io/crates/windows) crates, which are *unsafe*, but have everything.

WinSafe documentation:
* stable release: [docs.rs/winsafe](https://docs.rs/winsafe)
* master branch: [rodrigocfd.github.io/winsafe/winsafe](https://rodrigocfd.github.io/winsafe/winsafe/)

## Current status

This crate is still in alpha stage. Below is an estimated progress of feature groups:

| Feature group | Estimated progress |
| - | - |
| User windows (main, modal and control) | ![Progress](https://progress-bar.dev/100/) |
| Native controls | ![Progress](https://progress-bar.dev/75/) |
| Window messages | ![Progress](https://progress-bar.dev/55/) |
| Overall Win32 APIs | ![Progress](https://progress-bar.dev/25/) | |

## Usage

Add the dependency in your `Cargo.toml`:

```toml
[dependencies]
winsafe = "0.0.6"
```

To enable the DirectShow COM module, use:

```toml
[dependencies]
winsafe = { version = "0.0.6", features = ["dshow"] }
```

To enable the Shell COM module, use:

```toml
[dependencies]
winsafe = { version = "0.0.6", features = ["shell"] }
```

## Example

**Note:** You can find several examples in the dedicated repo: [github.com/rodrigocfd/winsafe-examples](https://github.com/rodrigocfd/winsafe-examples)

WinSafe allows you to create windows in two ways:

* programmatically defining parameters; or
* [loading dialogs](https://github.com/rodrigocfd/winsafe-examples/tree/master/03_dialog_resources) from a `.res` file created with a WYSIWYG resource editor.

The [example below](https://github.com/rodrigocfd/winsafe-examples/tree/master/01_button_click/) creates a window  with a button programmatically. Note how the click event is handled with a closure:

![Example 01](https://raw.githubusercontent.com/rodrigocfd/winsafe-examples/master/01_button_click/screen.gif)

```rust
#![windows_subsystem = "windows"]

use winsafe::prelude::*;
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
        self.btn_hello.on().bn_clicked({
            let wnd = self.wnd.clone(); // clone so it can be passed into the closure
            move || {
                wnd.hwnd().SetWindowText("Hello, world!")?;
                Ok(())
            }
        });
    }
}
```

## License

Licensed under [MIT license](https://opensource.org/licenses/MIT), see [LICENSE.md](LICENSE.md) for details.
