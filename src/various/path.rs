use crate::aliases::WinResult;
use crate::handles::HINSTANCE;

/// File path utilities.
///
/// Some of the functions are analog to
/// [`std::path::Path`](https://doc.rust-lang.org/std/path/struct.Path.html)
/// ones, but here they work upon `&str` instead of `&OsStr`.
pub struct Path {}

impl Path {
	/// Returns the path of the EXE file, without the EXE filename, and without
	/// a trailing backslash.
	///
	/// In a debug build, the `target\debug` folders will not show up.
	#[cfg(debug_assertions)]
	pub fn exe_path() -> WinResult<String> {
		let dbg = HINSTANCE::NULL.GetModuleFileName()?;
		Ok(
			format!("{}\\{}",
				Self::path_from( // target
					Self::path_from( // debug
						Self::path_from(&dbg).unwrap(), // exe name
					).unwrap(),
				).unwrap(),
				Self::file_from(&dbg).unwrap()),
		)
	}

	/// Returns the path of the EXE file, without the EXE filename, and without
	/// a trailing backslash.
	///
	/// In a debug build, the `target\debug` folders will not show up.
	#[cfg(not(debug_assertions))]
	pub fn exe_path() -> WinResult<String> {
		Ok(
			Self::path_from(&HINSTANCE::NULL.GetModuleFileName()?)
				.unwrap().to_owned(),
		)
	}

	/// Returns an iterator over each part of the path.
	pub fn iter(full_path: &str) -> impl Iterator<Item = &str> {
		PathIterator { path: full_path }
	}

	/// Extracts the file name from a full path.
	pub fn file_from(full_path: &str) -> Option<&str> {
		full_path.rfind('\\')
			.map(|idx| &full_path[idx + 1..])
	}

	/// Tells whether the full path ends in one of the given extensions,
	/// case-insensitive.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::Path;
	///
	/// println!("{}",
	///     Path::has_extension("file.txt", &[".txt", ".bat"]));
	/// ```
	pub fn has_extension<S: AsRef<str>>(full_path: &str, extensions: &[S]) -> bool {
		extensions.iter()
			.find(|ext| full_path.ends_with(ext.as_ref()))
			.is_some()
	}

	/// Removes the file name from a full path. Returned string won't have a
	/// trailing backslash.
	pub fn path_from(full_path: &str) -> Option<&str> {
		full_path.rfind('\\')
			.map(|idx| &full_path[0..idx])
	}

	/// Replaces the extension by the given one.
	pub fn replace_extension(full_path: &str, new_extension_with_dot: &str) -> String {
		full_path.rfind('.')
			.map_or_else(
				|| full_path.to_owned(),
				|idx| format!("{}{}", &full_path[0..idx], new_extension_with_dot),
			)
	}

	/// Replaces the file name by the given one.
	pub fn replace_file(full_path: &str, new_file: &str) -> String {
		Self::path_from(full_path)
			.map_or_else(
				|| new_file.to_owned(),
				|path| format!("{}\\{}", path, new_file),
			)
	}

	/// Keeps the file name and replaces the path by the given one.
	pub fn replace_path(full_path: &str, new_path_without_slash: &str) -> String {
		Self::file_from(full_path)
			.map_or_else(
				|| full_path.to_owned(),
				|file| format!("{}\\{}", new_path_without_slash, file),
			)
	}
}

//------------------------------------------------------------------------------

struct PathIterator<'a> {
	path: &'a str,
}

impl<'a> Iterator for PathIterator<'a> {
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
