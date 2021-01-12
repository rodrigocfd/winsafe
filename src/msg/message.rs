use crate::msg::Wm;

/// Trait to all message parameters.
///
/// Allows the conversion from/into the generic [`Wm`](crate::msg::Wm)
/// parameters, and also defines the return type of the message.
///
/// Used in functions like [`SendMessage`](crate::HWND::SendMessage) and
/// [`DefWindowProc`](`crate::HWND::DefWindowProc`).
pub trait Message {
	/// The specific type of the value returned by the message.
	type RetType;

	/// Converts the generic `isize` return value to the specific type returned
	/// by the message.
	fn convert_ret(v: isize) -> Self::RetType;

	/// Converts the message parameters to the generic [`Wm`](crate::msg::Wm)
	/// parameters.
	fn into_generic_wm(self) -> Wm;

	/// Converts the generic [`Wm`](crate::msg::Wm) parameters into the message
	/// parameters.
	fn from_generic_wm(parm: Wm) -> Self;
}
