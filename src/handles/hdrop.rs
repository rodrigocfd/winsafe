#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::shell32;
use crate::funcs::GetLastError;
use crate::structs::POINT;
use crate::WString;

handle_type! {
	/// Handle to an
	/// [internal drop structure](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdrop).
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
	///
	/// ```rust.ignore
	/// let files = hdrop.DragQueryFile().unwrap();
	/// for f in files.iter() {
	///   println!("File path: {}", f);
	/// }
	/// ```
	pub fn DragQueryFile(self) -> WinResult<Vec<String>> {
		let count = unsafe {
			shell32::DragQueryFileW(self.ptr, 0xffff_ffff, std::ptr::null_mut(), 0)
		};
		if count == 0 {
			return Err(GetLastError());
		}

		let mut wbuf = WString::default();
		let mut files = Vec::default();

		for i in 0..count {
			let mut len = unsafe {
				shell32::DragQueryFileW(self.ptr, i, std::ptr::null_mut(), 0) + 1 // room for terminating null
			};
			if len == 0 {
				return Err(GetLastError());
			}

			wbuf.realloc_buffer(len as usize);
			len = unsafe {
				shell32::DragQueryFileW(self.ptr, i, wbuf.as_mut_ptr(), len)
			};
			if len == 0 {
				return Err(GetLastError());
			}

			files.push(wbuf.to_string());
		}

		unsafe { shell32::DragFinish(self.ptr); }
		files.sort_by(|a, b| a.to_uppercase().cmp(&b.to_uppercase())); // case insensitive
		Ok(files)
	}

	/// [`DragQueryPoint`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragquerypoint)
	/// method.
	pub fn DragQueryPoint(self) -> (POINT, bool) {
		let mut pt = POINT::default();
		let clientArea = unsafe {
			shell32::DragQueryPoint(self.ptr, &mut pt as *mut _ as *mut _)
		};
		(pt, clientArea != 0)
	}
}
