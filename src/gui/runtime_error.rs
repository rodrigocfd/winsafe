use std::error::Error;
use std::fmt;

use crate::msg::WndMsg;

/// A specialized
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) for
/// runtime, which returns an [`RuntimeError`](crate::gui::RuntimeError) on
/// failure.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub type RunResult<T> = Result<T, RuntimeError>;

/// An user error that occurred within a closure of an event handling.
///
/// This error type wraps the actual user error, providing more information.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
#[derive(Debug)]
pub struct RuntimeError {
	src_msg: WndMsg,
	source: Box<dyn Error + Send + Sync>,
}

impl fmt::Display for RuntimeError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} - {}",
			self.src_msg.msg_id, self.source.to_string())
	}
}

impl Error for RuntimeError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(self.source.as_ref())
	}
}

impl RuntimeError {
	/// Constructs a new `RuntimeError` by wrapping the given error.
	pub fn new(
		src_msg: WndMsg, source: Box<dyn Error + Send + Sync>) -> RuntimeError
	{
		Self { src_msg, source }
	}

	/// The source message information where the error originated from.
	pub const fn src_msg(&self) -> WndMsg {
		self.src_msg
	}
}
