use crate::{co, kernel};
use crate::kernel::decl::{MultiByteToWideChar, Encoding, SysResult};

/// Stores a `Vec<u16>` buffer for a null-terminated
/// [Unicode UTF-16](https://learn.microsoft.com/en-us/windows/win32/intl/unicode-in-the-windows-api)
/// wide string natively used by Windows.
///
/// This is struct is mostly used internally by the library, as a bridge between
/// Windows and Rust strings.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[derive(Clone, Debug)]
pub struct WString {
	vec_u16: Vec<u16>,
}

impl Default for WString {
	fn default() -> Self {
		Self { vec_u16: Vec::<u16>::default() }
	}
}

impl std::fmt::Display for WString {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl WString {
	/// Creates and stores a new UTF-16 string from an optional
	/// [`&str`](https://doc.rust-lang.org/std/primitive.str.html).
	///
	/// The string will be stored with a terminating null.
	///
	/// If `s` is `None`, the internal buffer is not allocated – it simply calls
	/// `WString::default`.
	#[must_use]
	pub fn from_opt_str(s: Option<&str>) -> WString {
		match s {
			Some(s) => Self::from_str(s),
			None => Self::default(),
		}
	}

	/// Creates and stores a new UTF-16 string from an ordinary
	/// [`&str`](https://doc.rust-lang.org/std/primitive.str.html).
	///
	/// The string will be stored with a terminating null.
	#[must_use]
	pub fn from_str(s: &str) -> WString {
		Self {
			vec_u16: s.encode_utf16()
				.chain(std::iter::once(0x0000)) // append a terminating null
				.collect(),
		}
	}

	/// Creates and stores a new UTF-16 string from a
	/// [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) of ordinary
	/// strings. This new string will be stored as sequential null-separated
	/// strings, terminated with two nulls. That means that further retrieval
	/// operations will "see" only the first string.
	///
	/// This method is intended to pass multi-strings to native APIs, not to
	/// retrieve them.
	#[must_use]
	pub fn from_str_vec(v: &[impl AsRef<str>]) -> WString {
		let tot_chars = v.iter() // number of chars of all strings, including terminating nulls
			.fold(0, |tot, s| tot + s.as_ref().chars().count() + 1) // including terminating null
				+ 1; // double terminating null

		let mut vec_u16 = Vec::<u16>::with_capacity(tot_chars); // prealloc
		v.iter().for_each(|s|
			vec_u16.extend(
				s.as_ref().encode_utf16() // convert each string
					.chain(std::iter::once(0x0000)) // append a terminating null
			),
		);
		vec_u16.push(0x0000); // double terminating null

		Self { vec_u16 }
	}

	/// Creates a new UTF-16 string by copying from a buffer, specifying the
	/// number of existing chars, not counting a terminating null.
	///
	/// The string will be stored with a terminating null.
	#[must_use]
	pub fn from_wchars_count(src: *const u16, num_chars: usize) -> WString {
		if src.is_null() || num_chars == 0 {
			Self::default()
		} else {
			let tot_chars = num_chars + 1; // add room for terminating null
			let mut vec_u16 = vec![0x0000; tot_chars]; // alloc right away
			unsafe {
				src.copy_to_nonoverlapping(vec_u16.as_mut_ptr(), tot_chars - 1); // no terminating null to copy
				*vec_u16.get_unchecked_mut(tot_chars - 1) = 0x0000; // terminating null
			}

			Self { vec_u16 }
		}
	}

	/// Creates a new UTF-16 string by copying from a null-terminated buffer.
	///
	/// The string will be stored with a terminating null.
	#[must_use]
	pub fn from_wchars_nullt(src: *const u16) -> WString {
		if src.is_null() {
			Self::default()
		} else {
			Self::from_wchars_count(
				src, unsafe { kernel::ffi::lstrlenW(src) } as _)
		}
	}

	/// Creates a new UTF-16 string by copying from a slice.
	///
	/// The string will be stored with a terminating null.
	#[must_use]
	pub fn from_wchars_slice(src: &[u16]) -> WString {
		Self::from_wchars_count(src.as_ptr(), src.len())
	}

	/// Creates a new UTF-16 buffer allocated with an specific length. All
	/// UTF-16 chars will be set to zero.
	#[must_use]
	pub fn new_alloc_buf(num_chars: usize) -> WString {
		Self { vec_u16: vec![0x0000; num_chars] }
	}

	/// Returns a
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// mut pointer to the internal UTF-16 string buffer, to be passed to native
	/// Win32 functions. This is useful to receive strings.
	///
	/// # Panics
	///
	/// Panics if the buffer wasn't previously allocated. Be sure to alloc
	/// enough room, otherwise a buffer overrun may occur.
	#[must_use]
	pub unsafe fn as_mut_ptr(&mut self) -> *mut u16 {
		if self.vec_u16.is_empty() {
			panic!("Trying to use an unallocated WString buffer.")
		} else {
			self.vec_u16.as_mut_ptr()
		}
	}

	/// Returns a
	/// [`LPCWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// const pointer to the internal UTF-16 string buffer, to be passed to
	/// native Win32 functions.
	///
	/// **Note:** Returns a null pointer if the buffer wasn't previously
	/// allocated. Make sure the `WString` object outlives the function call,
	/// otherwise it will point to an invalid memory location.
	#[must_use]
	pub unsafe fn as_ptr(&self) -> *const u16 {
		if self.vec_u16.is_empty() {
			std::ptr::null()
		} else {
			self.vec_u16.as_ptr()
		}
	}

	/// Returns a slice to the internal
	/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) buffer. This
	/// is useful to receive strings.
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [u16] {
		self.vec_u16.as_mut_slice()
	}

	/// Returns a slice to the internal UTF-16 string buffer.
	#[must_use]
	pub fn as_slice(&self) -> &[u16] {
		self.vec_u16.as_slice()
	}

	/// Tells whether the internal bufer has not been allocated.
	#[must_use]
	pub fn buf_is_empty(&self) -> bool {
		self.vec_u16.is_empty()
	}

	/// Returns the size of the allocated internal buffer.
	///
	/// If the buffer was not allocated yet, returns zero.
	#[must_use]
	pub fn buf_len(&self) -> usize {
		self.vec_u16.len()
	}

	/// Resizes the internal buffer, to be used as a buffer for native Win32
	/// functions. All UTF-16 chars will be set to zero.
	///
	/// The underlying `Vec` will be resized with a call to
	/// [`Vec::resize`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.resize).
	///
	/// If the new size is zero, the internal buffer is deallocated.
	///
	/// **Note:** The internal memory can be moved after a realloc, so if you're
	/// using a pointer to the internal buffer, it may then point to an invalid
	/// memory location. After a realloc, the following methods must be called
	/// again:
	/// * [`as_mut_ptr`](crate::WString::as_mut_ptr);
	/// * [`as_ptr`](crate::WString::as_ptr).
	pub unsafe fn buf_realloc(&mut self, new_size: usize) {
		if new_size == 0 {
			self.vec_u16 = Vec::<u16>::default();
		} else {
			self.vec_u16.resize(new_size, 0x0000);
		}
	}

	/// Copies the content into an external buffer. A terminating null will be
	/// appended.
	///
	/// If `dest` is smaller, the string will be truncated.
	///
	/// If `dest` has 1 element, it will receive only the terminating null.
	pub fn copy_to_slice(&self, dest: &mut [u16]) {
		if dest.is_empty() {
			// Do nothing.
		} else if self.vec_u16.is_empty() {
			dest.iter_mut()
				.for_each(|dest| *dest = 0x0000);
		} else {
			let num_chars = std::cmp::min(self.vec_u16.len() - 1, dest.len() - 1); // no terminating null
			let src_wri = &self.vec_u16[..num_chars];
			let dest_wri = &mut dest[..num_chars];

			src_wri.iter()
				.zip(dest_wri.iter_mut())
				.for_each(|(src_ch, dest_ch)| *dest_ch = *src_ch);

			dest.iter_mut()
				.skip(num_chars)
				.for_each(|dest_ch| *dest_ch = 0x0000);
		}
	}

	/// Fills the entire buffer with zero values. The buffer size is not
	/// changed.
	pub fn fill_with_zero(&mut self) {
		self.vec_u16.iter_mut()
			.for_each(|ch| *ch = 0x0000);
	}

	/// Wrapper to
	/// [`lstrlen`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lstrlenw).
	///
	/// Returns the number of
	/// [`u16`](https://doc.rust-lang.org/std/primitive.u16.html) characters
	/// stored in the internal buffer, not counting the terminating null.
	#[must_use]
	pub fn str_len(&self) -> usize {
		unsafe { kernel::ffi::lstrlenW(self.as_ptr()) as _ }
	}

	/// Converts into
	/// [`String`](https://doc.rust-lang.org/std/string/struct.String.html). An
	/// unallocated buffer will simply be converted into an empty string.
	///
	/// # Panics
	///
	/// Panics if any invalid character is found.
	///
	/// If you're parsing raw data which may contain errors, prefer using
 	/// [`to_string_checked`](crate::WString::to_string_checked) instead.
	#[must_use]
	pub fn to_string(&self) -> String {
		self.to_string_checked().unwrap()
	}

	/// Converts into
	/// [`String`](https://doc.rust-lang.org/std/string/struct.String.html) by
	/// calling
	/// [`String::from_utf16`](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf16).
	/// An uncallocated will simply be converted into an empty string.
	///
	/// This method is useful if you're parsing raw data which may contain
	/// invalid characters. If you're dealing with a string known to be valid,
	/// [`to_string`](crate::WString::to_string) is more practical.
	#[must_use]
	pub fn to_string_checked(&self) -> Result<String, std::string::FromUtf16Error> {
		if self.vec_u16.is_empty() {
			Ok(String::default())
		} else {
			String::from_utf16(&self.vec_u16[..self.str_len()]) // without terminating null
		}
	}

	/// Guesses the encoding with [`Encoding::guess`](crate::Encoding::guess)
	/// and parses the data as a string.
	///
	/// If you're sure the data has UTF-8 encoding, you can also use the
	/// built-in
	/// [`String::from_utf8`](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8).
	///
	/// To serialize the string back into UTF-8 bytes, use the built-in
	/// [`String::into_bytes`](https://doc.rust-lang.org/std/string/struct.String.html#method.into_bytes).
	///
	/// # Examples
	///
	/// Parsing a file as string by [memory-mapping](crate::FileMapped) the file
	/// (usually the fastest method):
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{FileAccess, FileMapped, WString};
	///
	/// let file_in = FileMapped::open("C:\\Temp\\foo.txt", FileAccess::ExistingReadOnly)?;
	/// let str_contents = WString::parse(file_in.as_slice())?.to_string();
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	pub fn parse(data: &[u8]) -> SysResult<WString> {
		let mut data = data;
		if data.is_empty() { // nothing to parse
			return Ok(WString::default());
		}

		let (encoding, sz_bom) = Encoding::guess(data);
		data = &data[sz_bom..]; // skip BOM, if any

		let wstr = WString::from_wchars_slice(
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
		);
		Ok(wstr)
	}

	#[must_use]
	fn parse_ansi(data: &[u8]) -> Vec<u16> {
		let the_len = data.iter()
			.position(|ch| *ch == 0x00) // a terminating null means end of the string
			.unwrap_or(data.len());

		let mut str16 = Vec::<u16>::with_capacity(the_len + 1); // room for terminating null
		data.iter().for_each(|by| str16.push(*by as _)); // u8 to u16 raw conversion
		str16.push(0x0000); // terminating null
		str16
	}

	#[must_use]
	fn parse_utf16(data: &[u8], is_big_endian: bool) -> Vec<u16> {
		let data = if data.len() % 2 == 1 {
			&data[..data.len() - 1] // if odd number of bytes, discard last one
		} else {
			data
		};

		let the_len = data.chunks(2)
			.position(|ch2| ch2 == &[0x00, 0x00]) // a terminating null means end of the string
			.unwrap_or(data.len() / 2);

		let mut str16 = Vec::<u16>::with_capacity(the_len + 1); // room for terminating null
		data.chunks(2)
			.for_each(|ch2| {
				str16.push(
					if is_big_endian {
						u16::from_be_bytes(ch2.try_into().unwrap())
					} else {
						u16::from_le_bytes(ch2.try_into().unwrap())
					},
				);
			});
		str16.push(0x0000); // terminating null
		str16
	}
}
