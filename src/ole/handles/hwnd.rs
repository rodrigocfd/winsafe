#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::ole::ffi;
use crate::ole::privs::*;
use crate::prelude::*;

impl HWND {
	/// [`RegisterDragDrop`](https://learn.microsoft.com/en-us/windows/win32/api/ole2/nf-ole2-registerdragdrop)
	/// function.
	///
	/// Note that if you don't call [`OleInitialize`](crate::OleInitialize)
	/// before this function, you'll receive an
	/// [`ERROR::OUTOFMEMORY`](crate::co::ERROR::OUTOFMEMORY) error.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::cell::Cell;
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let drop_target = w::IDropTarget::new_impl();
	/// let revoke = Cell::new(Option::<w::RevokeDragDropGuard>::None);
	///
	/// // later...
	/// revoke.set(Some(hwnd.RegisterDragDrop(&drop_target)?));
	///
	/// // revoke...
	/// revoke.set(None);
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	pub fn RegisterDragDrop(&self, drop_target: &IDropTarget) -> HrResult<RevokeDragDropGuard> {
		unsafe {
			HrRet(ffi::RegisterDragDrop(self.ptr(), drop_target.ptr() as _))
				.to_hrresult()
				.map(|_| RevokeDragDropGuard::new(self))
		}
	}
}
