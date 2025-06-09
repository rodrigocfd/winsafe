/// [`GUID`](https://learn.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid)
/// struct.
///
/// The [`Default`](std::default::Default) implementation returns `GUID::NULL`
/// (all zeros). To create a new random `GUID`, use
/// [`CoCreateGuid`](crate::CoCreateGuid).
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GUID {
	data1: u32,
	data2: u16,
	data3: u16,
	data4: u64,
}

impl std::fmt::Display for GUID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
			self.data1,
			self.data2,
			self.data3,
			self.data4.swap_bytes() >> 48,
			self.data4.swap_bytes() & 0x0000_ffff_ffff_ffff,
		)
	}
}
impl std::fmt::Debug for GUID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self, f)
	}
}

impl GUID {
	/// The null `GUID`, with all zeros.
	pub const NULL: Self = Self::from_str("00000000-0000-0000-0000-000000000000");

	/// Creates a new `GUID` from a representative hex string, which can be
	/// copied straight from standard `GUID` declarations.
	///
	/// # Panics
	///
	/// Panics if the string has an invalid format.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let g = w::GUID::from_str("43826d1e-e718-42ee-bc55-a1e261c37bfe");
	/// ```
	#[must_use]
	pub const fn from_str(guid_str: &str) -> Self {
		if guid_str.len() != 36 {
			panic!("Bad number of GUID chars.");
		}

		let chs = guid_str.as_bytes();
		let p1 =
			Self::parse_block([chs[0], chs[1], chs[2], chs[3], chs[4], chs[5], chs[6], chs[7]]);
		let p2 = Self::parse_block([chs[9], chs[10], chs[11], chs[12]]);
		let p3 = Self::parse_block([chs[14], chs[15], chs[16], chs[17]]);
		let p4 = Self::parse_block([chs[19], chs[20], chs[21], chs[22]]);
		let p5 = Self::parse_block([
			chs[24], chs[25], chs[26], chs[27], chs[28], chs[29], chs[30], chs[31], chs[32],
			chs[33], chs[34], chs[35],
		]);

		Self {
			data1: p1 as _,
			data2: p2 as _,
			data3: p3 as _,
			data4: ((p4 << 48) | p5).swap_bytes(),
		}
	}

	#[must_use]
	const fn parse_block<const N: usize>(chars: [u8; N]) -> u64 {
		let mut res: u64 = 0;
		let mut idx: usize = 0;
		while idx < N {
			let ch = chars[idx];
			if !Self::valid_char(ch) {
				panic!("Bad GUID char.");
			}
			res += Self::char_to_num(ch) * 16_u64.pow((N - idx - 1) as _);
			idx += 1;
		}
		res
	}

	#[must_use]
	const fn valid_char(ch: u8) -> bool {
		(ch >= 48 && ch <= 57) // 0-9
			|| (ch >= 65 && ch <= 70) // A-F
			|| (ch >= 97 && ch <= 102) // a-f
	}

	#[must_use]
	const fn char_to_num(ch: u8) -> u64 {
		if ch >= 48 && ch <= 57 {
			ch as u64 - 48
		} else if ch >= 65 && ch <= 70 {
			ch as u64 - 65 + 10
		} else if ch >= 97 && ch <= 102 {
			ch as u64 - 97 + 10
		} else {
			panic!("Bad GUID char in conversion.");
		}
	}
}
