#![allow(non_camel_case_types)]

const_ordinary! { MF_TOPOLOGY: u32;
	/// [`MF_TOPOLOGY_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mf_topology_type)
	/// enumeration (`32`).
	=>
	=>
	OUTPUT_NODE 0
	SOURCESTREAM_NODE 1
	TRANSFORM_NODE 2
	TEE_NODE 3
}

const_bitflag! { MFSESSION_GETFULLTOPOLOGY: u32;
	/// [`MFSESSION_GETFULLTOPOLOGY_FLAGS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfsession_getfulltopology_flags)
	/// enumeration (`u32`).
	=>
	=>
	CURRENT 0x1
}

const_bitflag! { MFSESSION_SETTOPOLOGY: u32;
	/// [`MFSESSION_SETTOPOLOGY_FLAGS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfsession_settopology_flags)
	/// enumeration (`u32`).
	=>
	=>
	IMMEDIATE 0x1
	NORESOLUTION 0x2
	CLEAR_CURRENT 0x4
}

const_ordinary! { MFSTARTUP: u32;
	/// [`MFStartup`](crate::MFStartup) `flags` (`u32`).
	=>
	=>
	NOSOCKET 0x1
	LITE Self::NOSOCKET.0
	FULL 0
}
