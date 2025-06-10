#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::shell::{ffi, iterators::*};

handle! { HDROP;
	/// Handle to an
	/// [internal drop structure](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdrop).
}

impl HDROP {
	/// [`DragQueryFile`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew)
	/// function.
	///
	/// Returns an iterator over the dropped files.
	///
	/// # Examples
	///
	/// Iterating over the strings:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let mut hdrop: w::HDROP; // initialized somewhere
	/// # let mut hdrop = w::HDROP::NULL;
	///
	/// for file_path in hdrop.DragQueryFile()? {
	///     let file_path = file_path?;
	///     println!("File: {}", file_path);
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	///
	/// Collecting the strings into a [`Vec`](std::vec::Vec):
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let mut hdrop: w::HDROP; // initialized somewhere
	/// # let mut hdrop = w::HDROP::NULL;
	///
	/// let file_paths = hdrop.DragQueryFile()?
	///     .collect::<w::SysResult<Vec<_>>>()?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn DragQueryFile(&self) -> SysResult<impl Iterator<Item = SysResult<String>> + '_> {
		Ok(HdropIter::new(self)?)
	}

	/// [`DragQueryPoint`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragquerypoint)
	/// function.
	///
	/// Returns the coordinates and whether the drop occurred in the client area
	/// of the window.
	///
	/// Note that you must call this method before
	/// [`DragQueryFile`](crate::HDROP::DragQueryFile), because it invalidates
	/// the `HDROP` handle after the iterator is consumed.
	#[must_use]
	pub fn DragQueryPoint(&self) -> (POINT, bool) {
		let mut pt = POINT::default();
		let client_area = unsafe { ffi::DragQueryPoint(self.ptr(), pvoid(&mut pt)) };
		(pt, client_area != 0)
	}
}
