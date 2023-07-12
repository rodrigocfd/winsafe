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

const_ordinary! { MFSTARTUP: u32;
	/// [`MFStartup`](crate::MFStartup) `flags` (`u32`).
	=>
	=>
	NOSOCKET 0x1
	LITE Self::NOSOCKET.0
	FULL 0
}
