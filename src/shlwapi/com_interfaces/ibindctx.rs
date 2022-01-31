#![allow(non_snake_case)]

use crate::ffi_types::{HRES, PCSTR, PVOID};
use crate::kernel::decl::WString;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::OleIUnknown;
use crate::vt::IUnknownVT;

/// [`IBindCtx`](crate::IBindCtx) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shlwapi")))]
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

/// [`IBindCtx`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ibindctx)
/// COM interface over [`IBindCtxVT`](crate::vt::IBindCtxVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "shlwapi")))]
pub struct IBindCtx(ComPtr);

impl_iunknown!(IBindCtx, 0x0000000e, 0x0000, 0x0000, 0xc000, 0x000000000046);
impl ShlwapiIBindCtx for IBindCtx {}

/// [`IBindCtx`](crate::IBindCtx) methods from `shlwapi` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "shlwapi")))]
pub trait ShlwapiIBindCtx: OleIUnknown {
	/// [`IBindCtx::ReleaseBoundObjects`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ibindctx-releaseboundobjects)
	/// method.
	fn ReleaseBoundObjects(&self) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IBindCtxVT);
				(vt.ReleaseBoundObjects)(self.ptr())
			}
		)
	}

	/// [`IBindCtx::RevokeObjectParam`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ibindctx-revokeobjectparam)
	/// method.
	fn RevokeObjectParam(&self, key: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IBindCtxVT);
				(vt.RevokeObjectParam)(
					self.ptr(),
					WString::from_str(key).as_ptr(),
				)
			}
		)
	}
}
