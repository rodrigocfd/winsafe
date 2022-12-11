//! File path utilities.
//!
//! Some of the functions are similar to
//! [`std::path::Path`](https://doc.rust-lang.org/std/path/struct.Path.html)
//! ones, but here they work directly upon
//! [`&str`](https://doc.rust-lang.org/std/primitive.str.html) instead of
//! [`&OsStr`](https://doc.rust-lang.org/std/ffi/struct.OsStr.html).

use crate::co;
use crate::kernel::decl::{GetFileAttributes, HINSTANCE, SysResult};
use crate::prelude::{Handle, kernel_Hinstance, NativeBitflag};

/// Returns the path of the current EXE file, without the EXE filename, and
/// without a trailing backslash.
///
/// In a debug build, the `target\debug` folders will be suppressed.
#[cfg(debug_assertions)]
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn exe_path() -> SysResult<String> {
	let dbg = HINSTANCE::NULL.GetModuleFileName()?;
	Ok(
		get_path( // target
			get_path( // debug
				get_path(&dbg).unwrap(), // exe name
			).unwrap(),
		).unwrap()
			.to_owned(),
	)
}

/// Returns the path of the current EXE file, without the EXE filename, and
/// without a trailing backslash.
///
/// In a debug build, the `target\debug` folders will be suppressed.
#[cfg(not(debug_assertions))]
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn exe_path() -> SysResult<String> {
	Ok(
		get_path(&HINSTANCE::NULL.GetModuleFileName()?)
			.unwrap().to_owned(),
	)
}

/// Returns true if the path exists.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn exists(full_path: &str) -> bool {
	GetFileAttributes(full_path).is_ok()
}

/// Extracts the file name from a full path, if any.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::path;
///
/// let f = path::get_file_name("C:\\Temp\\foo.txt"); // foo.txt
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn get_file_name(full_path: &str) -> Option<&str> {
	match full_path.rfind('\\') {
		None => Some(full_path), // if no backslash, the whole string is the file name
		Some(idx) => if idx == full_path.chars().count() - 1 {
			None // last char is '\\', no file name
		} else {
			Some(&full_path[idx + 1..])
		},
	}
}

/// Extracts the full path, but the last part.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::path;
///
/// let p = path::get_path("C:\\Temp\\xx\\a.txt"); // C:\Temp\xx
/// let q = path::get_path("C:\\Temp\\xx\\");      // C:\Temp\xx
/// let r = path::get_path("C:\\Temp\\xx");        // C:\Temp"
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn get_path(full_path: &str) -> Option<&str> {
	full_path.rfind('\\') // if no backslash, the whole string is the file name, so no path
		.map(|idx| &full_path[0..idx])
}

/// Tells whether the full path ends in one of the given extensions,
/// case-insensitive.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::path;
///
/// println!("{}",
///     path::has_extension("file.txt", &[".txt", ".bat"]));
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn has_extension(full_path: &str, extensions: &[impl AsRef<str>]) -> bool {
	let full_path_u = full_path.to_uppercase();
	extensions.iter()
		.find(|ext| {
			let ext_u = ext.as_ref().to_uppercase();
			full_path_u.ends_with(&ext_u)
		})
		.is_some()
}

/// Returns true if the path is a directory.
///
/// # Panics
///
/// Panics if the path does not exist.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn is_directory(full_path: &str) -> bool {
	let flags = GetFileAttributes(full_path).unwrap();
	flags.has(co::FILE_ATTRIBUTE::DIRECTORY)
}

/// Returns true if the path is hidden.
///
/// # Panics
///
/// Panics if the path does not exist.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn is_hidden(full_path: &str) -> bool {
	let flags = GetFileAttributes(full_path).unwrap();
	flags.has(co::FILE_ATTRIBUTE::HIDDEN)
}

/// Returns an iterator over each part of the path.
///
/// To iterate over files within a directory, see
/// [`HFINDFILE::iter`](crate::prelude::kernel_Hfindfile::iter).
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn iter_parts(full_path: &str) -> impl Iterator<Item = &str> {
	PartsIterator { path: full_path }
}

/// Replaces the extension by the given one.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::path;
///
/// let p = path::replace_extension(
///     "C:\\Temp\\something.txt", ".sh"); // C:\Temp\something.sh
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn replace_extension(full_path: &str, new_extension: &str) -> String {
	if let Some(last) = full_path.chars().last() {
		if last == '\\' { // full_path is a directory, do nothing
			return rtrim_backslash(full_path).to_owned();
		}
	}

	let new_has_dot = new_extension.chars().next() == Some('.');
	match full_path.rfind('.') {
		None => format!("{}{}{}", // file name without extension, just append it
			full_path,
			if new_has_dot { "" } else { "." },
			new_extension,
		),
		Some(idx) => format!("{}{}{}",
			&full_path[0..idx],
			if new_has_dot { "" } else { "." },
			new_extension,
		),
	}
}

/// Replaces the file name by the given one.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn replace_file_name(full_path: &str, new_file: &str) -> String {
	match get_path(full_path) {
		None => new_file.to_owned(),
		Some(path) => format!("{}\\{}", path, new_file),
	}
}

/// Keeps the file name and replaces the path by the given one.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::path;
///
/// let p = path::replace_path( // C:\another\foo.txt
///     "C:\\Temp\\foo.txt",
///     "C:\\another",
/// );
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn replace_path(full_path: &str, new_path: &str) -> String {
	let file_name = get_file_name(full_path);
	format!("{}{}{}",
		rtrim_backslash(new_path),
		if file_name.is_some() { "\\" } else { "" },
		file_name.unwrap_or(""))
}

/// Removes a trailing backslash, if any.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::path;
///
/// let p = path::rtrim_backslash("C:\\Temp\\"); // C:\Temp
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[must_use]
pub fn rtrim_backslash(full_path: &str) -> &str {
	match full_path.chars().last() {
		None => full_path, // empty string
		Some(last_ch) => if last_ch == '\\' {
			let mut chars = full_path.chars();
			chars.next_back(); // remove last char
			chars.as_str()
		} else {
			full_path // no trailing backslash
		},
	}
}

//------------------------------------------------------------------------------

struct PartsIterator<'a> {
	path: &'a str,
}

impl<'a> Iterator for PartsIterator<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		if self.path.is_empty() {
			return None;
		}

		Some(match self.path.find('\\') {
			Some(idx) => {
				let cur_part = &self.path[..idx];
				self.path = &self.path[idx + 1..];
				cur_part
			},
			None => {
				let cur_part = self.path; // until the end
				self.path = &self.path[self.path.len()..]; // empty
				cur_part
			},
		})
	}
}
