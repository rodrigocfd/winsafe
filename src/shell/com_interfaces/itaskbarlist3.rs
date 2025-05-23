#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::vts::*;

com_interface! { ITaskbarList3: "ea1afb91-9e28-4b86-90e9-9e9f8a5eefaf";
	/// [`ITaskbarList3`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let obj = w::CoCreateInstance::<w::ITaskbarList3>(
	///     &co::CLSID::TaskbarList,
	///     None::<&w::IUnknown>,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl shell_ITaskbarList for ITaskbarList3 {}
impl shell_ITaskbarList2 for ITaskbarList3 {}
impl shell_ITaskbarList3 for ITaskbarList3 {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`ITaskbarList3`](crate::ITaskbarList3).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_ITaskbarList3: shell_ITaskbarList2 {
	/// [`ITaskbarList3::RegisterTab`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-registertab)
	/// method.
	fn RegisterTab(&self, hwnd_tab: &HWND, hwnd_mdi: &HWND) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<ITaskbarList3VT>(self).RegisterTab)(self.ptr(), hwnd_tab.ptr(), hwnd_mdi.ptr())
		})
	}

	/// [`ITaskbarList3::SetOverlayIcon`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setoverlayicon)
	/// method.
	fn SetOverlayIcon(
		&self,
		hwnd: &HWND,
		hicon: Option<&HICON>,
		description: &str,
	) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<ITaskbarList3VT>(self).SetOverlayIcon)(
				self.ptr(),
				hwnd.ptr(),
				hicon.map_or(std::ptr::null_mut(), |h| h.ptr()),
				WString::from_str(description).as_ptr(),
			)
		})
	}

	/// [`ITaskbarList3::SetProgressState`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
	/// method.
	fn SetProgressState(&self, hwnd: &HWND, tbpf_flags: co::TBPF) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<ITaskbarList3VT>(self).SetProgressState)(self.ptr(), hwnd.ptr(), tbpf_flags.raw())
		})
	}

	/// [`ITaskbarList3::SetProgressValue`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressvalue)
	/// method.
	///
	/// # Examples
	///
	/// Setting progress to 50%:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let tbar: w::ITaskbarList3; // initialized somewhere
	/// # let tbar = unsafe { w::ITaskbarList3::null() };
	/// let hwnd: w::HWND;
	/// # let hwnd = w::HWND::NULL;
	///
	/// tbar.SetProgressValue(&hwnd, 50, 100)?;
	/// # w::HrResult::Ok(())
	/// ```
	fn SetProgressValue(&self, hwnd: &HWND, completed: u64, total: u64) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<ITaskbarList3VT>(self).SetProgressValue)(self.ptr(), hwnd.ptr(), completed, total)
		})
	}

	/// [`ITaskbarList3::SetTabActive`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
	/// method.
	fn SetTabActive(&self, hwnd_tab: &HWND, hwnd_mdi: &HWND) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<ITaskbarList3VT>(self).SetTabActive)(
				self.ptr(),
				hwnd_tab.ptr(),
				hwnd_mdi.ptr(),
				0,
			)
		})
	}

	/// [`ITaskbarList3::SetTabOrder`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
	/// method.
	fn SetTabOrder(&self, hwnd_tab: &HWND, hwnd_insert_before: &HWND) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<ITaskbarList3VT>(self).SetTabOrder)(
				self.ptr(),
				hwnd_tab.ptr(),
				hwnd_insert_before.ptr(),
			)
		})
	}

	/// [`ITaskbarList3::SetThumbnailClip`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setthumbnailclip)
	/// method.
	fn SetThumbnailClip(&self, hwnd: &HWND, clip: Option<RECT>) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<ITaskbarList3VT>(self).SetThumbnailClip)(
				self.ptr(),
				hwnd.ptr(),
				pcvoid_or_null(clip.as_ref()),
			)
		})
	}

	/// [`ITaskbarList3::SetThumbnailTooltip`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setthumbnailtooltip)
	/// method.
	fn SetThumbnailTooltip(&self, hwnd: &HWND, tip: Option<&str>) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<ITaskbarList3VT>(self).SetThumbnailTooltip)(
				self.ptr(),
				hwnd.ptr(),
				WString::from_opt_str(tip).as_ptr(),
			)
		})
	}
}
