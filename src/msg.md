Parameters of [window messages](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).

[`Wm`] is the basic message, with `WPARAM` and `LPARAM` fields. All messages can be sent, so they all implement the [`MsgSend`](crate::prelude::MsgSend) trait. Some messages can also be received – that means you can handle them if you're implementing a custom window from scratch –, and these implement the [`MsgSendRecv`](crate::prelude::MsgSendRecv) trait.

# Sending messages

We want to delete the 3rd element of a [`ListView`](crate::gui::ListView) control. This can be done by sending it an [`LVM_DELETEITEM`](crate::msg::LvmDeleteItem) message via [`HWND::SendMessage`](crate::HWND::SendMessage). The message itself is a struct, which is initialized with the specific message parameters.

The message struct also defines the data type returned by `SendMessage`. In the example below, `LVM_DELETEITEM` returns `SysResult<()>`.

```rust,ignore
use winsafe::{self as w, prelude::*, msg};

let hlistview: w::HWND; // initialized somewhere
# let hlistview = w::HWND::NULL;

hlistview.SendMessage(
    msg::LvmDeleteItem {
        index: 2,
    },
).expect("Failed to delete item 2.");
```

Messages can be found in the [`msg`](crate::msg) module.

# Custom messages

In order to create a custom message, you must create a struct with the data it contains (if any) and implement the [`MsgSend`](crate::prelude::MsgSend) and [`MsgSendRecv`](crate::prelude::MsgSendRecv) traits:

```rust,ignore
use winsafe::{self as w, prelude::*, co, msg};

/// The integer value of our message ID.
pub const MAKE_TOAST: co::WM = unsafe { co::WM::from_raw(co::WM::USER.raw() + 20) };

/// Our message with its parameter.
struct MakeToast {
    how_many: u32,
}

impl MsgSend for MakeToast {
    type RetType = ();

    fn convert_ret(&self, _: isize) -> Self::RetType {
        ()
    }

    fn as_generic_wm(&mut self) -> msg::Wm {
        msg::Wm {
            msg_id: MAKE_TOAST,
            wparam: self.how_many as _,
            lparam: 0,
        }
    }
}

impl MsgSendRecv for MakeToast {
    fn from_generic_wm(p: msg::Wm) -> Self {
        Self {
            how_many: p.wparam as _,
        }
    }
}
```
