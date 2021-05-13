#![allow(non_snake_case)]

pub_struct_handle_gdi! {
	/// Handle to a
	/// [bitmap](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbitmap).
	HBITMAP
}

pub_struct_handle_gdi! {
	/// Handle to a
	/// [pen](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpen)
	/// GDI object.
	HPEN
}

pub_struct_handle_closeable! {
	/// Handle to an
	/// [event](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw).
	/// Originally just a `HANDLE`.
	HEVENT
}

pub_struct_handle_closeable! {
	/// Handle to a
	/// [thread](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information).
	/// Originally just a `HANDLE`.
	HTHREAD
}

pub_struct_handle! {
	/// Handle to an
	/// [tree view item](https://docs.microsoft.com/en-us/windows/win32/controls/tree-view-controls).
	HTREEITEM
}
