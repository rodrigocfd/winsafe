//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM interfaces constants.

const_type! {
	/// [`ITaskbarList::SetProgressState`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
	/// `tbpFlags`.
	TBPF, u32,

	NOPROGRESS, 0
	INDETERMINATE, 0x1
	NORMAL, 0x2
	ERROR, 0x4
	PAUSED, 0x8
}