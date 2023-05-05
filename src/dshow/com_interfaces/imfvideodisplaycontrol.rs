#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::dshow::decl::MFVideoNormalizedRect;
use crate::kernel::ffi_types::{BOOL, HANDLE, HRES, PCVOID, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{Handle, IntUnderlying, ole_IUnknown};
use crate::user::decl::{COLORREF, HWND, RECT, SIZE};
use crate::vt::IUnknownVT;

/// [`IMFVideoDisplayControl`](crate::IMFVideoDisplayControl) virtual table.
#[repr(C)]
pub struct IMFVideoDisplayControlVT {
	pub IUnknownVT: IUnknownVT,
	pub GetNativeVideoSize: fn(ComPtr, PVOID, PVOID) -> HRES,
	pub GetIdealVideoSize: fn(ComPtr, PVOID, PVOID) -> HRES,
	pub SetVideoPosition: fn(ComPtr, PCVOID, PCVOID) -> HRES,
	pub GetVideoPosition: fn(ComPtr, PVOID, PCVOID) -> HRES,
	pub SetAspectRatioMode: fn(ComPtr, u32) -> HRES,
	pub GetAspectRatioMode: fn(ComPtr, *mut u32) -> HRES,
	pub SetVideoWindow: fn(ComPtr, HANDLE) -> HRES,
	pub GetVideoWindow: fn(ComPtr, *mut HANDLE) -> HRES,
	pub RepaintVideo: fn(ComPtr) -> HRES,
	pub GetCurrentImage: fn(ComPtr, PVOID, *mut *mut u8, *mut u32, *mut i64) -> HRES,
	pub SetBorderColor: fn(ComPtr, u32) -> HRES,
	pub GetBorderColor: fn(ComPtr, *mut u32) -> HRES,
	pub SetRenderingPrefs: fn(ComPtr, u32) -> HRES,
	pub GetRenderingPrefs: fn(ComPtr, *mut u32) -> HRES,
	pub SetFullscreen: fn(ComPtr, BOOL) -> HRES,
	pub GetFullscreen: fn(ComPtr, *mut BOOL) -> HRES,
}

com_interface! { IMFVideoDisplayControl: "a490b1e4-ab84-4d31-a1b2-181e03b1077a";
	/// [`IMFVideoDisplayControl`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nn-evr-imfvideodisplaycontrol)
	/// COM interface over
	/// [`IMFVideoDisplayControlVT`](crate::vt::IMFVideoDisplayControlVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IMFGetService, IMFVideoDisplayControl};
	///
	/// let get_svc: IMFGetService; // initialized somewhere
	/// # let get_svc = IMFGetService::from(unsafe { winsafe::ComPtr::null() });
	///
	/// let controller_evr = get_svc
	///     .GetService::<IMFVideoDisplayControl>(
	///         &co::DSHOW_SERVICE::MR_VIDEO_RENDER_SERVICE,
	///     )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl dshow_IMFVideoDisplayControl for IMFVideoDisplayControl {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IMFVideoDisplayControl`](crate::IMFVideoDisplayControl).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IMFVideoDisplayControl: ole_IUnknown {
	/// [`IMFVideoDisplayControl::GetAspectRatioMode`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getaspectratiomode)
	/// method.
	#[must_use]
	fn GetAspectRatioMode(&self) -> HrResult<co::MFVideoARMode> {
		let mut mode = co::MFVideoARMode::None;
		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult(
				(vt.GetAspectRatioMode)(self.ptr(), &mut mode as *mut _ as _),
			)
		}.map(|_| mode)
	}

	/// [`IMFVideoDisplayControl::GetBorderColor`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getbordercolor)
	/// method;
	#[must_use]
	fn GetBorderColor(&self) -> HrResult<COLORREF> {
		let mut color = COLORREF::default();
		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult((vt.GetBorderColor)(self.ptr(), color.as_mut()))
		}.map(|_| color)
	}

	/// [`IMFVideoDisplayControl::GetFullscreen`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getfullscreen)
	/// method.
	#[must_use]
	fn GetFullscreen(&self) -> HrResult<bool> {
		let mut fulls = false;
		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult(
				(vt.GetFullscreen)(self.ptr(), &mut fulls as *mut _ as _),
			)
		}.map(|_| fulls)
	}

	/// [`IMFVideoDisplayControl::GetIdealVideoSize`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getidealvideosize)
	/// method.
	///
	/// Returns minimum and maximum ideal sizes.
	#[must_use]
	fn GetIdealVideoSize(&self) -> HrResult<(SIZE, SIZE)> {
		let (mut min, mut max) = (SIZE::default(), SIZE::default());
		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult(
				(vt.GetIdealVideoSize)(
					self.ptr(),
					&mut min as *mut _ as _,
					&mut max as *mut _ as _,
				),
			)
		}.map(|_| (min, max))
	}

	/// [`IMFVideoDisplayControl::GetNativeVideoSize`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getnativevideosize)
	/// method.
	///
	/// Returns native and aspect ratio sizes.
	#[must_use]
	fn GetNativeVideoSize(&self) -> HrResult<(SIZE, SIZE)> {
		let (mut native, mut aspec) = (SIZE::default(), SIZE::default());
		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult(
				(vt.GetNativeVideoSize)(
					self.ptr(),
					&mut native as *mut _ as _,
					&mut aspec as *mut _ as _,
				),
			)
		}.map(|_| (native, aspec))
	}

	/// [`IMFVideoDisplayControl::GetVideoPosition`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getvideoposition)
	/// method.
	#[must_use]
	fn GetVideoPosition(&self) -> HrResult<(MFVideoNormalizedRect, RECT)> {
		let mut norm_rc = MFVideoNormalizedRect::default();
		let mut rc = RECT::default();

		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult(
				(vt.GetVideoPosition)(
					self.ptr(),
					&mut norm_rc as *mut _ as _,
					&mut rc as *mut _ as _,
				),
			)
		}.map(|_| (norm_rc, rc))
	}

	/// [`IMFVideoDisplayControl::GetVideoWindow`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getvideowindow)
	/// method.
	#[must_use]
	fn GetVideoWindow(&self) -> HrResult<HWND> {
		let mut hwnd = HWND::NULL;
		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult((vt.GetVideoWindow)(self.ptr(), hwnd.as_mut()))
		}.map(|_| hwnd)
	}

	/// [`IMFVideoDisplayControl::RepaintVideo`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-repaintvideo)
	/// method.
	fn RepaintVideo(&self) -> HrResult<()> {
		match co::HRESULT(
			unsafe {
				let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
				(vt.RepaintVideo)(self.ptr())
			},
		) {
			co::HRESULT::S_OK
			| co::HRESULT::MF_E_INVALIDREQUEST => Ok(()),
			hr => Err(hr),
		}
	}

	/// [`IMFVideoDisplayControl::SetAspectRatioMode`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setaspectratiomode)
	/// method.
	fn SetAspectRatioMode(&self, mode: co::MFVideoARMode) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult((vt.SetAspectRatioMode)(self.ptr(), mode.0))
		}
	}

	/// [`IMFVideoDisplayControl::SetBorderColor`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setbordercolor)
	/// method.
	fn SetBorderColor(&self, color: COLORREF) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult((vt.SetBorderColor)(self.ptr(), color.into()))
		}
	}

	/// [`IMFVideoDisplayControl::SetFullscreen`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setfullscreen)
	/// method.
	fn SetFullscreen(&self, full_screen: bool) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult((vt.SetFullscreen)(self.ptr(), full_screen as _))
		}
	}

	/// [`IMFVideoDisplayControl::SetVideoPosition`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setvideoposition)
	/// method.
	///
	/// At least one parameter must be passed.
	fn SetVideoPosition(&self,
		src: Option<MFVideoNormalizedRect>, dest: Option<RECT>) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult(
				(vt.SetVideoPosition)(
					self.ptr(),
					src.as_ref().map_or(std::ptr::null(), |src| src as *const _ as _),
					dest.as_ref().map_or(std::ptr::null(), |dest| dest as *const _ as _),
				),
			)
		}
	}

	/// [`IMFVideoDisplayControl::SetVideoWindow`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setvideowindow)
	/// method.
	fn SetVideoWindow(&self, hwnd_video: &HWND) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IMFVideoDisplayControlVT>();
			ok_to_hrresult((vt.SetVideoWindow)(self.ptr(), hwnd_video.as_ptr()))
		}
	}
}
