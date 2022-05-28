use crate::msg::WndMsg;

/// A specialized
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) for
/// runtime, which returns an [`RuntimeError`](crate::gui::RuntimeError) on
/// failure.
///
/// # Examples
///
/// Converting into the generic [`ErrResult`](crate::ErrResult):
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, ErrResult, gui::RunResult};
///
/// let run_result: RunResult<()> = Ok(());
///
/// let err_result: ErrResult<()> = run_result.map_err(|err| err.into());
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub type RunResult<T> = Result<T, RuntimeError>;

/// An user error that occurred within a closure of a window message handling.
///
/// This error types wraps the actual user error along with the parameters of
/// the message where the error happened.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct RuntimeError {
	src_msg: WndMsg,
	source: Box<dyn std::error::Error + Send + Sync>,
}

impl std::error::Error for RuntimeError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		Some(self.source.as_ref())
	}
}

impl std::fmt::Debug for RuntimeError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "WM {} - {}",
			self.src_msg.msg_id, self.source.to_string())
	}
}

impl std::fmt::Display for RuntimeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		<Self as std::fmt::Debug>::fmt(self, f) // delegate to Debug trait
	}
}

impl RuntimeError {
	/// Constructs a new `RuntimeError` by wrapping the given error.
	#[must_use]
	pub const fn new(
		src_msg: WndMsg,
		source: Box<dyn std::error::Error + Send + Sync>) -> RuntimeError
	{
		Self { src_msg, source }
	}

	/// The source message information where the error originated from.
	#[must_use]
	pub const fn src_msg(&self) -> WndMsg {
		self.src_msg
	}
}
