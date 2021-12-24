#![allow(non_snake_case)]

use crate::co;
use crate::ffi_types::{HANDLE, HRES, PCSTR, PVOID};
use crate::kernel::decl::WString;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{ShellITaskbarList, ShellITaskbarList2};
use crate::user::decl::{HICON, HWND, RECT};
use crate::vt::ITaskbarList2VT;

/// [`ITaskbarList3`](crate::ITaskbarList3) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
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
/// COM interface over [`ITaskbarList3VT`](crate::vt::ITaskbarList3VT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{CLSID, co, CoCreateInstance, ITaskbarList3};
///
/// let obj = CoCreateInstance::<ITaskbarList3>(
///     &CLSID::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// # Ok::<_, co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub struct ITaskbarList3(ComPtr);

impl_iunknown!(ITaskbarList3, 0xea1afb91, 0x9e28, 0x4b86, 0x90e9, 0x9e9f8a5eefaf);
impl ShellITaskbarList for ITaskbarList3 {}
impl ShellITaskbarList2 for ITaskbarList3 {}
impl ShellITaskbarList3 for ITaskbarList3 {}

/// [`ITaskbarList3`](crate::ITaskbarList3) methods from `shell` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait ShellITaskbarList3: ShellITaskbarList2 {
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
		hwnd: HWND, tbpf_flags: co::TBPF) -> HrResult<()>
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
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HWND, ITaskbarList3};
	///
	/// let tbar: ITaskbarList3; // initialized somewhere
	/// # use winsafe::{CLSID, co::CLSCTX, CoCreateInstance};
	/// # let tbar = CoCreateInstance::<ITaskbarList3>(&CLSID::new(0,0,0,0,0), None, CLSCTX::INPROC_SERVER)?;
	/// let hwnd: HWND;
	/// # let hwnd = HWND::NULL;
	///
	/// tbar.SetProgressValue(hwnd, 50, 100)?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
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
