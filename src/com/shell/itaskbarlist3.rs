#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::iunknown::ComPtr;
use crate::com::shell;
use crate::com::shell::itaskbarlist::ITaskbarListT;
use crate::com::shell::itaskbarlist2::{ITaskbarList2T, ITaskbarList2VT};
use crate::ffi::{HANDLE, HRES, PCSTR, PVOID};
use crate::handles::{HICON, HWND};
use crate::privs::ok_to_hrresult;
use crate::structs::RECT;
use crate::various::WString;

/// [`ITaskbarList3`](crate::shell::ITaskbarList3) virtual table.
#[repr(C)]
pub struct ITaskbarList3VT {
	pub ITaskbarList2VT: ITaskbarList2VT,
	pub SetProgressValue: fn(ComPtr, HANDLE, u64, u64) -> HRES,
	pub SetProgressState: fn(ComPtr, HANDLE, u32) -> HRES,
	pub RegisterTab: fn(ComPtr, HANDLE, HANDLE) -> HRES,
	pub UnregisterTab: fn(ComPtr, HANDLE) -> HRES,
	pub SetTabOrder: fn(ComPtr, HANDLE, HANDLE) -> HRES,
	pub SetTabActive: fn(ComPtr, HANDLE, HANDLE, u32) -> HRES,
	pub ThumbBarAddButtons: fn(ComPtr, HANDLE, u32, PVOID) -> HRES,
	pub ThumbBarUpdateButtons: fn(ComPtr, HANDLE, u32, PVOID) -> HRES,
	pub ThumbBarSetImageList: fn(ComPtr, HANDLE, HANDLE) -> HRES,
	pub SetOverlayIcon: fn(ComPtr, HANDLE, HANDLE, PCSTR) -> HRES,
	pub SetThumbnailTooltip: fn(ComPtr, HANDLE, PCSTR) -> HRES,
	pub SetThumbnailClip: fn(ComPtr, HANDLE, PVOID) -> HRES,
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
	fn RegisterTab(&self, hwnd_tab: HWND, hwnd_mdi: HWND) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			ok_to_hrresult(
				(vt.RegisterTab)(self.ptr(), hwnd_tab.0, hwnd_mdi.0),
			)
		}
	}

	/// [`ITaskbarList3::SetOverlayIcon`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setoverlayicon)
	/// method.
	fn SetOverlayIcon(&self,
		hwnd: HWND, hicon: Option<HICON>, description: &str) -> HrResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			ok_to_hrresult(
				(vt.SetOverlayIcon)(
					self.ptr(),
					hwnd.0,
					hicon.map_or(std::ptr::null_mut(), |h| h.0),
					WString::from_str(description).as_ptr(),
				),
			)
		}
	}

	/// [`ITaskbarList3::SetProgressState`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
	/// method.
	fn SetProgressState(&self,
		hwnd: HWND, tbpf_flags: shell::co::TBPF) -> HrResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			ok_to_hrresult(
				(vt.SetProgressState)(self.ptr(), hwnd.0, tbpf_flags.0),
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
		hwnd: HWND, completed: u64, total: u64) -> HrResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			ok_to_hrresult(
				(vt.SetProgressValue)(self.ptr(), hwnd.0, completed, total),
			)
		}
	}

	/// [`ITaskbarList3::SetTabActive`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
	/// method.
	fn SetTabActive(&self, hwnd_tab: HWND, hwnd_mdi: HWND) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			ok_to_hrresult(
				(vt.SetTabActive)(self.ptr(), hwnd_tab.0, hwnd_mdi.0, 0),
			)
		}
	}

	/// [`ITaskbarList3::SetTabOrder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
	/// method.
	fn SetTabOrder(&self,
		hwnd_tab: HWND, hwnd_insert_before: HWND) -> HrResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			ok_to_hrresult(
				(vt.SetTabOrder)(self.ptr(), hwnd_tab.0, hwnd_insert_before.0),
			)
		}
	}

	/// [`ITaskbarList3::SetThumbnailClip`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setthumbnailclip)
	/// method.
	fn SetThumbnailClip(&self, hwnd: HWND, clip: Option<RECT>) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			ok_to_hrresult(
				(vt.SetThumbnailClip)(self.ptr(), hwnd.0, &clip as *const _ as _),
			)
		}
	}

	/// [`ITaskbarList3::SetThumbnailTooltip`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setthumbnailtooltip)
	/// method.
	fn SetThumbnailTooltip(&self,
		hwnd: HWND, tip: Option<&str>) -> HrResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList3VT);
			ok_to_hrresult(
				(vt.SetThumbnailTooltip)(
					self.ptr(),
					hwnd.0,
					tip.map_or(std::ptr::null_mut(), |s| WString::from_str(s).as_ptr()),
				),
			)
		}
	}
}
