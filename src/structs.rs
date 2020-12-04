/// [`GUID`](https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid)
/// struct.
#[repr(C)]
pub struct GUID {
	data1: u32,
	data2: u16,
	data3: u16,
	data4: u64,
}

impl GUID {
	/// Creates a new GUID from hex numbers, which can be copied straight from
	/// standard GUID definitions.
	///
	/// Example for `IUnknown`:
	/// ```
	/// let g = GUID::new(0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046);
	/// ```
	pub fn new(p1: u32, p2: u16, p3: u16, p4: u16, p5: u64) -> GUID {
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