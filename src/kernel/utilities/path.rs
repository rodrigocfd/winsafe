//! File path utilities.
//!
//! Some of the functions are similar to
//! [`std::path::Path`](https://doc.rust-lang.org/std/path/struct.Path.html)
//! ones, but here they work directly upon
//! [`&str`](https://doc.rust-lang.org/std/primitive.str.html) instead of
//! [`&OsStr`](https://doc.rust-lang.org/std/ffi/struct.OsStr.html).

use crate::co;
use crate::kernel::decl::{
	GetFileAttributes, HFINDFILE, HINSTANCE, SysResult, WIN32_FIND_DATA,
};
use crate::kernel::guard::HfindfileGuard;
use crate::prelude::{Handle, kernel_Hfindfile, kernel_Hinstance, NativeBitflag};

/// Returns an iterator over the files and folders within a directory.
/// Optionally, a wildcard can be specified to filter files by name.
///
/// # Examples
///
/// Listing all text files in a directory:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::path;
///
/// for file_path in path::dir_list("C:\\temp", Some("*.txt")) {
///     let file_path = file_path?;
///     println!("{}", file_path);
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn dir_list<'a>(
	dir_path: &'a str,
	filter: Option<&'a str>) -> impl Iterator<Item = SysResult<String>> + 'a
{
	DirListIter::new(dir_path, filter)
}

/// Returns the path of the current EXE file, without the EXE filename, and
/// without a trailing backslash.
///
/// In a debug build, the `target\debug` folders will be suppressed.
#[cfg(debug_assertions)]
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
#[must_use]
pub fn exe_path() -> SysResult<String> {
	Ok(
		get_path(&HINSTANCE::NULL.GetModuleFileName()?)
			.unwrap().to_owned(),
	)
}

/// Returns true if the path exists.
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
#[must_use]
pub fn is_hidden(full_path: &str) -> bool {
	let flags = GetFileAttributes(full_path).unwrap();
	flags.has(co::FILE_ATTRIBUTE::HIDDEN)
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

/// Returns a `Vec` with each part of the full path.
#[must_use]
pub fn split_parts(full_path: &str) -> Vec<&str> {
	let no_bs = rtrim_backslash(full_path);
	no_bs.split('\\').collect()
}

//------------------------------------------------------------------------------

struct DirListIter<'a> {
	dir_path: &'a str,
	filter: Option<&'a str>,
	hfind: Option<HfindfileGuard>,
	wfd: WIN32_FIND_DATA,
	no_more: bool,
}

impl<'a> Iterator for DirListIter<'a> {
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.no_more {
			return None;
		}

		let found = match &self.hfind {
			None => { // first pass
				let dir_final = match self.filter {
					None => format!("{}\\*", self.dir_path),
					Some(filter) => format!("{}\\{}", self.dir_path, filter),
				};

				let found = match HFINDFILE::FindFirstFile(&dir_final, &mut self.wfd) {
					Err(e) => {
						self.no_more = true; // prevent further iterations
						return Some(Err(e));
					},
					Ok((hfind, found)) => {
						self.hfind = Some(hfind); // store our find handle
						found
					},
				};
				found
			},
			Some(hfind) => { // subsequent passes
				match hfind.FindNextFile(&mut self.wfd) {
					Err(e) => {
						self.no_more = true; // prevent further iterations
						return Some(Err(e));
					},
					Ok(found) => found,
				}
			},
		};

		if found {
			Some(Ok(format!("{}\\{}", self.dir_path, self.wfd.cFileName())))
		} else {
			None
		}
	}
}

impl<'a> DirListIter<'a> {
	fn new(dir_path: &'a str, filter: Option<&'a str>) -> Self {
		Self {
			dir_path: rtrim_backslash(dir_path),
			filter,
			hfind: None,
			wfd: WIN32_FIND_DATA::default(),
			no_more: false,
		}
	}
}
