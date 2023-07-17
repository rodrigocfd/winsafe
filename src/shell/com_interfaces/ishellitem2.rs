#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{BOOL, COMPTR, HRES, PCVOID, PSTR, PVOID};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::{ole_IBindCtx, shell_IShellItem};
use crate::vt::IShellItemVT;

/// [`IShellItem2`](crate::IShellItem2) virtual table.
#[repr(C)]
pub struct IShellItem2VT {
	pub IShellItemVT: IShellItemVT,
	pub GetPropertyStore: fn(COMPTR, u32, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyStoreWithCreateObject: fn(COMPTR, u32, COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyStoreForKeys: fn(COMPTR, PCVOID, u32, u32, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyDescriptionList: fn(COMPTR, PCVOID, PCVOID, *mut COMPTR) -> HRES,
	pub Update: fn(COMPTR, COMPTR) -> HRES,
	pub GetProperty: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub GetCLSID: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub GetFileTime: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub GetInt32: fn(COMPTR, PCVOID, *mut i32) -> HRES,
	pub GetString: fn(COMPTR, PCVOID, *mut PSTR) -> HRES,
	pub GetUInt32: fn(COMPTR, PCVOID, *mut u32) -> HRES,
	pub GetUInt64: fn(COMPTR, PCVOID, *mut u64) -> HRES,
	pub GetBool: fn(COMPTR, PCVOID, *mut BOOL) -> HRES,
}

com_interface! { IShellItem2: "7e9fb0d3-919f-4307-ab2e-9b1860310c93";
	/// [`IShellItem2`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem2)
	/// COM interface over [`IShellItem2VT`](crate::vt::IShellItem2VT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with
	/// [`SHCreateItemFromParsingName`](crate::SHCreateItemFromParsingName)
	/// function.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IBindCtx, IShellItem2, SHCreateItemFromParsingName};
	///
	/// let shi = SHCreateItemFromParsingName::<IShellItem2>(
	///     "C:\\Temp\\foo.txt",
	///     None::<&IBindCtx>,
	/// )?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
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
	fn Update(&self, pbc: &impl ole_IBindCtx) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { (vt::<IShellItem2VT>(self).Update)(self.ptr(), pbc.ptr()) },
		)
	}
}
