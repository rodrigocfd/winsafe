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
	pub fn RegisterDragDrop(&self, drop_target: &IDropTarget) -> HrResult<()> {
		HrRet(unsafe { ffi::RegisterDragDrop(self.ptr(), drop_target.ptr() as _) }).to_hrresult()
	}

	/// [`RevokeDragDrop`](https://learn.microsoft.com/en-us/windows/win32/api/ole2/nf-ole2-revokedragdrop)
	/// function.
	pub fn RevokeDragDrop(&self) -> HrResult<()> {
		HrRet(unsafe { ffi::RevokeDragDrop(self.ptr()) }).to_hrresult()
	}
}
