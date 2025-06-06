//! File path utilities.
//!
//! Some of the functions are similar to [`std::path::Path`] ones, but here they
//! work directly upon [`&str`](str) instead of [`&OsStr`](std::ffi::OsStr).

use crate::co;
use crate::decl::*;
use crate::kernel::iterators::*;
use crate::prelude::*;

/// Returns an iterator over the files and folders within a directory.
/// Optionally, a wildcard can be specified to filter files by name.
///
/// This is a high-level abstraction over [`HFINDFILE`](crate::HFINDFILE)
/// iteration functions.
///
/// # Examples
///
/// Listing all text files in a directory:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// for file_path in w::path::dir_list("C:\\temp", Some("*.txt")) {
///     let file_path = file_path?;
///     println!("{}", file_path);
/// }
/// # w::SysResult::Ok(())
/// ```
#[must_use]
pub fn dir_list<'a>(
	dir_path: &'a str,
	filter: Option<&'a str>,
) -> impl Iterator<Item = SysResult<String>> + 'a {
	DirListIter::new(dir_path.to_owned(), filter)
}

/// Returns an interator over the files within a directory, and all its
/// subdirectories, recursively.
///
/// This is a high-level abstraction over [`HFINDFILE`](crate::HFINDFILE)
/// iteration functions.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// // Ordinary for loop
/// for file_path in w::path::dir_walk("C:\\Temp") {
///     let file_path = file_path?;
///     println!("{}", file_path);
/// }
///
/// // Closure with try_for_each
/// w::path::dir_walk("C:\\Temp")
///     .try_for_each(|file_path| {
///         let file_path = file_path?;
///         println!("{}", file_path);
///         Ok(())
///     })?;
///
/// // Collecting into a Vec
/// let all = w::path::dir_walk("C:\\Temp")
///     .collect::<w::SysResult<Vec<_>>>()?;
///
/// // Transforming and collecting into a Vec
/// let all = w::path::dir_walk("C:\\Temp")
///     .map(|file_path| {
///         let file_path = file_path?;
///         Ok(format!("PATH: {}", file_path))
///     })
///     .collect::<w::SysResult<Vec<_>>>()?;
/// # w::SysResult::Ok(())
/// ```
#[must_use]
pub fn dir_walk<'a>(dir_path: &'a str) -> impl Iterator<Item = SysResult<String>> + 'a {
	DirWalkIter::new(dir_path.to_owned())
}

/// Returns a new string with the path of the current EXE file, without the EXE
/// filename, and without a trailing backslash.
///
/// In a debug build, the `target\debug` folders will be suppressed.
#[cfg(debug_assertions)]
#[must_use]
pub fn exe_path() -> SysResult<String> {
	let dbg = HINSTANCE::NULL.GetModuleFileName()?;
	Ok(get_path(
		get_path(
			get_path(&dbg).unwrap(), // exe name; go up target and debug folders
		)
		.unwrap(),
	)
	.unwrap()
	.to_owned())
}

/// Returns a new string with the path of the current EXE file, without the EXE
/// filename, and without a trailing backslash.
///
/// In a debug build, the `target\debug` folders will be suppressed.
#[cfg(not(debug_assertions))]
#[must_use]
pub fn exe_path() -> SysResult<String> {
	Ok(get_path(&HINSTANCE::NULL.GetModuleFileName()?)
		.unwrap()
		.to_owned())
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
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let f = w::path::get_file_name("C:\\Temp\\foo.txt"); // foo.txt
/// ```
#[must_use]
pub fn get_file_name(full_path: &str) -> Option<&str> {
	match full_path.rfind('\\') {
		None => Some(full_path), // if no backslash, the whole string is the file name
		Some(idx) => {
			if idx == full_path.chars().count() - 1 {
				None // last char is '\\', no file name
			} else {
				Some(&full_path[idx + 1..])
			}
		},
	}
}

/// Extracts the full path, but the last part.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let p = w::path::get_path("C:\\Temp\\xx\\a.txt"); // C:\Temp\xx
/// let q = w::path::get_path("C:\\Temp\\xx\\");      // C:\Temp\xx
/// let r = w::path::get_path("C:\\Temp\\xx");        // C:\Temp"
/// ```
#[must_use]
pub fn get_path(full_path: &str) -> Option<&str> {
	full_path
		.rfind('\\') // if no backslash, the whole string is the file name, so no path
		.map(|idx| &full_path[0..idx])
}

/// Tells whether the full path ends in one of the given extensions,
/// case-insensitive.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// println!("{}",
///     w::path::has_extension("file.txt", &[".txt", ".bat"]));
/// ```
#[must_use]
pub fn has_extension(full_path: &str, extensions: &[impl AsRef<str>]) -> bool {
	let full_path_u = full_path.to_uppercase();
	extensions
		.iter()
		.find(|ext| {
			let ext_u = ext.as_ref().to_uppercase();
			full_path_u.ends_with(&ext_u)
		})
		.is_some()
}

/// Returns true if the path is a directory. Calls
/// [`GetFileAttributes`](crate::GetFileAttributes).
///
/// # Panics
///
/// Panics if the path does not exist.
#[must_use]
pub fn is_directory(full_path: &str) -> bool {
	let flags = GetFileAttributes(full_path).unwrap();
	flags.has(co::FILE_ATTRIBUTE::DIRECTORY)
}

/// Returns true if the path is hidden. Calls
/// [`GetFileAttributes`](crate::GetFileAttributes).
///
/// # Panics
///
/// Panics if the path does not exist.
#[must_use]
pub fn is_hidden(full_path: &str) -> bool {
	let flags = GetFileAttributes(full_path).unwrap();
	flags.has(co::FILE_ATTRIBUTE::HIDDEN)
}

/// Replaces the file extension by the given one, returning a new string.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let p = w::path::replace_extension(
///     "C:\\Temp\\something.txt", ".sh"); // C:\Temp\something.sh
/// ```
#[must_use]
pub fn replace_extension(full_path: &str, new_extension: &str) -> String {
	if let Some(last) = full_path.chars().last() {
		if last == '\\' {
			return rtrim_backslash(full_path).to_owned(); // full_path is a directory, do nothing
		}
	}

	let new_has_dot = new_extension.chars().next() == Some('.');
	match full_path.rfind('.') {
		None => format!(
			"{}{}{}", // file name without extension, just append it
			full_path,
			if new_has_dot { "" } else { "." },
			new_extension,
		),
		Some(idx) => {
			format!("{}{}{}", &full_path[0..idx], if new_has_dot { "" } else { "." }, new_extension,)
		},
	}
}

/// Replaces the file name by the given one, returning a new string.
#[must_use]
pub fn replace_file_name(full_path: &str, new_file: &str) -> String {
	match get_path(full_path) {
		None => new_file.to_owned(),
		Some(path) => format!("{}\\{}", path, new_file),
	}
}

/// Keeps the file name and replaces the path by the given one, returning a new
/// string.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let p = w::path::replace_path( // C:\another\foo.txt
///     "C:\\Temp\\foo.txt",
///     "C:\\another",
/// );
/// ```
#[must_use]
pub fn replace_path(full_path: &str, new_path: &str) -> String {
	let file_name = get_file_name(full_path);
	format!(
		"{}{}{}",
		rtrim_backslash(new_path),
		if file_name.is_some() { "\\" } else { "" },
		file_name.unwrap_or("")
	)
}

/// Removes a trailing backslash, if any.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let p = w::path::rtrim_backslash("C:\\Temp\\"); // C:\Temp
/// ```
#[must_use]
pub fn rtrim_backslash(full_path: &str) -> &str {
	match full_path.chars().last() {
		None => full_path, // empty string
		Some(last_ch) => {
			if last_ch == '\\' {
				let mut chars = full_path.chars();
				chars.next_back(); // remove last char
				chars.as_str()
			} else {
				full_path // no trailing backslash
			}
		},
	}
}

/// Returns a `Vec` with each part of the full path.
#[must_use]
pub fn split_parts(full_path: &str) -> Vec<&str> {
	let no_bs = rtrim_backslash(full_path);
	no_bs.split('\\').collect()
}
