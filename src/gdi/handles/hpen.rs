#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, gdi};
use crate::gdi::decl::LOGPEN;
use crate::gdi::guard::DeleteObjectGuard;
use crate::kernel::decl::SysResult;
use crate::kernel::privs::ptr_to_sysresult;
use crate::prelude::GdiObject;
use crate::user::decl::COLORREF;

impl_handle! { HPEN;
	/// Handle to a
	/// [pen](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpen)
	/// GDI object.
}

impl GdiObject for HPEN {}
impl gdi_Hpen for HPEN {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HPEN`](crate::HPEN).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hpen: GdiObject {
	/// [`CreatePen`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpen)
	/// static method.
	#[must_use]
	fn CreatePen(
		style: co::PS,
		width: i32,
		color: COLORREF,
	) -> SysResult<DeleteObjectGuard<HPEN>>
	{
		ptr_to_sysresult(
			unsafe { gdi::ffi::CreatePen(style.0, width, color.0) },
			|ptr| DeleteObjectGuard::new(HPEN(ptr)),
		)
	}

	/// [`CreatePenIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpenindirect)
	/// static method.
	#[must_use]
	fn CreatePenIndirect(lp: &mut LOGPEN) -> SysResult<DeleteObjectGuard<HPEN>> {
		ptr_to_sysresult(
			unsafe { gdi::ffi::CreatePenIndirect(lp as *const _ as _) },
			|ptr| DeleteObjectGuard::new(HPEN(ptr)),
		)
	}

	/// [`GetStockObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// static method.
	#[must_use]
	fn GetStockObject(sp: co::STOCK_PEN) -> SysResult<HPEN> {
		ptr_to_sysresult(
			unsafe { gdi::ffi::GetStockObject(sp.0) },
			|ptr| HPEN(ptr),
		)
	}
}
