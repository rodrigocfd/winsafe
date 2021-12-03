//! [IDL](https://docs.microsoft.com/en-us/windows/win32/api/_com/) constants
//! and types of constants.

#![allow(non_camel_case_types)]

const_bitflag! { LOCKTYPE: u32;
	/// [`LOCKTYPE`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/ne-objidl-locktype)
	/// enumeration (`u32`).
	=>
	=>
	WRITE 1
	EXCLUSIVE 2
	ONLYONCE 4
}

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

const_bitflag! { STGC: u32;
	/// [`STGC`](https://docs.microsoft.com/en-us/windows/win32/api/wtypes/ne-wtypes-stgc)
	/// enumeration (`u32`).
	=>
	=>
	DEFAULT 0
	OVERWRITE 1
	ONLYIFCURRENT 2
	DANGEROUSLYCOMMITMERELYTODISKCACHE 4
	CONSOLIDATE 8
}

const_ordinary! { STREAM_SEEK: u32;
	/// [`STREAM_SEEK`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/ne-objidl-stream_seek)
	/// enumeration (`u32`).
	=>
	=>
	SET 0
	CUR 1
	END 2
}
