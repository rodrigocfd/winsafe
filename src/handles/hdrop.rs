#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::aliases::WinResult;
use crate::ffi::shell32;
use crate::funcs::GetLastError;
use crate::privs::MAX_PATH;
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
	/// for file_path in hdrop.iter()? {
	///     let file_path = file_path?;
	///     println!("File: {}", file_path);
	/// }
	/// ```
	pub fn iter<'a>(&'a self) -> WinResult<impl Iterator<Item = WinResult<String>> + 'a> {
		DropsIter::new(*self)
	}
}

//------------------------------------------------------------------------------

struct DropsIter<'a> {
	hdrop: HDROP,
	buffer: WString,
	count: u32,
	current: u32,
	owner_: PhantomData<&'a ()>,
}

impl<'a> Drop for DropsIter<'a> {
	fn drop(&mut self) {
		self.hdrop.DragFinish();
	}
}

impl<'a> Iterator for DropsIter<'a> {
	type Item = WinResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match self.hdrop
			.DragQueryFile(Some(self.current), Some(&mut self.buffer))
		{
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
			Ok(_) => {
				self.current += 1;
				Some(Ok(self.buffer.to_string()))
			},
		}
	}
}

impl<'a> DropsIter<'a> {
	fn new(hdrop: HDROP) -> WinResult<Self> {
		Ok(Self {
			hdrop,
			buffer: WString::new_alloc_buffer(MAX_PATH + 1), // so we alloc just once
			count: hdrop.DragQueryFile(None, None)?,
			current: 0,
			owner_: PhantomData,
		})
	}
}
