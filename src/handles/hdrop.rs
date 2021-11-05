#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::shell32;
use crate::funcs::GetLastError;
use crate::structs::POINT;
use crate::various::WString;

/// Handle to an
/// [internal drop structure](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdrop).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HDROP(pub(crate) *mut std::ffi::c_void);

impl_handle!(HDROP);

impl HDROP {
	/// [`DragFinish`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragfinish)
	/// method.
	///
	/// Prefer using [`HDROP::iter`](crate::HDROP::iter), which calls
	/// `DragFinish` automatically.
	pub fn DragFinish(self) {
		unsafe { shell32::DragFinish(self.0) }
	}

	/// [`DragQueryFile`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew)
	/// method.
	///
	/// This method is rather tricky, consider using
	/// [`HDROP::iter`](crate::HDROP::iter).
	pub fn DragQueryFile(self,
		ifile: Option<u32>, buf: Option<&mut WString>) -> WinResult<u32>
	{
		let cch = buf.as_ref().map_or(0, |buf| buf.buffer_size());

		match unsafe {
			shell32::DragQueryFileW(
				self.0,
				ifile.unwrap_or(0xffff_ffff),
				buf.map_or(std::ptr::null_mut(), |buf| buf.as_mut_ptr()),
				cch as _,
			)
		} {
			0 => Err(GetLastError()),
			char_count => Ok(char_count),
		}
	}

	/// [`DragQueryPoint`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragquerypoint)
	/// method.
	///
	/// Returns the coordinates and whether the drop occurred in the client area
	/// of the window.
	pub fn DragQueryPoint(self) -> (POINT, bool) {
		let mut pt = POINT::default();
		let client_area = unsafe {
			shell32::DragQueryPoint(self.0, &mut pt as *mut _ as _)
		};
		(pt, client_area != 0)
	}

	/// Returns an iterator over the dropped files by calling
	/// [`HDROP::DragQueryFile`](crate::HDROP::DragQueryFile) consecutively,
	/// then frees the handle by calling
	/// [`HDROP::DragFinish`](crate::HDROP::DragFinish).
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HDROP;
	///
	/// let hdrop: HDROP; // initialized somewhere
	///
	/// for file_res in hdrop.iter() {
	///     let file_path = file_res?;
	///     println!("File: {}", file_path);
	/// }
	/// ```
	pub fn iter(self) -> impl Iterator<Item = WinResult<String>> {
		DropsIter::new(self)
	}
}

//------------------------------------------------------------------------------

struct DropsIter {
	hdrop: HDROP,
	first_pass: bool,
	cur_index: u32,
	count: u32,
	buf: WString,
}

impl Drop for DropsIter {
	fn drop(&mut self) {
		self.hdrop.DragFinish();
	}
}

impl Iterator for DropsIter {
	type Item = WinResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.first_pass {
			self.first_pass = false;
			self.count = match self.hdrop.DragQueryFile(None, None) {
				Err(e) => return Some(Err(e)), // cur_index == count (zero), so no further iterations
				Ok(count) => count,
			};
		}

		if self.cur_index == self.count { // no more files?
			return None;
		}

		let len = match self.hdrop.DragQueryFile(Some(self.cur_index), None) {
			Err(e) => {
				self.cur_index = self.count; // no further iterations will be done
				return Some(Err(e))
			},
			Ok(len) => len as usize, // number of chars to be retrieved, not including terminating null
		};

		self.buf.realloc_buffer(len + 1); // reuse buffer
		if let Err(e) = self.hdrop.DragQueryFile(Some(self.cur_index), Some(&mut self.buf)) {
			self.cur_index = self.count; // no further iterations will be done
			return Some(Err(e));
		}

		self.cur_index += 1;
		Some(Ok(self.buf.to_string()))
	}
}

impl DropsIter {
	fn new(hdrop: HDROP) -> Self {
		Self {
			hdrop,
			first_pass: true,
			cur_index: 0,
			count: 0,
			buf: WString::default(),
		}
	}
}
