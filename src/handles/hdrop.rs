#![allow(non_snake_case)]

use crate::ffi::{HANDLE, shell32};
use crate::internal_defs::mut_void;
use crate::structs::POINT;
use crate::Utf16;

handle_type! {
	/// Handle to an
	/// [internal drop structure](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdrop).
	/// Exposes methods.
	HDROP
}

impl HDROP {
	/// This method calls
	/// [`DragQueryFile`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew)
	/// repeatedly to retrieve all files, then calls
	/// [`DragFinish`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragfinish).
	///
	/// # Examples
	///
	/// Retrieve all files at once:
	/// ```rust.ignore
	/// let files = hdrop.DragQueryFile().unwrap();
	/// for f in files.iter() {
	///   println!("File path: {}", f);
	/// }
	/// ```
	pub fn DragQueryFile(&self) -> Result<Vec<String>, ()> {
		let count = unsafe {
			shell32::DragQueryFileW(self.0, 0xffff_ffff, std::ptr::null_mut(), 0)
		};
		if count == 0 {
			return Err(());
		}

		let mut buf = Utf16::default();
		let mut files = Vec::default();

		for i in 0..count {
			let mut len = unsafe {
				shell32::DragQueryFileW(self.0, i, std::ptr::null_mut(), 0) + 1 // room for terminating null
			};
			if len == 0 {
				return Err(());
			}

			buf.realloc_buffer(len as usize);
			len = unsafe {
				shell32::DragQueryFileW(self.0, i, buf.as_mut_ptr(), len)
			};
			if len == 0 {
				return Err(());
			}

			files.push(buf.to_string());
		}

		unsafe { shell32::DragFinish(self.0); }
		files.sort_by(|a, b| a.to_uppercase().cmp(&b.to_uppercase())); // case insensitive
		Ok(files)
	}

	/// [`DragQueryPoint`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragquerypoint)
	/// method.
	pub fn DragQueryPoint(&self) -> (POINT, bool) {
		let mut pt = POINT::default();
		let clientArea = unsafe {
			shell32::DragQueryPoint(self.0, mut_void(&mut pt))
		};
		(pt, clientArea != 0)
	}
}