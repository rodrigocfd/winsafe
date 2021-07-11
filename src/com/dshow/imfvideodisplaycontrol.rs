#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPI};
use crate::ffi::{BOOL, HANDLE, HRESULT, PCVOID, PVOID};
use crate::structs::IID;

/// [`IMFVideoDisplayControl`](crate::dshow::IMFVideoDisplayControl) virtual
/// table.
pub struct IMFVideoDisplayControlVT {
	pub IUnknownVT: IUnknownVT,
	pub GetNativeVideoSize: fn(PPI, PVOID, PVOID) -> HRESULT,
	pub GetIdealVideoSize: fn(PPI, PVOID, PVOID) -> HRESULT,
	pub SetVideoPosition: fn(PPI, PCVOID, PCVOID) -> HRESULT,
	pub GetVideoPosition: fn(PPI, PVOID, PCVOID) -> HRESULT,
	pub SetAspectRatioMode: fn(PPI, u32) -> HRESULT,
	pub GetAspectRatioMode: fn(PPI, *mut u32) -> HRESULT,
	pub SetVideoWindow: fn(PPI, HANDLE) -> HRESULT,
	pub GetVideoWindow: fn(PPI, *mut HANDLE) -> HRESULT,
	pub RepaintVideo: fn(PPI) -> HRESULT,
	pub GetCurrentImage: fn(PPI, PVOID, *mut *mut u8, *mut u32, *mut i64) -> HRESULT,
	pub SetBorderColor: fn(PPI, u32) -> HRESULT,
	pub GetBorderColor: fn(PPI, *mut u32) -> HRESULT,
	pub SetRenderingPrefs: fn(PPI, u32) -> HRESULT,
	pub GetRenderingPrefs: fn(PPI, *mut u32) -> HRESULT,
	pub SetFullscreen: fn(PPI, BOOL) -> HRESULT,
	pub GetFullscreen: fn(PPI, *mut BOOL) -> HRESULT,
}

/// [`IMFVideoDisplayControl`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nn-evr-imfvideodisplaycontrol)
/// COM interface over
/// [`IMFVideoDisplayControlVT`](crate::dshow::vt::IMFVideoDisplayControlVT).
/// Inherits from [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IMFVideoDisplayControl {
	pub(crate) ppvt: PPI,
}

impl_send_sync_fromppvt!(IMFVideoDisplayControl);

impl ComInterface for IMFVideoDisplayControl {
	const IID: IID = IID::new(0xa490b1e4, 0xab84, 0x4d31, 0xa1b2, 0x181e03b1077a);
}

macro_rules! impl_IMFVideoDisplayControl {
	($name:ty, $vt:ty) => {
		use crate::co;
		use crate::com::dshow::co as dshowco;
		use crate::com::dshow::MFVideoNormalizedRect;
		use crate::com::funcs::CoTaskMemFree;
		use crate::handles::HWND;
		use crate::structs::{BITMAPINFOHEADER, COLORREF, RECT, SIZE};

		impl $name {
			fn imfvideodisplaycontrol_vt(&self) -> &IMFVideoDisplayControlVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`IMFVideoDisplayControl::GetAspectRatioMode`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getaspectratiomode)
			/// method.
			pub fn GetAspectRatioMode(&self) -> WinResult<dshowco::MFVideoARMode> {
				let mut mode = dshowco::MFVideoARMode::None;
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
						&mut bih as *mut _ as _,
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
						&mut min as *mut _ as _,
						&mut max as *mut _ as _,
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
						&mut native as *mut _ as _,
						&mut aspec as *mut _ as _,
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
						&mut pnrc as *mut _ as _,
						&mut rc as *mut _ as _,
					),
				).map(|_| (pnrc, rc))
			}

			/// [`IMFVideoDisplayControl::GetVideoWindow`](https://docs.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getvideowindow)
			/// method.
			pub fn GetVideoWindow(&self) -> WinResult<HWND> {
				let mut hwnd = HWND::NULL;
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
			pub fn SetAspectRatioMode(&self,
				mode: dshowco::MFVideoARMode) -> WinResult<()>
			{
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
						src.as_ref().map_or(std::ptr::null(), |src| src as *const _ as _),
						dest.as_ref().map_or(std::ptr::null(), |dest| dest as *const _ as _),
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

impl_IUnknown!(IMFVideoDisplayControl, IMFVideoDisplayControlVT);
impl_IMFVideoDisplayControl!(IMFVideoDisplayControl, IMFVideoDisplayControlVT);
