#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::dshow::vt::IMFVideoDisplayControlVT;
use crate::dshow::MFVideoNormalizedRect;
use crate::handles::HWND;
use crate::privs::{
	hr_to_winresult_bool,
	hr_to_winresult,
	ref_as_pcvoid,
	ref_as_pvoid,
};
use crate::structs::{RECT, SIZE};

macro_rules! IMFVideoDisplayControl_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IMFVideoDisplayControl::GetAspectRatioMode`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getaspectratiomode)
			/// method.
			pub fn GetAspectRatioMode(&self) -> WinResult<co::MFVideoARMode> {
				let ppvt = unsafe { self.ppvt::<IMFVideoDisplayControlVT>() };
				let mut mode = co::MFVideoARMode::None;
				hr_to_winresult_bool(
					unsafe {
						((**ppvt).GetAspectRatioMode)(ppvt, &mut mode as *mut _ as _)
					},
				).map(|_| mode)
			}

			/// [`IMFVideoDisplayControl::GetFullscreen`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getfullscreen)
			/// method.
			pub fn GetFullscreen(&self) -> WinResult<bool> {
				let mut fulls = false;
				let ppvt = unsafe { self.ppvt::<IMFVideoDisplayControlVT>() };
				hr_to_winresult_bool(
					unsafe {
						((**ppvt).GetFullscreen)(ppvt, &mut fulls as *mut _ as _)
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
				let ppvt = unsafe { self.ppvt::<IMFVideoDisplayControlVT>() };
				hr_to_winresult_bool(
					unsafe {
						((**ppvt).GetIdealVideoSize)(
							ppvt,
							ref_as_pvoid(&mut min),
							ref_as_pvoid(&mut max),
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
				let ppvt = unsafe { self.ppvt::<IMFVideoDisplayControlVT>() };
				hr_to_winresult_bool(
					unsafe {
						((**ppvt).GetNativeVideoSize)(
							ppvt,
							ref_as_pvoid(&mut native),
							ref_as_pvoid(&mut aspec),
						)
					},
				).map(|_| (native, aspec))
			}

			/// [`IMFVideoDisplayControl::GetVideoWindow`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getvideowindow)
			/// method.
			pub fn GetVideoWindow(&self) -> WinResult<HWND> {
				let mut hwnd = unsafe { HWND::null_handle() };
				let ppvt = unsafe { self.ppvt::<IMFVideoDisplayControlVT>() };
				hr_to_winresult_bool(
					unsafe { ((**ppvt).GetVideoWindow)(ppvt, &mut hwnd.ptr) },
				).map(|_| hwnd)
			}

			/// [`IMFVideoDisplayControl::RepaintVideo`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-repaintvideo)
			/// method.
			pub fn RepaintVideo(&self) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IMFVideoDisplayControlVT>() };
				match co::ERROR(
					unsafe { ((**ppvt).RepaintVideo)(ppvt) } as _,
				) {
					co::ERROR::S_OK | co::ERROR::MF_E_INVALIDREQUEST => Ok(()),
					err => Err(err),
				}
			}

			/// [`IMFVideoDisplayControl::SetAspectRatioMode`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setaspectratiomode)
			/// method.
			pub fn SetAspectRatioMode(&self, mode: co::MFVideoARMode) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IMFVideoDisplayControlVT>() };
				hr_to_winresult(
					unsafe { ((**ppvt).SetAspectRatioMode)(ppvt, mode.0) },
				)
			}

			/// [`IMFVideoDisplayControl::SetFullscreen`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setfullscreen)
			/// method.
			pub fn SetFullscreen(&self, fullScreen: bool) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IMFVideoDisplayControlVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetFullscreen)(ppvt, fullScreen as _)
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
				let ppvt = unsafe { self.ppvt::<IMFVideoDisplayControlVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetVideoPosition)(
							ppvt,
							src.as_ref().map_or(std::ptr::null(), |src| ref_as_pcvoid(src)),
							dest.as_ref().map_or(std::ptr::null(), |dest| ref_as_pcvoid(dest)),
						)
					},
				)
			}

			/// [`IMFVideoDisplayControl::SetVideoWindow`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setvideowindow)
			/// method.
			pub fn SetVideoWindow(&self, hwndVideo: HWND) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IMFVideoDisplayControlVT>() };
				hr_to_winresult(
					unsafe { ((**ppvt).SetVideoWindow)(ppvt, hwndVideo.ptr) },
				)
			}
		}
	};
}

IMFVideoDisplayControl_impl! {
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
	IMFVideoDisplayControl, IMFVideoDisplayControlVT
}
