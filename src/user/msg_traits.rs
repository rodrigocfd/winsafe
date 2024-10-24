use crate::msg::*;

/// Parameters of a message which can be sent. Implemented by
/// [all defined messages](crate::msg).
///
/// Allows the conversion to the generic [`WndMsg`](crate::msg::WndMsg)
/// parameters, and also defines the return type of the message.
///
/// Used in functions like
/// [`SendMessage`](crate::prelude::user_Hwnd::SendMessage) and
/// [`DefWindowProc`](crate::prelude::user_Hwnd::DefWindowProc).
pub trait MsgSend {
	/// The specific type of the value returned by the message.
	type RetType;

	/// Unmarshaling method which converts the generic `isize` return value to
	/// the specific type returned by the message.
	///
	/// # Safety
	///
	/// Return values often involve pointers and require casts, make sure the
	/// conversions are correct.
	#[must_use]
	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType;

	/// Marshaling method which converts the specific message parameters struct
	/// into the generic [`WndMsg`](crate::msg::WndMsg) message struct.
	#[must_use]
	fn as_generic_wm(&mut self) -> WndMsg;
}

/// Parameters of a message which can be sent and handled (received).
/// Implemented by [`WndMsg`](crate::msg::WndMsg) and all
/// [msg::wm](`crate::msg::wm`) messages.
///
/// Allows the conversion from and to the generic [`WndMsg`](crate::msg::WndMsg)
/// parameters, and also defines the return type of the message.
pub trait MsgSendRecv: MsgSend {
	/// Unmarshaling method which converts the generic
	/// [`WndMsg`](crate::msg::WndMsg) parameters struct into the specific
	/// message struct.
	///
	/// # Safety
	///
	/// Message parameters often involve pointers and require casts, make sure
	/// the conversions are correct.
	#[must_use]
	unsafe fn from_generic_wm(parm: WndMsg) -> Self;
}
