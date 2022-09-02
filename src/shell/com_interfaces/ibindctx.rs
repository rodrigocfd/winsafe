#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::WString;
use crate::kernel::ffi_types::{HRES, PCSTR, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IBindCtx`](crate::IBindCtx) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct IBindCtxVT {
	pub IUnknownVT: IUnknownVT,
	pub RegisterObjectBound: fn(ComPtr, ComPtr) -> HRES,
	pub RevokeObjectBound: fn(ComPtr, ComPtr) -> HRES,
	pub ReleaseBoundObjects: fn(ComPtr) -> HRES,
	pub SetBindOptions: fn(ComPtr, PVOID) -> HRES,
	pub GetBindOptions: fn(ComPtr, PVOID) -> HRES,
	pub GetRunningObjectTable: fn(ComPtr, *mut ComPtr) -> HRES,
	pub RegisterObjectParam: fn(ComPtr, PCSTR, ComPtr) -> HRES,
	pub GetObjectParam: fn(ComPtr, PCSTR, *mut ComPtr) -> HRES,
	pub EnumObjectParam: fn(ComPtr, *mut ComPtr) -> HRES,
	pub RevokeObjectParam: fn(ComPtr, PCSTR) -> HRES,
}

com_interface! { IBindCtx: "shell";
	"0000000e-0000-0000-c000-000000000046";
	/// [`IBindCtx`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ibindctx)
	/// COM interface over [`IBindCtxVT`](crate::vt::IBindCtxVT).
	///
	/// Automatically calls
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_IBindCtx for IBindCtx {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IBindCtx`](crate::IBindCtx).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_IBindCtx: ole_IUnknown {
	/// [`IBindCtx::ReleaseBoundObjects`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ibindctx-releaseboundobjects)
	/// method.
	fn ReleaseBoundObjects(&self) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				let vt = self.vt_ref::<IBindCtxVT>();
				(vt.ReleaseBoundObjects)(self.ptr())
			}
		)
	}

	/// [`IBindCtx::RevokeObjectParam`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ibindctx-revokeobjectparam)
	/// method.
	fn RevokeObjectParam(&self, key: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				let vt = self.vt_ref::<IBindCtxVT>();
				(vt.RevokeObjectParam)(
					self.ptr(),
					WString::from_str(key).as_ptr(),
				)
			}
		)
	}
}
