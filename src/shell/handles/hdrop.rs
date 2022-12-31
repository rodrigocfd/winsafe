#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::{GetLastError, SysResult, WString};
use crate::kernel::privs::{MAX_PATH, replace_handle_value};
use crate::prelude::Handle;
use crate::shell;
use crate::user::decl::POINT;

impl_handle! { HDROP;
	/// Handle to an
	/// [internal drop structure](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdrop).
}

impl shell_Hdrop for HDROP {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`HDROP`](crate::HDROP).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_Hdrop: Handle {
	/// Returns an iterator over the dropped files by calling
	/// [`HDROP::DragQueryFile`](crate::prelude::shell_Hdrop::DragQueryFile)
	/// consecutively, then frees the handle by calling
	/// [`HDROP::DragFinish`](crate::prelude::shell_Hdrop::DragFinish).
	///
	/// # Examples
	///
	/// Iterating over the strings:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::HDROP;
	///
	/// let hdrop: HDROP; // initialized somewhere
	/// # let hdrop = HDROP::NULL;
	///
	/// for file_path in hdrop.iter()? {
	///     let file_path = file_path?;
	///     println!("File: {}", file_path);
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	///
	/// Collecting the strings into a
	/// [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html):
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HDROP, SysResult};
	///
	/// let hdrop: HDROP; // initialized somewhere
	/// # let hdrop = HDROP::NULL;
	///
	/// let file_paths = hdrop.iter()?
	///     .collect::<SysResult<Vec<_>>>()?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn iter(&self) -> SysResult<Box<dyn Iterator<Item = SysResult<String>> + '_>> {
		Ok(Box::new(DropsIter::new(self)?))
	}

	/// [`DragFinish`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragfinish)
	/// method.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using [`HDROP::iter`](crate::prelude::shell_Hdrop::iter), which
	/// calls `DragFinish` automatically.
	fn DragFinish(&self) {
		unsafe {
			shell::ffi::DragFinish(self.as_ptr());
			replace_handle_value(self, Self::INVALID);
		}
	}

	/// [`DragQueryFile`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew)
	/// method.
	///
	/// # Safety
	///
	/// The `buf` must be allocated with the correct length.
	///
	/// This method is rather tricky, consider using
	/// [`HDROP::iter`](crate::prelude::shell_Hdrop::iter).
	unsafe fn DragQueryFile(&self,
		ifile: Option<u32>, buf: Option<&mut WString>) -> SysResult<u32>
	{
		let cch = buf.as_ref().map_or(0, |buf| buf.buf_len());

		match unsafe {
			shell::ffi::DragQueryFileW(
				self.as_ptr(),
				ifile.unwrap_or(0xffff_ffff),
				buf.map_or(std::ptr::null_mut(), |buf| buf.as_mut_ptr()),
				cch as _,
			)
		} {
			0 => Err(GetLastError()),
			char_count => Ok(char_count),
		}
	}

	/// [`DragQueryPoint`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragquerypoint)
	/// method.
	///
	/// Returns the coordinates and whether the drop occurred in the client area
	/// of the window.
	#[must_use]
	fn DragQueryPoint(&self) -> (POINT, bool) {
		let mut pt = POINT::default();
		let client_area = unsafe {
			shell::ffi::DragQueryPoint(self.as_ptr(), &mut pt as *mut _ as _)
		};
		(pt, client_area != 0)
	}
}

//------------------------------------------------------------------------------

struct DropsIter<'a, H>
	where H: shell_Hdrop,
{
	hdrop: &'a H,
	buffer: WString,
	count: u32,
	current: u32,
}

impl<'a, H> Drop for DropsIter<'a, H>
	where H: shell_Hdrop,
{
	fn drop(&mut self) {
		self.hdrop.DragFinish();
	}
}

impl<'a, H> Iterator for DropsIter<'a, H>
	where H: shell_Hdrop,
{
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match unsafe {
			self.hdrop.DragQueryFile(Some(self.current), Some(&mut self.buffer))
		} {
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

impl<'a, H> DropsIter<'a, H>
	where H: shell_Hdrop,
{
	fn new(hdrop: &'a H) -> SysResult<Self> {
		Ok(Self {
			hdrop,
			buffer: WString::new_alloc_buf(MAX_PATH + 1), // so we alloc just once
			count: unsafe { hdrop.DragQueryFile(None, None)? },
			current: 0,
		})
	}
}
