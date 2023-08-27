use crate::co;
use crate::decl::*;
use crate::kernel::ffi;

pub const SSO_LEN: usize = 20;

/// Stores a `[u16]` buffer for a null-terminated
/// [Unicode UTF-16](https://learn.microsoft.com/en-us/windows/win32/intl/unicode-in-the-windows-api)
/// wide string natively used by Windows.
///
/// Uses
/// [Short String Optimization](https://joellaity.com/2020/01/31/string.html)
/// technique for faster performance.
///
/// This is struct is mostly used internally by the library, as a bridge between
/// Windows and Rust strings.
#[derive(Default, Clone)]
pub struct WString {
	buf: Buffer,
}

impl std::fmt::Display for WString {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.buf.to_string_checked().unwrap())
	}
}

impl std::fmt::Debug for WString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		<Buffer as std::fmt::Debug>::fmt(&self.buf, f) // simply delegate
	}
}

impl PartialEq for WString {
	fn eq(&self, other: &Self) -> bool {
		<Buffer as PartialEq>::eq(&self.buf, &other.buf)
    }
}

impl Eq for WString {}

impl PartialOrd for WString {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		<Buffer as PartialOrd>::partial_cmp(&self.buf, &other.buf)
    }
}

impl Ord for WString {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		<Buffer as Ord>::cmp(&self.buf, &other.buf)
    }
}

impl WString {
	/// Stores an UTF-16 null-terminated string from an optional [`&str`](str).
	///
	/// If `s` is `None`, no allocation is made.
	#[must_use]
	pub fn from_opt_str(s: Option<impl AsRef<str>>) -> Self {
		Self { buf: Buffer::from_opt_str(s) }
	}

	/// Stores an UTF-16 null-terminated string from a [`&str`](str).
	#[must_use]
	pub fn from_str(s: impl AsRef<str>) -> Self {
		Self { buf: Buffer::from_str(s) }
	}

	/// Stores a series of UTF-16 null-terminated strings. The buffer will end
	/// with two terminating nulls – that means further retrieval operations
	/// will "see" only the first string.
	///
	/// This method is used internally by the library, and not intended to be
	/// used externally.
	#[must_use]
	pub fn from_str_vec(v: &[impl AsRef<str>]) -> Self {
		Self { buf: Buffer::from_str_vec(v) }
	}

	/// Stores an UTF-16 null-terminated string by copying from a buffer,
	/// specifying the number of chars to be copied.
	///
	/// The `src` buffer doesn't need to be null-terminated.
	#[must_use]
	pub fn from_wchars_count(src: *const u16, num_chars: usize) -> Self {
		Self { buf: Buffer::from_wchars_count(src, num_chars) }
	}

	/// Stores an UTF-16 null-terminated string by copying from a
	/// null-terminated buffer. The string length is retrieved with
	/// [`lstrlen`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lstrlenw).
	#[must_use]
	pub fn from_wchars_nullt(src: *const u16) -> Self {
		Self { buf: Buffer::from_wchars_nullt(src) }
	}

	/// Stores an UTF-16 null-terminated string by copying from a slice.
	///
	/// The `src` slice doesn't need to be null-terminated.
	#[must_use]
	pub fn from_wchars_slice(src: &[u16]) -> Self {
		Self { buf: Buffer::from_wchars_slice(src) }
	}

	/// Allocates an UTF-16 buffer with an specific length. All elements will be
	/// set to zero.
	#[must_use]
	pub fn new_alloc_buf(sz: usize) -> Self {
		Self { buf: Buffer::new_alloc_buf(sz) }
	}

	/// Returns a mutable
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// pointer to the internal UTF-16 string buffer, to be passed to native
	/// Win32 functions. This is useful to receive strings.
	///
	/// # Panics
	///
	/// Panics if the buffer was not allocated.
	///
	/// # Safety
	///
	/// Be sure to alloc enough room, otherwise a buffer overrun may occur.
	#[must_use]
	pub unsafe fn as_mut_ptr(&mut self) -> *mut u16 {
		self.buf.as_mut_ptr()
	}

	/// Returns a mutable slice to the internal UTF-16 string buffer.
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [u16] {
		self.buf.as_mut_slice()
	}

	/// Returns a
	/// [`LPCWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// pointer to the internal UTF-16 string buffer, to be passed to native
	/// Win32 functions.
	///
	/// If the buffer was not allocated, returns a null pointer.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		self.buf.as_ptr()
	}

	/// Returns a slice to the internal UTF-16 string buffer.
	#[must_use]
	pub fn as_slice(&self) -> &[u16] {
		self.buf.as_slice()
	}

	/// Returns the size of the allocated internal buffer. Note that the
	/// terminating null, if existing, is also counted.
	///
	/// If the buffer was not allocated yet, returns zero.
	#[must_use]
	pub const fn buf_len(&self) -> usize {
		self.buf.buf_len()
	}

	/// Copies the content into an external buffer. A terminating null will be
	/// appended.
	///
	/// If `dest` is smaller, the string will be truncated.
	///
	/// If `dest` has 1 element, it will receive only the terminating null.
	pub fn copy_to_slice(&self, dest: &mut [u16]) {
		if !dest.is_empty() {
			let usable_len = dest.len() - 1; // leave room for terminating null
			self.as_slice()
				.iter()
				.zip(dest[..usable_len].iter_mut())
				.for_each(|(src, dest)| *dest = *src);
			dest[usable_len..].iter_mut()
				.for_each(|dest| *dest = 0x0000); // fill the rest with zero
		}
	}

	/// Fills the entire buffer with zeros.
	pub fn fill_with_zero(&mut self) {
		self.as_mut_slice()
			.iter_mut()
			.for_each(|ch| *ch = 0x0000);
	}

	/// Returns `true` if the internal buffer has been allocated.
	#[must_use]
	pub const fn is_allocated(&self) -> bool {
		self.buf.is_allocated()
	}

	/// Converts into [`String`](std::string::String) by calling
	/// [`String::from_utf16`](std::string::String::from_utf16). An uncallocated
	/// will simply be converted into an empty string.
	///
	/// This method is useful if you're parsing raw data which may contain
	/// invalid characters. If you're dealing with a string known to be valid,
	/// [`to_string`](std::string::ToString::to_string) is more practical.
	#[must_use]
	pub fn to_string_checked(&self
	) -> Result<String, std::string::FromUtf16Error>
	{
		self.buf.to_string_checked()
	}

	/// Wrapper to
	/// [`lstrlen`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lstrlenw).
	///
	/// Returns the number of [`u16`] characters stored in the internal buffer,
	/// not counting the terminating null.
	#[must_use]
	pub fn str_len(&self) -> usize {
		unsafe { ffi::lstrlenW(self.buf.as_ptr()) as _ }
	}

	/// Guesses the encoding with [`Encoding::guess`](crate::Encoding::guess)
	/// and parses the data as a string.
	///
	/// If you're sure the data has UTF-8 encoding, you can also use the
	/// built-in [`String::from_utf8`](std::string::String::from_utf8).
	///
	/// To serialize the string back into UTF-8 bytes, use the built-in
	/// [`String::into_bytes`](std::string::String::into_bytes).
	///
	/// # Examples
	///
	/// Usually the fastest way to read the text from a file is by mapping its
	/// contents in memory with [`FileMapped`](crate::FileMapped), then parsing:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let file_in = w::FileMapped::open(
	///     "C:\\Temp\\foo.txt",
	///     w::FileAccess::ExistingReadOnly,
	/// )?;
	/// let wstr = w::WString::parse(file_in.as_slice())?;
	/// let str_contents = wstr.to_string();
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	pub fn parse(data: &[u8]) -> SysResult<Self> {
		let mut data = data;
		if data.is_empty() { // nothing to parse
			return Ok(Self::default());
		}

		let (encoding, sz_bom) = Encoding::guess(data);
		data = &data[sz_bom..]; // skip BOM, if any

		Ok(Self::from_wchars_slice(
			&match encoding {
				Encoding::Ansi => Self::parse_ansi(data),
				Encoding::Win1252 => MultiByteToWideChar(co::CP::WINDOWS_1252, co::MBC::NoValue, data)?,
				Encoding::Utf8 => MultiByteToWideChar(co::CP::UTF8, co::MBC::NoValue, data)?,
				Encoding::Utf16be => Self::parse_utf16(data, true),
				Encoding::Utf16le => Self::parse_utf16(data, false),
				Encoding::Utf32be
				| Encoding::Utf32le
				| Encoding::Scsu
				| Encoding::Bocu1
				| Encoding::Unknown => panic!("Encoding {} not implemented.", encoding),
			}
		))
	}

	fn parse_ansi(data: &[u8]) -> Vec<u16> {
		data.iter()
			.take_while(|ch| **ch != 0x0000) // ignore terminating null, if any
			.map(|ch| *ch as u16) // raw u8 to u16 conversion
			.collect()
	}

	fn parse_utf16(data: &[u8], is_big_endian: bool) -> Vec<u16> {
		let data = if data.len() % 2 == 1 {
			&data[..data.len() - 1] // if odd number of bytes, discard last one
		} else {
			data
		};

		data.chunks(2)
			.take_while(|ch2| **ch2 != [0x00, 0x00]) // ignore terminating null, if any
			.map(|ch2| {
				if is_big_endian {
					u16::from_be_bytes(ch2.try_into().unwrap())
				} else {
					u16::from_le_bytes(ch2.try_into().unwrap())
				}
			})
			.collect()
	}
}

//------------------------------------------------------------------------------

enum Buffer {
	Stack([u16; SSO_LEN]),
	Heap(HeapBlock),
	Unallocated,
}

impl Default for Buffer {
	fn default() -> Self {
		Self::Unallocated
	}
}

impl Clone for Buffer {
	fn clone(&self) -> Self {
		match self {
			Self::Unallocated => Self::Unallocated,
			_ => {
				let mut new_self = Self::new_alloc_buf(self.buf_len());
				self.as_slice()
					.iter()
					.zip(new_self.as_mut_slice())
					.for_each(|(src, dest)| *dest = *src);
				new_self
			},
		}
	}
}

impl std::fmt::Debug for Buffer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Self::Stack(arr) =>
				format!("STACK({}) \"{}\"", arr.len(), self.to_string_checked().unwrap()),
			Self::Heap(block) =>
				format!("HEAP({}) \"{}\"",
					block.len() / std::mem::size_of::<u16>(), self.to_string_checked().unwrap()),
			Self::Unallocated =>
				"(UNALLOCATED)".to_owned(),
		})
	}
}

impl PartialEq for Buffer {
	fn eq(&self, other: &Self) -> bool {
		self.as_slice().eq(other.as_slice())
    }
}

impl Eq for Buffer {}

impl PartialOrd for Buffer {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
    }
}

impl Ord for Buffer {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.as_slice().cmp(other.as_slice())
    }
}

impl Buffer {
	fn from_opt_str(s: Option<impl AsRef<str>>) -> Self {
		match s {
			Some(s) => Self::from_str(s),
			None => Self::Unallocated,
		}
	}

	fn from_str(s: impl AsRef<str>) -> Self {
		let s_len = s.as_ref().encode_utf16().count();
		if s_len == 0 {
			Self::Unallocated
		} else {
			let num_chars = s_len + 1; // room for terminating null
			let mut new_self = Self::new_alloc_buf(num_chars);
			s.as_ref()
				.encode_utf16()
				.into_iter()
				.zip(new_self.as_mut_slice())
				.for_each(|(src, dest)| *dest = src);
			new_self
		}
	}

	fn from_str_vec(v: &[impl AsRef<str>]) -> Self {
		let tot_chars = v.iter() // number of chars of all strings, including terminating nulls
			.fold(0, |tot, s| tot + s.as_ref().chars().count() + 1) // include terminating null
			+ 1; // double terminating null
		let mut new_self = Self::new_alloc_buf(tot_chars);
		v.iter()
			.map(|s| {
				s.as_ref()
					.encode_utf16()
					.into_iter()
			})
			.flatten()
			.zip(new_self.as_mut_slice())
			.for_each(|(src, dest)| *dest = src);
		new_self
	}

	fn from_wchars_count(src: *const u16, num_chars: usize) -> Self {
		if src.is_null() || num_chars == 0 {
			Self::Unallocated
		} else {
			Self::from_wchars_slice(
				unsafe { std::slice::from_raw_parts(src, num_chars) },
			)
		}
	}

	fn from_wchars_nullt(src: *const u16) -> Self {
		Self::from_wchars_count(src, unsafe { ffi::lstrlenW(src) as _ })
	}

	fn from_wchars_slice(src: &[u16]) -> Self {
		if src.is_empty() {
			Self::Unallocated
		} else {
			let num_chars = src.iter()
				.take_while(|ch| **ch != 0x0000) // skip terminating null, if any
				.count()
				+ 1; // room for terminating null
			let mut new_self = Self::new_alloc_buf(num_chars);
			src.iter()
				.take_while(|ch| **ch != 0x0000) // skip terminating null, if any
				.zip(new_self.as_mut_slice())
				.for_each(|(src, dest)| *dest = *src);
			new_self
		}
	}

	fn new_alloc_buf(num_chars: usize) -> Self {
		if num_chars == 0 {
			Self::Unallocated
		} else if num_chars <= SSO_LEN {
			Self::Stack([0x0000; SSO_LEN])
		} else {
			Self::Heap(
				HeapBlock::alloc(num_chars * std::mem::size_of::<u16>())
					.unwrap(), // assume no allocation errors
			)
		}
	}

	unsafe fn as_mut_ptr(&mut self) -> *mut u16 {
		match self {
			Self::Stack(arr) => arr.as_mut_ptr(),
			Self::Heap(block) => block.as_mut_ptr() as _,
			Self::Unallocated => panic!("Trying to use an unallocated WString buffer."),
		}
	}

	fn as_mut_slice(&mut self) -> &mut [u16] {
		match self {
			Self::Stack(arr) => arr,
			Self::Heap(block) => unsafe { block.as_mut_slice_aligned::<_>() },
			Self::Unallocated => &mut [],
		}
	}

	fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Stack(arr) => arr.as_ptr(),
			Self::Heap(block) => block.as_ptr() as _,
			Self::Unallocated => std::ptr::null(),
		}
	}

	fn as_slice(&self) -> &[u16] {
		match self {
			Self::Stack(arr) => arr,
			Self::Heap(block) => unsafe { block.as_slice_aligned::<_>() },
			Self::Unallocated => &[],
		}
	}

	const fn buf_len(&self) -> usize {
		match self {
			Self::Stack(arr) => arr.len(),
			Self::Heap(block) => block.len() / std::mem::size_of::<u16>(),
			Self::Unallocated => 0,
		}
	}

	const fn is_allocated(&self) -> bool {
		match self {
			Self::Unallocated => false,
			_ => true,
		}
	}

	fn to_string_checked(&self) -> Result<String, std::string::FromUtf16Error> {
		match self {
			Self::Unallocated => Ok(String::default()),
			_ => String::from_utf16(
				&self.as_slice()
					.into_iter()
					.take_while(|ch| **ch != 0x0000) // remove all trailing zeros
					.map(|ch| *ch)
					.collect::<Vec<_>>(),
			),
		}
	}
}
