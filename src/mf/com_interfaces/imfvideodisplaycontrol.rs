#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFVideoDisplayControl: "a490b1e4-ab84-4d31-a1b2-181e03b1077a";
	/// [`IMFVideoDisplayControl`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nn-evr-imfvideodisplaycontrol)
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
	/// let get_svc: w::IMFGetService; // initialized somewhere
	/// # let get_svc = unsafe { w::IMFGetService::null() };
	///
	/// let controller_evr = get_svc
	///     .GetService::<w::IMFVideoDisplayControl>(
	///         &co::MF_SERVICE::MR_VIDEO_RENDER_SERVICE,
	///     )?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl mf_IMFVideoDisplayControl for IMFVideoDisplayControl {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFVideoDisplayControl`](crate::IMFVideoDisplayControl).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFVideoDisplayControl: ole_IUnknown {
	/// [`IMFVideoDisplayControl::GetAspectRatioMode`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getaspectratiomode)
	/// method.
	#[must_use]
	fn GetAspectRatioMode(&self) -> HrResult<co::MFVideoARMode> {
		let mut mode = co::MFVideoARMode::default();
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).GetAspectRatioMode)(self.ptr(), mode.as_mut())
		})
		.to_hrresult()
		.map(|_| mode)
	}

	/// [`IMFVideoDisplayControl::GetBorderColor`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getbordercolor)
	/// method;
	#[must_use]
	fn GetBorderColor(&self) -> HrResult<COLORREF> {
		let mut color = COLORREF::default();
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).GetBorderColor)(self.ptr(), color.as_mut())
		})
		.to_hrresult()
		.map(|_| color)
	}

	/// [`IMFVideoDisplayControl::GetFullscreen`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getfullscreen)
	/// method.
	#[must_use]
	fn GetFullscreen(&self) -> HrResult<bool> {
		let mut fulls = 0;
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).GetFullscreen)(self.ptr(), &mut fulls)
		})
		.to_hrresult()
		.map(|_| fulls != 0)
	}

	/// [`IMFVideoDisplayControl::GetIdealVideoSize`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getidealvideosize)
	/// method.
	///
	/// Returns minimum and maximum ideal sizes.
	#[must_use]
	fn GetIdealVideoSize(&self) -> HrResult<(SIZE, SIZE)> {
		let (mut min, mut max) = (SIZE::default(), SIZE::default());
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).GetIdealVideoSize)(
				self.ptr(),
				pvoid(&mut min),
				pvoid(&mut max),
			)
		})
		.to_hrresult()
		.map(|_| (min, max))
	}

	/// [`IMFVideoDisplayControl::GetNativeVideoSize`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getnativevideosize)
	/// method.
	///
	/// Returns native and aspect ratio sizes.
	#[must_use]
	fn GetNativeVideoSize(&self) -> HrResult<(SIZE, SIZE)> {
		let (mut native, mut aspec) = (SIZE::default(), SIZE::default());
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).GetNativeVideoSize)(
				self.ptr(),
				pvoid(&mut native),
				pvoid(&mut aspec),
			)
		})
		.to_hrresult()
		.map(|_| (native, aspec))
	}

	/// [`IMFVideoDisplayControl::GetVideoPosition`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getvideoposition)
	/// method.
	#[must_use]
	fn GetVideoPosition(&self) -> HrResult<(MFVideoNormalizedRect, RECT)> {
		let mut norm_rc = MFVideoNormalizedRect::default();
		let mut rc = RECT::default();

		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).GetVideoPosition)(
				self.ptr(),
				pvoid(&mut norm_rc),
				pvoid(&mut rc),
			)
		})
		.to_hrresult()
		.map(|_| (norm_rc, rc))
	}

	/// [`IMFVideoDisplayControl::GetVideoWindow`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getvideowindow)
	/// method.
	#[must_use]
	fn GetVideoWindow(&self) -> HrResult<HWND> {
		let mut hwnd = HWND::NULL;
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).GetVideoWindow)(self.ptr(), hwnd.as_mut())
		})
		.to_hrresult()
		.map(|_| hwnd)
	}

	/// [`IMFVideoDisplayControl::RepaintVideo`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-repaintvideo)
	/// method.
	fn RepaintVideo(&self) -> HrResult<()> {
		match unsafe {
			co::HRESULT::from_raw((vt::<IMFVideoDisplayControlVT>(self).RepaintVideo)(self.ptr()))
		} {
			co::HRESULT::S_OK | co::HRESULT::MF_E_INVALIDREQUEST => Ok(()),
			hr => Err(hr),
		}
	}

	/// [`IMFVideoDisplayControl::SetAspectRatioMode`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setaspectratiomode)
	/// method.
	fn SetAspectRatioMode(&self, mode: co::MFVideoARMode) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).SetAspectRatioMode)(self.ptr(), mode.raw())
		})
		.to_hrresult()
	}

	/// [`IMFVideoDisplayControl::SetBorderColor`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setbordercolor)
	/// method.
	fn SetBorderColor(&self, color: COLORREF) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).SetBorderColor)(self.ptr(), color.into())
		})
		.to_hrresult()
	}

	/// [`IMFVideoDisplayControl::SetFullscreen`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setfullscreen)
	/// method.
	fn SetFullscreen(&self, full_screen: bool) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).SetFullscreen)(self.ptr(), full_screen as _)
		})
		.to_hrresult()
	}

	/// [`IMFVideoDisplayControl::SetRenderingPrefs`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setrenderingprefs)
	/// method.
	fn SetRenderingPrefs(&self, render_flags: co::MFVideoRenderPrefs) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).SetRenderingPrefs)(self.ptr(), render_flags.raw())
		})
		.to_hrresult()
	}

	/// [`IMFVideoDisplayControl::SetVideoPosition`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setvideoposition)
	/// method.
	///
	/// At least one parameter must be passed.
	fn SetVideoPosition(
		&self,
		src: Option<MFVideoNormalizedRect>,
		dest: Option<RECT>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).SetVideoPosition)(
				self.ptr(),
				pcvoid_or_null(src.as_ref()),
				pcvoid_or_null(dest.as_ref()),
			)
		})
		.to_hrresult()
	}

	/// [`IMFVideoDisplayControl::SetVideoWindow`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setvideowindow)
	/// method.
	fn SetVideoWindow(&self, hwnd_video: &HWND) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFVideoDisplayControlVT>(self).SetVideoWindow)(self.ptr(), hwnd_video.ptr())
		})
		.to_hrresult()
	}
}
