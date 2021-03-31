use crate::co;

/// Trait to the parameters of a message that can be sent. Implemented by [all
/// messages](crate::msg).
///
/// Allows the conversion to the generic [`WndMsg`](crate::msg::WndMsg)
/// parameters, and also defines the return type of the message.
///
/// Used in functions like [`SendMessage`](crate::HWND::SendMessage) and
/// [`DefWindowProc`](`crate::HWND::DefWindowProc`).
pub trait MsgSend {
	/// The specific type of the value returned by the message.
	type RetType;

	/// Converts the generic `isize` return value to the specific type returned
	/// by the message.
	fn convert_ret(&self, v: isize) -> Self::RetType;

	/// Converts the specific message parameters struct into the generic
	/// [`WndMsg`](crate::msg::WndMsg) message struct.
	fn as_generic_wm(&self) -> WndMsg;
}

/// Trait to the parameters of a message that can be sent and handled.
/// Implemented by [WndMsg](crate::msg::WndMsg) and all
/// [msg::wm](`crate::msg::wm`) messages.
///
/// Allows the conversion from and to the generic [`WndMsg`](crate::msg::WndMsg)
/// parameters, and also defines the return type of the message.
pub trait MsgSendRecv: MsgSend {
	/// Converts the generic [`WndMsg`](crate::msg::WndMsg) parameters struct
	/// into the specific message struct.
	fn from_generic_wm(parm: WndMsg) -> Self;
}

/// Generic
/// [window message](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues)
/// parameters: `WPARAM` and `LPARAM`.
///
/// All message types can be converted to `WndMsg` via the
/// [`as_generic_wm`](crate::msg::MsgSend::as_generic_wm) method.
///
/// Return type: `isize`.
#[derive(Copy, Clone)]
pub struct WndMsg {
	/// The [`co::WM`](crate::co::WM) constant that identifies the window message.
	pub msg_id: co::WM,
	/// First message parameter.
	pub wparam: usize,
	/// Second message parameter.
	pub lparam: isize,
}

impl MsgSend for WndMsg {
	type RetType = isize;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v
	}

	fn as_generic_wm(&self) -> Self {
		*self
	}
}

impl MsgSendRecv for WndMsg {
	fn from_generic_wm(p: Self) -> Self {
		p
	}
}
