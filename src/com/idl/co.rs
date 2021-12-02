//! [IDL](https://docs.microsoft.com/en-us/windows/win32/api/_com/) constants
//! and types of constants.

const_ordinary! { PICTYPE: i16;
	/// [`PICTYPE`](https://docs.microsoft.com/en-us/windows/win32/com/pictype-constants)
	/// constants (`i16`).
	=>
	=>
	UNINITIALIZED -1
	NONE 0
	BITMAP 1
	METAFILE 2
	ICON 3
	ENHMETAFILE 4
}
