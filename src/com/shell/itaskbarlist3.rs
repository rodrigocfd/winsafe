#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::iunknown::ComPtr;
use crate::com::shell;
use crate::com::shell::itaskbarlist::ITaskbarListT;
use crate::com::shell::itaskbarlist2::{ITaskbarList2T, ITaskbarList2VT};
use crate::ffi::{HANDLE, HRESULT, PCSTR, PVOID};
use crate::handles::HWND;
use crate::privs::hr_to_winresult;

/// [`ITaskbarList3`](crate::shell::ITaskbarList3) virtual table.
#[repr(C)]
pub struct ITaskbarList3VT {
	pub ITaskbarList2VT: ITaskbarList2VT,
	pub SetProgressValue: fn(ComPtr, HANDLE, u64, u64) -> HRESULT,
	pub SetProgressState: fn(ComPtr, HANDLE, u32) -> HRESULT,
	pub RegisterTab: fn(ComPtr, HANDLE, HANDLE) -> HRESULT,
	pub UnregisterTab: fn(ComPtr, HANDLE) -> HRESULT,
	pub SetTabOrder: fn(ComPtr, HANDLE, HANDLE) -> HRESULT,
	pub SetTabActive: fn(ComPtr, HANDLE, HANDLE, u32) -> HRESULT,
	pub ThumbBarAddButtons: fn(ComPtr, HANDLE, u32, PVOID) -> HRESULT,
	pub ThumbBarUpdateButtons: fn(ComPtr, HANDLE, u32, PVOID) -> HRESULT,
	pub ThumbBarSetImageList: fn(ComPtr, HANDLE, HANDLE) -> HRESULT,
	pub SetOverlayIcon: fn(ComPtr, HANDLE, HANDLE, PCSTR) -> HRESULT,
	pub SetThumbnailTooltip: fn(ComPtr, HANDLE, PCSTR) -> HRESULT,
	pub SetThumbnailClip: fn(ComPtr, HANDLE, PVOID) -> HRESULT,
}

/// [`ITaskbarList3`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3)
/// COM interface over [`ITaskbarList3VT`](crate::shell::vt::ITaskbarList3VT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::prelude::*;
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj = CoCreateInstance::<shell::ITaskbarList3>(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct ITaskbarList3(ComPtr);

impl_iunknown!(ITaskbarList3, 0xea1afb91, 0x9e28, 0x4b86, 0x90e9, 0x9e9f8a5eefaf);
impl ITaskbarListT for ITaskbarList3 {}
impl ITaskbarList2T for ITaskbarList3 {}
impl ITaskbarList3T for ITaskbarList3 {}

/// Exposes the [`ITaskbarList3`](crate::shell::ITaskbarList3) methods.
pub trait ITaskbarList3T: ITaskbarList2T {
	/// [`ITaskbarList3::RegisterTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-registertab)
	/// method.
	fn RegisterTab(&self, hwnd_tab: HWND, hwnd_mdi: HWND) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			hr_to_winresult(
				(vt.RegisterTab)(self.ptr(), hwnd_tab.ptr, hwnd_mdi.ptr),
			)
		}
	}

	/// [`ITaskbarList3::SetProgressState`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
	/// method.
	fn SetProgressState(&self,
		hwnd: HWND, tbpf_flags: shell::co::TBPF) -> WinResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			hr_to_winresult(
				(vt.SetProgressState)(self.ptr(), hwnd.ptr, tbpf_flags.0),
			)
		}
	}

	/// [`ITaskbarList3::SetProgressValue`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressvalue)
	/// method.
	///
	/// # Examples
	///
	/// Setting progress to 50%:
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::{HWND, shell};
	///
	/// let obj: shell::ITaskbarList3; // initialized somewhere
	/// let hwnd: HWND;
	///
	/// obj.SetProgressValue(hwnd, 50, 100)?;
	/// ```
	fn SetProgressValue(&self,
		hwnd: HWND, completed: u64, total: u64) -> WinResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			hr_to_winresult(
				(vt.SetProgressValue)(self.ptr(), hwnd.ptr, completed, total),
			)
		}
	}

	/// [`ITaskbarList3::SetTabActive`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
	/// method.
	fn SetTabActive(&self, hwnd_tab: HWND, hwnd_mdi: HWND) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			hr_to_winresult(
				(vt.SetTabActive)(self.ptr(), hwnd_tab.ptr, hwnd_mdi.ptr, 0),
			)
		}
	}

	/// [`ITaskbarList3::SetTabOrder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
	/// method.
	fn SetTabOrder(&self,
		hwnd_tab: HWND, hwnd_insert_before: HWND) -> WinResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			hr_to_winresult(
				(vt.SetTabOrder)(self.ptr(), hwnd_tab.ptr, hwnd_insert_before.ptr),
			)
		}
	}
}
