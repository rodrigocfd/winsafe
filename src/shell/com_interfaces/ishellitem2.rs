#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{BOOL, HRES, PCVOID, PSTR, PVOID};
use crate::ole::decl::{ComPtr, HrResult, IBindCtx};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{ole_IUnknown, shell_IShellItem};
use crate::vt::IShellItemVT;

/// [`IShellItem2`](crate::IShellItem2) virtual table.
#[repr(C)]
pub struct IShellItem2VT {
	pub IShellItemVT: IShellItemVT,
	pub GetPropertyStore: fn(ComPtr, u32, PCVOID, *mut ComPtr) -> HRES,
	pub GetPropertyStoreWithCreateObject: fn(ComPtr, u32, ComPtr, PCVOID, *mut ComPtr) -> HRES,
	pub GetPropertyStoreForKeys: fn(ComPtr, PCVOID, u32, u32, PCVOID, *mut ComPtr) -> HRES,
	pub GetPropertyDescriptionList: fn(ComPtr, PCVOID, PCVOID, *mut ComPtr) -> HRES,
	pub Update: fn(ComPtr, ComPtr) -> HRES,
	pub GetProperty: fn(ComPtr, PCVOID, PVOID) -> HRES,
	pub GetCLSID: fn(ComPtr, PCVOID, PVOID) -> HRES,
	pub GetFileTime: fn(ComPtr, PCVOID, PVOID) -> HRES,
	pub GetInt32: fn(ComPtr, PCVOID, *mut i32) -> HRES,
	pub GetString: fn(ComPtr, PCVOID, *mut PSTR) -> HRES,
	pub GetUInt32: fn(ComPtr, PCVOID, *mut u32) -> HRES,
	pub GetUInt64: fn(ComPtr, PCVOID, *mut u64) -> HRES,
	pub GetBool: fn(ComPtr, PCVOID, *mut BOOL) -> HRES,
}

com_interface! { IShellItem2: "7e9fb0d3-919f-4307-ab2e-9b1860310c93";
	/// [`IShellItem2`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem2)
	/// COM interface over [`IShellItem2VT`](crate::vt::IShellItem2VT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_IShellItem for IShellItem2 {}
impl shell_IShellItem2 for IShellItem2 {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellItem2`](crate::IShellItem2).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IShellItem2: shell_IShellItem {
	/// [`IShellItem2::Update`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-update)
	/// method.
	fn Update(&self, pbc: &IBindCtx) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IShellItem2VT>();
			ok_to_hrresult((vt.Update)(self.ptr(), pbc.ptr()))
		}
	}
}
