#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::WString;
use crate::kernel::ffi_types::{COMPTR, HRES, PCSTR, PVOID};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IBindCtx`](crate::IBindCtx) virtual table.
#[repr(C)]
pub struct IBindCtxVT {
	pub IUnknownVT: IUnknownVT,
	pub RegisterObjectBound: fn(COMPTR, COMPTR) -> HRES,
	pub RevokeObjectBound: fn(COMPTR, COMPTR) -> HRES,
	pub ReleaseBoundObjects: fn(COMPTR) -> HRES,
	pub SetBindOptions: fn(COMPTR, PVOID) -> HRES,
	pub GetBindOptions: fn(COMPTR, PVOID) -> HRES,
	pub GetRunningObjectTable: fn(COMPTR, *mut COMPTR) -> HRES,
	pub RegisterObjectParam: fn(COMPTR, PCSTR, COMPTR) -> HRES,
	pub GetObjectParam: fn(COMPTR, PCSTR, *mut COMPTR) -> HRES,
	pub EnumObjectParam: fn(COMPTR, *mut COMPTR) -> HRES,
	pub RevokeObjectParam: fn(COMPTR, PCSTR) -> HRES,
}

com_interface! { IBindCtx: "0000000e-0000-0000-c000-000000000046";
	/// [`IBindCtx`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ibindctx)
	/// COM interface over [`IBindCtxVT`](crate::vt::IBindCtxVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IBindCtx for IBindCtx {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IBindCtx`](crate::IBindCtx).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IBindCtx: ole_IUnknown {
	/// [`IBindCtx::ReleaseBoundObjects`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ibindctx-releaseboundobjects)
	/// method.
	fn ReleaseBoundObjects(&self) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { (vt::<IBindCtxVT>(self).ReleaseBoundObjects)(self.ptr()) },
		)
	}

	/// [`IBindCtx::RevokeObjectParam`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ibindctx-revokeobjectparam)
	/// method.
	fn RevokeObjectParam(&self, key: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IBindCtxVT>(self).RevokeObjectParam)(
					self.ptr(),
					WString::from_str(key).as_ptr(),
				)
			},
		)
	}
}
