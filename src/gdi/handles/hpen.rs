#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

handle! { HPEN;
	/// Handle to a
	/// [pen](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpen)
	/// GDI object.
}

impl GdiObject for HPEN {}

impl HPEN {
	/// [`CreatePen`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpen)
	/// function.
	#[must_use]
	pub fn CreatePen(
		style: co::PS,
		width: i32,
		color: COLORREF,
	) -> SysResult<DeleteObjectGuard<HPEN>> {
		unsafe {
			PtrRet(ffi::CreatePen(style.raw(), width, color.into()))
				.to_invalidparm_handle()
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreatePenIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpenindirect)
	/// function.
	#[must_use]
	pub fn CreatePenIndirect(lp: &mut LOGPEN) -> SysResult<DeleteObjectGuard<HPEN>> {
		unsafe {
			PtrRet(ffi::CreatePenIndirect(pcvoid(lp)))
				.to_invalidparm_handle()
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`GetStockObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// function.
	#[must_use]
	pub fn GetStockObject(sp: co::STOCK_PEN) -> SysResult<HPEN> {
		PtrRet(unsafe { ffi::GetStockObject(sp.raw()) }).to_invalidparm_handle()
	}
}
