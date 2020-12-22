/// A logic error that occurred in a high-level GUI object.
///
/// This error is always caused by the user – internal Win32 errors are
/// represented by the [`ERROR`](crate::co::ERROR) type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuiError {
	desc: String,
}

impl std::fmt::Display for GuiError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Display::fmt(&self.desc, f) // delegate
	}
}

impl std::error::Error for GuiError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}
}

impl GuiError {
	pub(super) fn new(desc: &str) -> GuiError {
		GuiError {
			desc: String::from(desc),
		}
	}
}