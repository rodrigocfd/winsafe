/// [`GUID`](https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GUID {
	data1: u32,
	data2: u16,
	data3: u16,
	data4: u64,
}

impl std::fmt::Display for GUID {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
			self.data1, self.data2, self.data3,
			self.data4.swap_bytes() >> 48,
			self.data4.swap_bytes() & 0x0000_ffff_ffff_ffff,
		)
	}
}

impl GUID {
	/// Creates a new `GUID` from hex numbers, which can be copied straight from
	/// standard `GUID` definitions.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::GUID;
	///
	/// let g = GUID::new(0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046);
	/// ```
	pub const fn new(p1: u32, p2: u16, p3: u16, p4: u16, p5: u64) -> GUID {
		let mut guid = GUID {
			data1: p1,
			data2: p2,
			data3: p3,
			data4: ((p4 as u64) << 48) | p5,
		};
		guid.data4 = guid.data4.swap_bytes();
		guid
	}
}

/// COM class ID. Just a safe abstraction over a [`GUID`](crate::GUID).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CLSID(GUID);

impl From<GUID> for CLSID {
	fn from(guid: GUID) -> Self {
		Self(guid)
	}
}

impl AsRef<GUID> for CLSID {
	fn as_ref(&self) -> &GUID {
		&self.0
	}
}

impl std::fmt::Display for CLSID {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl CLSID {
	/// Creates a new `CLSID` from hex numbers, which can be copied straight from
	/// standard `CLSID` definitions.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::CLSID;
	///
	/// let g = CLSID::new(0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046);
	/// ```
	pub const fn new(p1: u32, p2: u16, p3: u16, p4: u16, p5: u64) -> CLSID {
		Self(GUID::new(p1, p2, p3, p4, p5))
	}
}

/// COM interface ID. Just a safe abstraction over a [`GUID`](crate::GUID).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct IID(GUID);

impl From<GUID> for IID {
	fn from(guid: GUID) -> Self {
		Self(guid)
	}
}

impl AsRef<GUID> for IID {
	fn as_ref(&self) -> &GUID {
		&self.0
	}
}

impl std::fmt::Display for IID {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl IID {
	/// Creates a new `IID` from hex numbers, which can be copied straight from
	/// standard `IID` definitions.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::IID;
	///
	/// let g = IID::new(0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046);
	/// ```
	pub const fn new(p1: u32, p2: u16, p3: u16, p4: u16, p5: u64) -> IID {
		Self(GUID::new(p1, p2, p3, p4, p5))
	}
}
