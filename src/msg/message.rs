use crate::msg::Wm;

/// Trait to the parameters of a message that can be sent.
///
/// Allows the conversion to the generic [`Wm`](crate::msg::Wm) parameters, and
/// also defines the return type of the message.
///
/// Used in functions like [`SendMessage`](crate::HWND::SendMessage) and
/// [`DefWindowProc`](`crate::HWND::DefWindowProc`).
pub trait Message {
	/// The specific type of the value returned by the message.
	type RetType;

	/// Converts the generic `isize` return value to the specific type returned
	/// by the message.
	fn convert_ret(&self, v: isize) -> Self::RetType;

	/// Converts the message parameters to the generic [`Wm`](crate::msg::Wm)
	/// parameters.
	fn as_generic_wm(&self) -> Wm;
}

/// Trait to the parameters of a message that can be sent and handled.
///
/// Allows the conversion from and to the generic [`Wm`](crate::msg::Wm)
/// parameters, and also defines the return type of the message.
///
/// Used in functions like [`SendMessage`](crate::HWND::SendMessage) and
/// [`DefWindowProc`](`crate::HWND::DefWindowProc`).
pub trait MessageHandleable: Message {
	/// Converts the generic [`Wm`](crate::msg::Wm) parameters into the message
	/// parameters.
	fn from_generic_wm(parm: Wm) -> Self;
}
