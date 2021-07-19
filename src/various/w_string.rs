use std::cmp::Ordering;

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::MultiByteToWideChar;

/// String encodings that can be guessed by
/// [`WString::guess_encoding`](crate::WString::guess_encoding).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
	Unknown,
	Ansi,
	Win1252,
	Utf8,
	Utf16be,
	Utf16le,
	Utf32be,
	Utf32le,
	Scsu,
	Bocu1,
}

impl std::fmt::Display for Encoding {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", match self {
			Self::Unknown => "Unknown",
			Self::Ansi => "ANSI",
			Self::Win1252 => "Windows 1252",
			Self::Utf8 => "UTF-8",
			Self::Utf16be => "UTF-16 BE",
			Self::Utf16le => "UTF-16 LE",
			Self::Utf32be => "UTF-32 BE",
			Self::Utf32le => "UTF-32 LE",
			Self::Scsu => "SCSU",
			Self::Bocu1 => "BOCU1",
		})
	}
}

/// Stores a `Vec<u16>` buffer for a null-terminated
/// [Unicode UTF-16](https://docs.microsoft.com/en-us/windows/win32/intl/unicode-in-the-windows-api)
/// wide string natively used by Windows.
///
/// Performs UTF-8 conversions and can be used as a buffer to low-level Win32
/// functions.
///
/// See an example in [`HWND::GetWindowText`](crate::HWND::GetWindowText).
#[derive(Clone)]
pub struct WString {
	vec_u16: Option<Vec<u16>>,
}

impl Default for WString {
	fn default() -> Self {
		Self { vec_u16: None }
	}
}

impl std::fmt::Display for WString {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl WString {
	/// Creates and stores a new UTF-16 string from an optional `&str`.
	///
	/// The string will be stored with a terminating null.
	pub fn from_opt_str(s: Option<&str>) -> WString {
		Self {
			vec_u16: s.map(
				|s| s.encode_utf16()
					.chain(std::iter::once(0x0000)) // append a terminating null
					.collect(),
			)
		}
	}

	/// Creates and stores a new UTF-16 string from an ordinary `&str`.
	///
	/// The string will be stored with a terminating null.
	pub fn from_str(s: &str) -> WString {
		Self::from_opt_str(Some(s))
	}

	/// Creates and stores a new UTF-16 string from a `Vec` of ordinary strings.
	/// This new string will be stored as sequential null-separated strings,
	/// terminated with two nulls. That means that further retrieval operations
	/// will "see" only the first string.
	///
	/// This method is intended to pass multi-strings to native APIs, not to
	/// retrieve them.
	pub fn from_str_vec<S: AsRef<str>>(v: &[S]) -> WString {
		let mut tot_chars = 0; // number of chars of all strings, including terminating nulls
		for s in v.iter() {
			tot_chars = s.as_ref().len() + 1; // including terminating null
		}
		tot_chars += 1; // double terminating null

		let mut buf16 = Vec::with_capacity(tot_chars);
		for s in v.iter() {
			buf16.extend(
				s.as_ref().encode_utf16()
					.chain(std::iter::once(0x0000)) // append a terminating null
			);
		}
		buf16.push(0x0000); // double terminating null

		Self { vec_u16: Some(buf16) }
	}

	/// Creates a new UTF-16 string by copying from a buffer, specifying the
	/// number of existing chars, not counting a terminating null.
	///
	/// The string will be stored with a terminating null.
	pub fn from_wchars_count(src: *const u16, num_chars: usize) -> WString {
		if src.is_null() || num_chars == 0 {
			Self::default()
		} else {
			let tot_chars = num_chars + 1; // add room for terminating null
			let mut buf16 = vec![0x0000; tot_chars];
			unsafe {
				src.copy_to_nonoverlapping(buf16.as_mut_ptr(), tot_chars - 1); // no terminating null to copy
				*buf16.get_unchecked_mut(tot_chars - 1) = 0x0000;
			}

			Self { vec_u16: Some(buf16) }
		}
	}

	/// Creates a new UTF-16 string by copying from a null-terminated buffer.
	///
	/// The string will be stored with a terminating null.
	pub fn from_wchars_nullt(src: *const u16) -> WString {
		if src.is_null() {
			Self::default()
		} else {
			Self::from_wchars_count(src, unsafe { kernel32::lstrlenW(src) } as _)
		}
	}

	/// Creates a new UTF-16 string by copying from a slice.
	///
	/// The string will be stored with a terminating null.
	pub fn from_wchars_slice(src: &[u16]) -> WString {
		Self::from_wchars_count(src.as_ptr(), src.len())
	}

	/// Creates a new UTF-16 buffer allocated with an specific length. All
	/// UTF-16 chars will be set to zero.
	pub fn new_alloc_buffer(num_chars: usize) -> WString {
		let mut me = Self::default();
		me.realloc_buffer(num_chars);
		me
	}

	/// Returns a `LPWSTR` mut pointer to the internal UTF-16 string buffer, to
	/// be passed to native Win32 functions. This is useful to receive strings.
	///
	/// # Panics
	///
	/// Panics if the buffer wasn't previously allocated. Be sure to alloc
	/// enough room, otherwise a buffer overrun may occur.
	pub unsafe fn as_mut_ptr(&mut self) -> *mut u16 {
		self.vec_u16.as_mut()
			.map_or_else(
				|| panic!("Trying to use an unallocated WString buffer."),
				|v| v.as_mut_ptr(),
			)
	}

	/// Returns a `LPCWSTR` const pointer to the internal UTF-16 string buffer,
	/// to be passed to native Win32 functions.
	///
	/// **Note:** Returns a null pointer if the buffer wasn't previously
	/// allocated. Make sure the `WString` object outlives the function call,
	/// otherwise it will point to an invalid memory location.
	pub unsafe fn as_ptr(&self) -> *const u16 {
		self.vec_u16.as_ref()
			.map_or(std::ptr::null(), |v| v.as_ptr())
	}

	/// Returns a slice to the internal `u16` buffer. This is useful to receive
	/// strings.
	///
	/// # Panics
	///
	/// Panics if the buffer wasn't previously allocated. Be sure to alloc
	/// enough room, otherwise a buffer overrun may occur.
	pub fn as_mut_slice(&mut self) -> &mut [u16] {
		self.vec_u16.as_mut()
			.map_or_else(
				|| panic!("Trying to use an unallocated WString buffer."),
				|v| v.as_mut_slice(),
			)
	}

	/// Returns a slice to the internal UTF-16 string buffer.
	///
	/// # Panics
	///
	/// Panics if the buffer wasn't previously allocated. Make sure the
	/// `WString` object outlives the function call, otherwise it will point to
	/// an invalid memory location.
	pub fn as_slice(&self) -> &[u16] {
		self.vec_u16.as_ref()
			.map_or_else(
				|| panic!("Trying to use an unallocated WString buffer."),
				|v| v.as_slice(),
			)
	}

	/// Returns the size of the allocated internal buffer.
	///
	/// If the buffer was not allocated yet, returns zero.
	pub fn buffer_size(&self) -> usize {
		self.vec_u16.as_ref()
			.map_or(0, |v| v.len())
	}

	/// Copies the content into an external buffer. A terminating null will be
	/// appended.
	///
	/// If `dest` is smaller, the string will be truncated.
	///
	/// # Panics
	///
	/// Panics if `dest` has zero length. If length is 1, the buffer will
	/// receive a single null char.
	pub fn copy_to_slice(&self, dest: &mut [u16]) {
		if dest.is_empty() {
			panic!("Destination buffer cannot have zero length");
		}

		if let Some(vec_u16_ref) = self.vec_u16.as_ref() {
			let num_chars = std::cmp::min(vec_u16_ref.len() - 1, dest.len() - 1); // no terminating null
			unsafe {
				vec_u16_ref.as_ptr()
					.copy_to_nonoverlapping(dest.as_mut_ptr(), num_chars);

				for i in num_chars..dest.len() {
					*dest.get_unchecked_mut(i) = 0x0000; // zero the rest of the slice
				}
			}
		}
	}

	/// Fills the entire buffer with zero values. The buffer size is not
	/// changed.
	pub fn fill_with_zero(&mut self) {
		if let Some(vec_u16_ref) = self.vec_u16.as_mut() {
			for wchar in vec_u16_ref {
				*wchar = 0x0000;
			}
		}
	}

	/// Tells whether the internal buffer is storing a null string pointer, or
	/// if it's holding a string with a length of zero.
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Tells whether the internal buffer is storing a null string pointer.
	pub fn is_null(&self) -> bool {
		self.vec_u16.is_none()
	}

	/// Wrapper to
	/// [`lstrlen`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lstrlenw).
	///
	/// Returns the number of `u16` characters stored in the internal buffer,
	/// not counting the terminating null.
	pub fn len(&self) -> usize {
		self.vec_u16.as_ref()
			.map_or(0, |v| unsafe { kernel32::lstrlenW(v.as_ptr())} as _ )
	}

	/// Resizes the internal buffer, to be used as a buffer for native Win32
	/// functions. All UTF-16 chars will be set to zero.
	///
	/// If the new size is zero, the internal buffer is deallocated.
	///
	/// **Note:** The internal memory can move after a realloc, so if you're
	/// using the internal buffer somewhere, update it by calling `as_const_ptr`
	/// or `as_mut_buffer` again.
	pub fn realloc_buffer(&mut self, new_size: usize) {
		if new_size == 0 {
			self.vec_u16 = None; // dealloc
		} else {
			if self.vec_u16.is_none() {
				self.vec_u16 = Some(Vec::default()); // create if not yet; default Vec is empty
			}
			self.vec_u16.as_mut().unwrap().resize(new_size, 0x0000); // filled with nulls
		}
	}

	/// Converts into `String`. An internal null pointer will simply be converted
	/// into an empty string.
	///
	/// # Panics
	///
	/// Panics if any invalid character is found.
	///
	/// If you're parsing raw data which may contain errors, prefer using
 	/// [`to_string_checked`](crate::WString::to_string_checked) instead.
	pub fn to_string(&self) -> String {
		self.to_string_checked().unwrap()
	}

	/// Converts into `String` by calling `String::from_utf16`. An internal null
	/// pointer will simply be converted into an empty string.
	///
	/// This method is useful if you're parsing raw data which may contain
	/// invalid characters. If you're dealing with a string known to be valid,
	/// [`to_string`](crate::WString::to_string) is more practical.
	pub fn to_string_checked(&self) -> Result<String, std::string::FromUtf16Error> {
		self.vec_u16.as_ref()
			.map_or(
				Ok(String::default()),
				|v| String::from_utf16(&v[..self.len()]), // without terminating null
			)
	}

	/// Guesses the [`Encoding`](crate::Encoding) of the given data, also
	/// returning the size of its BOM, if any.
	pub fn guess_encoding(data: &[u8]) -> (Encoding, usize) {
		let has_bom = |bom_bytes: &[u8]| -> bool {
			data.len() >= bom_bytes.len()
				&& data[..bom_bytes.len()].cmp(bom_bytes) == Ordering::Equal
		};

		const UTF8: [u8; 3] = [0xef, 0xbb, 0xbf];
		if has_bom(&UTF8) { // UTF-8 BOM
			return (Encoding::Utf8, UTF8.len());
		}

		const UTF16BE: [u8; 2] = [0xfe, 0xff];
		if has_bom(&UTF16BE) {
			return (Encoding::Utf32be, UTF16BE.len());
		}

		const UTF16LE: [u8; 2] = [0xff, 0xfe];
		if has_bom(&UTF16LE) {
			return (Encoding::Utf16le, UTF16LE.len());
		}

		const UTF32BE: [u8; 4] = [0x00, 0x00, 0xfe, 0xff];
		if has_bom(&UTF32BE) {
			return (Encoding::Utf32be, UTF32BE.len())
		}

		const UTF32LE: [u8; 4] = [0xff, 0xfe, 0x00, 0x00];
		if has_bom(&UTF32LE) {
			return (Encoding::Utf32le, UTF32LE.len())
		}

		const SCSU: [u8; 3] = [0x0e, 0xfe, 0xff];
		if has_bom(&SCSU) {
			return (Encoding::Scsu, SCSU.len())
		}

		const BOCU1: [u8; 3] = [0xfb, 0xee, 0x28];
		if has_bom(&BOCU1) {
			return (Encoding::Bocu1, BOCU1.len())
		}

		// No BOM found, guess UTF-8 without BOM, or Windows-1252 (superset of
		// ISO-8859-1).
		let mut can_be_win1252 = false;
		for i in 0..data.len() - 1 {
			if data[i] > 0x7f { // 127
				can_be_win1252 = true;
				if i <= data.len() - 2 && (
					(data[i] == 0xc2 && (data[i+1] >= 0xa1 && data[i+1] <= 0xbf)) || // http://www.utf8-chartable.de
					(data[i] == 0xc3 && (data[i+1] >= 0x80 && data[i+1] <= 0xbf)) )
				{
					return (Encoding::Utf8, 0); // UTF-8 without BOM
				}
			}
		}

		(if can_be_win1252 { Encoding::Win1252 } else { Encoding::Ansi }, 0)
	}

	/// Guesses the encoding with
	/// [`WString::guess_encoding`](crate::WString::guess_encoding) and parses
	/// the data as string.
	pub fn parse_str(data: &[u8]) -> WinResult<WString> {
		let mut data = data;
		if data.is_empty() { // nothing to parse
			return Ok(WString::default());
		}

		let (encoding, sz_bom) = Self::guess_encoding(data);
		data = &data[sz_bom..]; // skip BOM, if any

		Ok(Self {
			vec_u16: Some(match encoding {
				Encoding::Ansi => Self::parse_ansi_str(data),
				Encoding::Win1252 => MultiByteToWideChar(co::CP::WINDOWS_1252, co::MBC::NoValue, data)?,
				Encoding::Utf8 => MultiByteToWideChar(co::CP::UTF8, co::MBC::NoValue, data)?,
				Encoding::Utf16be => Self::parse_utf16_str(data, true),
				Encoding::Utf16le => Self::parse_utf16_str(data, false),
				Encoding::Utf32be
				| Encoding::Utf32le
				| Encoding::Scsu
				| Encoding::Bocu1
				| Encoding::Unknown => panic!("Encoding {} not implemented.", encoding),
			}),
		})
	}

	fn parse_ansi_str(data: &[u8]) -> Vec<u16> {
		let mut the_len = data.len();
		for (idx, by) in data.iter().enumerate() {
			if *by == 0x00 { // found terminating null amidst data, stop processing
				the_len = idx;
				break;
			}
		}

		let mut str16 = Vec::with_capacity(the_len + 1); // room for terminating null
		data.iter().for_each(|by| str16.push(*by as _)); // u8 to u18 raw conversion
		str16.push(0x0000); // terminating null
		str16
	}

	fn parse_utf16_str(data: &[u8], is_big_endian: bool) -> Vec<u16> {
		let data = if data.len() % 2 == 1 {
			&data[..data.len() - 1] // if odd number of bytes, discard last one
		} else {
			data
		};

		let mut str16: Vec<u16> = Vec::with_capacity(data.len() / 2 + 1); // room for terminating null
		for i in (0..data.len()).step_by(2) {
			let (by0, by1) = unsafe {
				(*data.get_unchecked(i), *data.get_unchecked(i + 1))
			};

			if by0 == 0x00 && by1 == 0x00 {
				break; // found terminating null amidst data, stop processing
			}

			let (by0, by1) = (by0 as u16, by1 as u16); // avoid shift left overflow
			str16.push(if is_big_endian {
				(by0 << 8) | by1
			} else {
				by0 | (by1 << 8)
			} as _);
		}

		str16.push(0x0000); // terminating null
		str16
	}
}
