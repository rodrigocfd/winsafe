#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::dshow;
use crate::com::dshow::any_structs::MFVideoNormalizedRect;
use crate::com::funcs::CoTaskMemFree;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{BOOL, HANDLE, HRESULT, PCVOID, PVOID};
use crate::handles::{Handle, HWND};
use crate::privs::hr_to_winresult;
use crate::structs::{BITMAPINFOHEADER, COLORREF, RECT, SIZE};

/// [`IMFVideoDisplayControl`](crate::dshow::IMFVideoDisplayControl) virtual
/// table.
#[repr(C)]
pub struct IMFVideoDisplayControlVT {
	pub IUnknownVT: IUnknownVT,
	pub GetNativeVideoSize: fn(ComPtr, PVOID, PVOID) -> HRESULT,
	pub GetIdealVideoSize: fn(ComPtr, PVOID, PVOID) -> HRESULT,
	pub SetVideoPosition: fn(ComPtr, PCVOID, PCVOID) -> HRESULT,
	pub GetVideoPosition: fn(ComPtr, PVOID, PCVOID) -> HRESULT,
	pub SetAspectRatioMode: fn(ComPtr, u32) -> HRESULT,
	pub GetAspectRatioMode: fn(ComPtr, *mut u32) -> HRESULT,
	pub SetVideoWindow: fn(ComPtr, HANDLE) -> HRESULT,
	pub GetVideoWindow: fn(ComPtr, *mut HANDLE) -> HRESULT,
	pub RepaintVideo: fn(ComPtr) -> HRESULT,
	pub GetCurrentImage: fn(ComPtr, PVOID, *mut *mut u8, *mut u32, *mut i64) -> HRESULT,
	pub SetBorderColor: fn(ComPtr, u32) -> HRESULT,
	pub GetBorderColor: fn(ComPtr, *mut u32) -> HRESULT,
	pub SetRenderingPrefs: fn(ComPtr, u32) -> HRESULT,
	pub GetRenderingPrefs: fn(ComPtr, *mut u32) -> HRESULT,
	pub SetFullscreen: fn(ComPtr, BOOL) -> HRESULT,
	pub GetFullscreen: fn(ComPtr, *mut BOOL) -> HRESULT,
}

/// [`IMFVideoDisplayControl`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nn-evr-imfvideodisplaycontrol)
/// COM interface over
/// [`IMFVideoDisplayControlVT`](crate::dshow::vt::IMFVideoDisplayControlVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::prelude::*;
/// use winsafe::dshow;
///
/// let get_svc: dshow::IMFGetService; // initialized somewhere
///
/// let controller_evr = get_svc
///     .GetService::<dshow::IMFVideoDisplayControl>(
///         &dshow::guid::MR_VIDEO_RENDER_SERVICE,
///     )?;
/// ```
pub struct IMFVideoDisplayControl(ComPtr);

impl_iunknown!(IMFVideoDisplayControl, 0xa490b1e4, 0xab84, 0x4d31, 0xa1b2, 0x181e03b1077a);
impl IMFVideoDisplayControlT for IMFVideoDisplayControl {}

/// Exposes the [`IMFVideoDisplayControl`](crate::dshow::IMFVideoDisplayControl)
/// methods.
pub trait IMFVideoDisplayControlT: IUnknownT {
	/// [`IMFVideoDisplayControl::GetAspectRatioMode`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getaspectratiomode)
	/// method.
	fn GetAspectRatioMode(&self) -> WinResult<dshow::co::MFVideoARMode> {
		let mut mode = dshow::co::MFVideoARMode::None;
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult(
				(vt.GetAspectRatioMode)(self.ptr(), &mut mode as *mut _ as _),
			)
		}.map(|_| mode)
	}

	/// [`IMFVideoDisplayControl::GetBorderColor`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getbordercolor)
	/// method;
	fn GetBorderColor(&self) -> WinResult<COLORREF> {
		let mut color = COLORREF::new(0, 0, 0);
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult((vt.GetBorderColor)(self.ptr(), &mut color.0))
		}.map(|_| color)
	}

	/// [`GetCurrentImage`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getcurrentimage)
	/// method.
	///
	/// Returns bitmap description, DIB bytes and time stamp.
	fn GetCurrentImage(&self)
		-> WinResult<(BITMAPINFOHEADER, Vec<u8>, i64)>
	{
		let mut bih = BITMAPINFOHEADER::default();
		let mut dib_ptr: *mut u8 = std::ptr::null_mut();
		let mut dib_sz = u32::default();
		let mut time_stamp = i64::default();

		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult(
				(vt.GetCurrentImage)(
					self.ptr(),
					&mut bih as *mut _ as _,
					&mut dib_ptr,
					&mut dib_sz,
					&mut time_stamp,
				),
			)
		}.map(|_| {
			let dib_vec = unsafe {
				std::slice::from_raw_parts(dib_ptr, dib_sz as _)
			}.to_vec();
			CoTaskMemFree(dib_ptr);
			(bih, dib_vec, time_stamp)
		})
	}

	/// [`IMFVideoDisplayControl::GetFullscreen`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getfullscreen)
	/// method.
	fn GetFullscreen(&self) -> WinResult<bool> {
		let mut fulls = false;
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult(
				(vt.GetFullscreen)(self.ptr(), &mut fulls as *mut _ as _),
			)
		}.map(|_| fulls)
	}

	/// [`IMFVideoDisplayControl::GetIdealVideoSize`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getidealvideosize)
	/// method.
	///
	/// Returns minimum and maximum ideal sizes.
	fn GetIdealVideoSize(&self) -> WinResult<(SIZE, SIZE)> {
		let (mut min, mut max) = (SIZE::default(), SIZE::default());
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult(
				(vt.GetIdealVideoSize)(
					self.ptr(),
					&mut min as *mut _ as _,
					&mut max as *mut _ as _,
				),
			)
		}.map(|_| (min, max))
	}

	/// [`IMFVideoDisplayControl::GetNativeVideoSize`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getnativevideosize)
	/// method.
	///
	/// Returns native and aspect ratio sizes.
	fn GetNativeVideoSize(&self) -> WinResult<(SIZE, SIZE)> {
		let (mut native, mut aspec) = (SIZE::default(), SIZE::default());
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult(
				(vt.GetNativeVideoSize)(
					self.ptr(),
					&mut native as *mut _ as _,
					&mut aspec as *mut _ as _,
				),
			)
		}.map(|_| (native, aspec))
	}

	/// [`IMFVideoDisplayControl::GetVideoPosition`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getvideoposition)
	/// method.
	fn GetVideoPosition(&self)
		-> WinResult<(MFVideoNormalizedRect, RECT)>
	{
		let mut norm_rc = MFVideoNormalizedRect::default();
		let mut rc = RECT::default();

		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult(
				(vt.GetVideoPosition)(
					self.ptr(),
					&mut norm_rc as *mut _ as _,
					&mut rc as *mut _ as _,
				),
			)
		}.map(|_| (norm_rc, rc))
	}

	/// [`IMFVideoDisplayControl::GetVideoWindow`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getvideowindow)
	/// method.
	fn GetVideoWindow(&self) -> WinResult<HWND> {
		let mut hwnd = HWND::NULL;
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult((vt.GetVideoWindow)(self.ptr(), &mut hwnd.0))
		}.map(|_| hwnd)
	}

	/// [`IMFVideoDisplayControl::RepaintVideo`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-repaintvideo)
	/// method.
	fn RepaintVideo(&self) -> WinResult<()> {
		match co::ERROR(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
				(vt.RepaintVideo)(self.ptr()) as _
			}
		) {
			co::ERROR::S_OK | co::ERROR::MF_E_INVALIDREQUEST => Ok(()),
			err => Err(err),
		}
	}

	/// [`IMFVideoDisplayControl::SetAspectRatioMode`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setaspectratiomode)
	/// method.
	fn SetAspectRatioMode(&self,
		mode: dshow::co::MFVideoARMode) -> WinResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult((vt.SetAspectRatioMode)(self.ptr(), mode.0))
		}
	}

	/// [`IMFVideoDisplayControl::SetBorderColor`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setbordercolor)
	/// method.
	fn SetBorderColor(&self, color: COLORREF) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult((vt.SetBorderColor)(self.ptr(), color.0))
		}
	}

	/// [`IMFVideoDisplayControl::SetFullscreen`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setfullscreen)
	/// method.
	fn SetFullscreen(&self, full_screen: bool) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult((vt.SetFullscreen)(self.ptr(), full_screen as _))
		}
	}

	/// [`IMFVideoDisplayControl::SetVideoPosition`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setvideoposition)
	/// method.
	///
	/// At least one parameter must be passed.
	fn SetVideoPosition(&self,
		src: Option<MFVideoNormalizedRect>,
		dest: Option<RECT>) -> WinResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult(
				(vt.SetVideoPosition)(
					self.ptr(),
					src.as_ref().map_or(std::ptr::null(), |src| src as *const _ as _),
					dest.as_ref().map_or(std::ptr::null(), |dest| dest as *const _ as _),
				),
			)
		}
	}

	/// [`IMFVideoDisplayControl::SetVideoWindow`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setvideowindow)
	/// method.
	fn SetVideoWindow(&self, hwnd_video: HWND) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMFVideoDisplayControlVT);
			hr_to_winresult((vt.SetVideoWindow)(self.ptr(), hwnd_video.0))
		}
	}
}
