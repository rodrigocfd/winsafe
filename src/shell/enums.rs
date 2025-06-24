use crate::decl::*;

/// Variable parameter for:
///
/// * [`SHELLEXECUTEINFO`](crate::SHELLEXECUTEINFO)
pub enum IcoMon<'a> {
	/// No parameter (null value).
	None,
	/// The `hIcon` parameter.
	Ico(&'a HICON),
	/// The `hMonitor` parameter.
	Mon(&'a HMONITOR),
}

impl<'a> Default for IcoMon<'a> {
	fn default() -> Self {
		Self::None
	}
}
