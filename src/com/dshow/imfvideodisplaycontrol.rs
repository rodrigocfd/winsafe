#![allow(non_snake_case)]

macro_rules! IMFVideoDisplayControl_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::co;
		use crate::com::dshow::MFVideoNormalizedRect;
		use crate::com::dshow::vt::IMFVideoDisplayControlVT;
		use crate::com::funcs::CoTaskMemFree;
		use crate::handles::HWND;
		use crate::privs::ref_as_pvoid;
		use crate::structs::{BITMAPINFOHEADER, COLORREF, RECT, SIZE};

		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn imfvideodisplaycontrol_vt(&self) -> &IMFVideoDisplayControlVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IMFVideoDisplayControl::GetAspectRatioMode`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getaspectratiomode)
			/// method.
			pub fn GetAspectRatioMode(&self) -> WinResult<co::MFVideoARMode> {
				let mut mode = co::MFVideoARMode::None;
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().GetAspectRatioMode)(
						self.ppvt,
						&mut mode as *mut _ as _,
					),
				).map(|_| mode)
			}

			/// [`IMFVideoDisplayControl::GetBorderColor`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getbordercolor)
			/// method;
			pub fn GetBorderColor(&self) -> WinResult<COLORREF> {
				let mut color = COLORREF::new(0, 0, 0);
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().GetBorderColor)(
						self.ppvt,
						&mut color.0,
					),
				).map(|_| color)
			}

			/// [`GetCurrentImage`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getcurrentimage)
			/// method.
			///
			/// Returns bitmap description, DIB bytes and time stamp.
			pub fn GetCurrentImage(&self)
				-> WinResult<(BITMAPINFOHEADER, Vec<u8>, i64)>
			{
				let mut bih = BITMAPINFOHEADER::default();
				let mut pDib: *mut u8 = std::ptr::null_mut();
				let mut cbDib: u32 = 0;
				let mut timeStamp: i64 = 0;
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().GetCurrentImage)(
						self.ppvt,
						ref_as_pvoid(&mut bih),
						&mut pDib,
						&mut cbDib,
						&mut timeStamp,
					),
				).map(|_| {
					let vecDib = unsafe { std::slice::from_raw_parts(pDib, cbDib as _) }.to_vec();
					CoTaskMemFree(pDib);
					(bih, vecDib, timeStamp)
				})
			}

			/// [`IMFVideoDisplayControl::GetFullscreen`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getfullscreen)
			/// method.
			pub fn GetFullscreen(&self) -> WinResult<bool> {
				let mut fulls = false;
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().GetFullscreen)(
						self.ppvt,
						&mut fulls as *mut _ as _,
					),
				).map(|_| fulls)
			}

			/// [`IMFVideoDisplayControl::GetIdealVideoSize`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getidealvideosize)
			/// method.
			///
			/// Returns minimum and maximum ideal sizes.
			pub fn GetIdealVideoSize(&self) -> WinResult<(SIZE, SIZE)> {
				let mut min = SIZE::default();
				let mut max = SIZE::default();
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().GetIdealVideoSize)(
						self.ppvt,
						ref_as_pvoid(&mut min),
						ref_as_pvoid(&mut max),
					),
				).map(|_| (min, max))
			}

			/// [`IMFVideoDisplayControl::GetNativeVideoSize`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getnativevideosize)
			/// method.
			///
			/// Returns native and aspect ratio sizes.
			pub fn GetNativeVideoSize(&self) -> WinResult<(SIZE, SIZE)> {
				let mut native = SIZE::default();
				let mut aspec = SIZE::default();
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().GetNativeVideoSize)(
						self.ppvt,
						ref_as_pvoid(&mut native),
						ref_as_pvoid(&mut aspec),
					),
				).map(|_| (native, aspec))
			}

			/// [`IMFVideoDisplayControl::GetVideoPosition`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getvideoposition)
			/// method.
			pub fn GetVideoPosition(&self)
				-> WinResult<(MFVideoNormalizedRect, RECT)>
			{
				let mut pnrc = MFVideoNormalizedRect::default();
				let mut rc = RECT::default();
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().GetVideoPosition)(
						self.ppvt,
						ref_as_pvoid(&mut pnrc),
						ref_as_pvoid(&mut rc),
					),
				).map(|_| (pnrc, rc))
			}

			/// [`IMFVideoDisplayControl::GetVideoWindow`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getvideowindow)
			/// method.
			pub fn GetVideoWindow(&self) -> WinResult<HWND> {
				let mut hwnd = unsafe { HWND::null_handle() };
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().GetVideoWindow)(
						self.ppvt,
						&mut hwnd.ptr,
					),
				).map(|_| hwnd)
			}

			/// [`IMFVideoDisplayControl::RepaintVideo`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-repaintvideo)
			/// method.
			pub fn RepaintVideo(&self) -> WinResult<()> {
				match co::ERROR(
					(self.imfvideodisplaycontrol_vt().RepaintVideo)(self.ppvt) as _,
				) {
					co::ERROR::S_OK | co::ERROR::MF_E_INVALIDREQUEST => Ok(()),
					err => Err(err),
				}
			}

			/// [`IMFVideoDisplayControl::SetAspectRatioMode`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setaspectratiomode)
			/// method.
			pub fn SetAspectRatioMode(&self, mode: co::MFVideoARMode) -> WinResult<()> {
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().SetAspectRatioMode)(
						self.ppvt,
						mode.0,
					),
				)
			}

			/// [`IMFVideoDisplayControl::SetBorderColor`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setbordercolor)
			/// method.
			pub fn SetBorderColor(&self, color: COLORREF) -> WinResult<()> {
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().SetBorderColor)(
						self.ppvt,
						color.0,
					),
				)
			}

			/// [`IMFVideoDisplayControl::SetFullscreen`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setfullscreen)
			/// method.
			pub fn SetFullscreen(&self, fullScreen: bool) -> WinResult<()> {
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().SetFullscreen)(
						self.ppvt,
						fullScreen as _,
					),
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
					(self.imfvideodisplaycontrol_vt().SetVideoPosition)(
						self.ppvt,
						src.as_ref().map_or(std::ptr::null(), |src| ref_as_pcvoid(src)),
						dest.as_ref().map_or(std::ptr::null(), |dest| ref_as_pcvoid(dest)),
					),
				)
			}

			/// [`IMFVideoDisplayControl::SetVideoWindow`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-setvideowindow)
			/// method.
			pub fn SetVideoWindow(&self, hwndVideo: HWND) -> WinResult<()> {
				hr_to_winresult(
					(self.imfvideodisplaycontrol_vt().SetVideoWindow)(
						self.ppvt,
						hwndVideo.ptr,
					),
				)
			}
		}
	};
}

IMFVideoDisplayControl_impl! {
	/// [`IMFVideoDisplayControl`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nn-evr-imfvideodisplaycontrol)
	/// COM interface over
	/// [`IMFVideoDisplayControlVT`](crate::dshow::vt::IMFVideoDisplayControlVT).
	/// Inherits from [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IMFVideoDisplayControl, crate::com::dshow::vt::IMFVideoDisplayControlVT
}
