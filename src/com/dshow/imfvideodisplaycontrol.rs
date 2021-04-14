#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::{IUnknown, IUnknownVT, PPComVT};
use crate::com::dshow::vt::IMFVideoDisplayControlVT;
use crate::com::funcs::{hr_to_winresult, hr_to_winresult_bool};
use crate::dshow::MFVideoNormalizedRect;
use crate::handles::HWND;
use crate::structs::{RECT, SIZE};

/// [`IMFVideoDisplayControl`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nn-evr-imfvideodisplaycontrol)
/// COM interface.
///
/// Virtual table: [`IMFVideoDisplayControlVT`](crate::dshow::vt::IMFVideoDisplayControlVT).
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IMFVideoDisplayControl {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<IMFVideoDisplayControlVT>> for IMFVideoDisplayControl {
	fn from(ppv: PPComVT<IMFVideoDisplayControlVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl IMFVideoDisplayControl {
	unsafe fn ppv(&self) -> PPComVT<IMFVideoDisplayControlVT> {
		self.IUnknown.ppv::<IMFVideoDisplayControlVT>()
	}

	/// [`IMFVideoDisplayControl::GetAspectRatioMode`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getaspectratiomode)
	/// method.
	pub fn GetAspectRatioMode(&self) -> WinResult<co::MFVideoARMode> {
		let mut mode = co::MFVideoARMode::None;
		hr_to_winresult_bool(
			unsafe {
				((**self.ppv()).GetAspectRatioMode)(
					self.ppv(),
					&mut mode as *mut _ as *mut _,
				)
			},
		).map(|_| mode)
	}

	/// [`IMFVideoDisplayControl::GetFullscreen`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getfullscreen)
	/// method.
	pub fn GetFullscreen(&self) -> WinResult<bool> {
		let mut fulls = false;
		hr_to_winresult_bool(
			unsafe {
				((**self.ppv()).GetFullscreen)(
					self.ppv(),
					&mut fulls as *mut _ as *mut _,
				)
			},
		).map(|_| fulls)
	}

	/// [`IMFVideoDisplayControl::GetIdealVideoSize`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getidealvideosize)
	/// method.
	///
	/// Returns minimum and maximum ideal sizes.
	pub fn GetIdealVideoSize(&self) -> WinResult<(SIZE, SIZE)> {
		let mut min = SIZE::default();
		let mut max = SIZE::default();
		hr_to_winresult_bool(
			unsafe {
				((**self.ppv()).GetIdealVideoSize)(
					self.ppv(),
					&mut min as *mut _ as *mut _,
					&mut max as *mut _ as *mut _,
				)
			},
		).map(|_| (min, max))
	}

	/// [`IMFVideoDisplayControl::GetNativeVideoSize`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getnativevideosize)
	/// method.
	///
	/// Returns native and aspect ratio sizes.
	pub fn GetNativeVideoSize(&self) -> WinResult<(SIZE, SIZE)> {
		let mut native = SIZE::default();
		let mut aspec = SIZE::default();
		hr_to_winresult_bool(
			unsafe {
				((**self.ppv()).GetNativeVideoSize)(
					self.ppv(),
					&mut native as *mut _ as *mut _,
					&mut aspec as *mut _ as *mut _,
				)
			},
		).map(|_| (native, aspec))
	}

	/// [`IMFVideoDisplayControl::GetVideoWindow`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getvideowindow)
	/// method.
	pub fn GetVideoWindow(&self) -> WinResult<HWND> {
		let mut hwnd = unsafe { HWND::null_handle() };
		hr_to_winresult_bool(
			unsafe { ((**self.ppv()).GetVideoWindow)(self.ppv(), &mut hwnd.ptr) },
		).map(|_| hwnd)
	}

	/// [`IMFVideoDisplayControl::RepaintVideo`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-repaintvideo)
	/// method.
	pub fn RepaintVideo(&self) -> WinResult<()> {
		match co::ERROR(
			unsafe { ((**self.ppv()).RepaintVideo)(self.ppv()) } as u32,
		) {
			co::ERROR::S_OK | co::ERROR::MF_E_INVALIDREQUEST => Ok(()),
			err => Err(err),
		}
	}

	/// [`IMFVideoDisplayControl::SetAspectRatioMode`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setaspectratiomode)
	/// method.
	pub fn SetAspectRatioMode(&self, mode: co::MFVideoARMode) -> WinResult<()> {
		hr_to_winresult(
			unsafe { ((**self.ppv()).SetAspectRatioMode)(self.ppv(), mode.0) },
		)
	}

	/// [`IMFVideoDisplayControl::SetFullscreen`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setfullscreen)
	/// method.
	pub fn SetFullscreen(&self, fullScreen: bool) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetFullscreen)(self.ppv(), fullScreen as i32)
			},
		)
	}

	/// [`IMFVideoDisplayControl::SetVideoPosition`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setvideoposition)
	/// method.
	///
	/// At least one parameter must be passed.
	pub fn SetVideoPosition(&self,
		src: Option<MFVideoNormalizedRect>, dest: Option<RECT>) -> WinResult<()>
	{
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetVideoPosition)(
					self.ppv(),
					match &src {
						Some(src) => src as *const _ as *const _,
						None => std::ptr::null(),
					},
					match &dest {
						Some(dest) => dest as *const _ as *const _,
						None => std::ptr::null(),
					},
				)
			},
		)
	}

	/// [`IMFVideoDisplayControl::SetVideoWindow`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setvideowindow)
	/// method.
	pub fn SetVideoWindow(&self, hwndVideo: HWND) -> WinResult<()> {
		hr_to_winresult(
			unsafe { ((**self.ppv()).SetVideoWindow)(self.ppv(), hwndVideo.ptr) },
		)
	}
}
