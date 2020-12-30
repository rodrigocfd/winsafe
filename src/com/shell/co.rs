//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM interfaces constants.

const_type! { TBPF, u32,
	/// [`ITaskbarList3::SetProgressState`](crate::shell::ITaskbarList3::SetProgressState)
	/// `tbpFlags`.

	NOPROGRESS, 0
	INDETERMINATE, 0x1
	NORMAL, 0x2
	ERROR, 0x4
	PAUSED, 0x8
}
