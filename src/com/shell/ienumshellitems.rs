#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::HRESULT;
use crate::privs::hr_to_winresult;

/// [`IEnumShellItems`](crate::shell::IEnumShellItems) virtual table.
pub struct IEnumShellItemsVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(ComPtr, *mut ComPtr, *mut u32) -> HRESULT,
	pub Skip: fn(ComPtr, u32) -> HRESULT,
	pub Reset: fn(ComPtr) -> HRESULT,
	pub Clone: fn(ComPtr, u32) -> HRESULT,
}

/// [`IEnumShellItems`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ienumshellitems)
/// COM interface over
/// [`IEnumShellItemsVT`](crate::shell::vt::IEnumShellItemsVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IEnumShellItems(ComPtr);

impl_iunknown!(IEnumShellItems, 0x70629033, 0xe363, 0x4a28, 0xa567, 0x0db78006e6d7);
impl IEnumShellItemsT for IEnumShellItems {}

/// Exposes the [`IEnumShellItems`](crate::shell::IEnumShellItems) methods.
pub trait IEnumShellItemsT: IUnknownT {
	/// [`IEnumShellItems::Reset`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ienumshellitems-reset)
	/// method.
	fn reset(&self) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IEnumShellItemsVT);
			hr_to_winresult((vt.Reset)(self.ptr()))
		}
	}

	/// [`IEnumShellItems::Skip`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ienumshellitems-skip)
	/// method.
	fn skip(&self, count: u32) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IEnumShellItemsVT);
			hr_to_winresult((vt.Skip)(self.ptr(), count))
		}
	}
}
