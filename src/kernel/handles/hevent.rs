use crate::prelude::HandleClose;

impl_handle! { HEVENT: "kernel";
	/// Handle to an
	/// [event](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HEVENT {}
