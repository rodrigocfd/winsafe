#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::{HRESULT, PCVOID, PVOID};
use crate::structs::IID;

/// [`IMediaSeeking`](crate::dshow::IMediaSeeking) virtual table.
pub struct IMediaSeekingVT {
	pub IUnknownVT: IUnknownVT,
	pub GetCapabilities: fn(PPVT, *mut u32) -> HRESULT,
	pub CheckCapabilities: fn(PPVT, *mut u32) -> HRESULT,
	pub IsFormatSupported: fn(PPVT, PCVOID) -> HRESULT,
	pub QueryPreferredFormat: fn(PPVT, PVOID) -> HRESULT,
	pub GetTimeFormat: fn(PPVT, PVOID) -> HRESULT,
	pub IsUsingTimeFormat: fn(PPVT, PCVOID) -> HRESULT,
	pub SetTimeFormat: fn(PPVT, PCVOID) -> HRESULT,
	pub GetDuration: fn(PPVT, *mut i64) -> HRESULT,
	pub GetStopPosition: fn(PPVT, *mut i64) -> HRESULT,
	pub GetCurrentPosition: fn(PPVT, *mut i64) -> HRESULT,
	pub ConvertTimeFormat: fn(PPVT, *mut i64, PCVOID, i64, PCVOID) -> HRESULT,
	pub SetPositions: fn(PPVT, *mut i64, u32, *mut i64, u32) -> HRESULT,
	pub GetPositions: fn(PPVT, *mut i64, *mut i64) -> HRESULT,
	pub GetAvailable: fn(PPVT, *mut i64, *mut i64) -> HRESULT,
	pub SetRate: fn(PPVT, f64) -> HRESULT,
	pub GetRate: fn(PPVT, *mut f64) -> HRESULT,
	pub GetPreroll: fn(PPVT, *mut i64) -> HRESULT,
}

/// [`IMediaSeeking`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediaseeking)
/// COM interface over
/// [`IMediaSeekingVT`](crate::dshow::vt::IMediaSeekingVT). Inherits from
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IMediaSeeking {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for IMediaSeeking {
	const IID: IID = IID::new(0x36b73880, 0xc2c8, 0x11cf, 0x8b46, 0x00805f6cef60);
}

macro_rules! impl_IMediaSeeking {
	($name:ty, $vt:ty) => {
		use crate::co;
		use crate::com::dshow::co as dshowco;
		use crate::com::dshow::guid;
		use crate::structs::GUID;

		impl $name {
			fn imediaseeking_vt(&self) -> &IMediaSeekingVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`IMediaSeeking::ConvertTimeFormat`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-converttimeformat)
			/// method.
			pub fn ConvertTimeFormat(&self,
				targetFormat: &GUID, source: i64, sourceFormat: &GUID) -> WinResult<i64>
			{
				let mut target: i64 = 0;
				hr_to_winresult(
					(self.imediaseeking_vt().ConvertTimeFormat)(
						self.ppvt,
						&mut target,
						targetFormat as *const _ as _,
						source,
						sourceFormat as *const _ as _,
					),
				).map(|_| target)
			}

			/// [`IMediaSeeking::GetAvailable`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getavailable)
			/// method.
			///
			/// Returns earliest and latest times for efficient seeking.
			pub fn GetAvailable(&self) -> WinResult<(i64, i64)> {
				let mut early: i64 = 0;
				let mut late: i64 = 0;
				hr_to_winresult(
					(self.imediaseeking_vt().GetPositions)(
						self.ppvt,
						&mut early,
						&mut late,
					),
				).map(|_| (early, late))
			}

			/// [`IMediaSeeking::GetCurrentPosition method`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getcurrentposition)
			/// method.
			pub fn GetCurrentPosition(&self) -> WinResult<i64> {
				let mut pos: i64 = 0;
				hr_to_winresult(
					(self.imediaseeking_vt().GetCurrentPosition)(
						self.ppvt,
						&mut pos,
					),
				).map(|_| pos)
			}

			/// [`IMediaSeeking::GetDuration`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getduration)
			/// method.
			pub fn GetDuration(&self) -> WinResult<i64> {
				let mut duration: i64 = 0;
				hr_to_winresult(
					(self.imediaseeking_vt().GetDuration)(self.ppvt, &mut duration),
				).map(|_| duration)
			}

			/// [`IMediaSeeking::GetPositions`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getpositions)
			/// method.
			///
			/// Returns current and stop positions.
			pub fn GetPositions(&self) -> WinResult<(i64, i64)> {
				let mut current: i64 = 0;
				let mut stop: i64 = 0;
				hr_to_winresult(
					(self.imediaseeking_vt().GetPositions)(
						self.ppvt,
						&mut current,
						&mut stop,
					),
				).map(|_| (current, stop))
			}

			/// [`IMediaSeeking::GetPreroll`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getpreroll)
			/// method.
			pub fn GetPreroll(&self) -> WinResult<i64> {
				let mut preroll: i64 = 0;
				hr_to_winresult(
					(self.imediaseeking_vt().GetPreroll)(self.ppvt, &mut preroll),
				).map(|_| preroll)
			}

			/// [`IMediaSeeking::GetRate`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getrate)
			/// method.
			pub fn GetRate(&self) -> WinResult<f64> {
				let mut rate: f64 = 0.0;
				hr_to_winresult(
					(self.imediaseeking_vt().GetRate)(self.ppvt, &mut rate),
				).map(|_| rate)
			}

			/// [`IMediaSeeking::GetStopPosition`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getstopposition)
			/// method.
			pub fn GetStopPosition(&self) -> WinResult<i64> {
				let mut pos: i64 = 0;
				hr_to_winresult(
					(self.imediaseeking_vt().GetStopPosition)(self.ppvt, &mut pos),
				).map(|_| pos)
			}

			/// [`IMediaSeeking::GetTimeFormat`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-gettimeformat)
			/// method.
			pub fn GetTimeFormat(&self) -> WinResult<GUID> {
				let mut timeGuid = guid::TIME_FORMAT_NONE;
				hr_to_winresult(
					(self.imediaseeking_vt().GetStopPosition)(
						self.ppvt,
						&mut timeGuid as *mut _ as _,
					),
				).map(|_| timeGuid)
			}

			/// [`IMediaSeeking::SetPositions`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-setpositions)
			/// method.
			pub fn SetPositions(&self,
				mut current: i64, currentFlags: dshowco::SEEKING_FLAGS,
				mut stop: i64, stopFlags: dshowco::SEEKING_FLAGS) -> WinResult<()>
			{
				match co::ERROR(
					(self.imediaseeking_vt().SetPositions)(
						self.ppvt,
						&mut current,
						currentFlags.0,
						&mut stop,
						stopFlags.0,
					) as _,
				) {
					co::ERROR::S_OK | co::ERROR::S_FALSE => Ok(()),
					err => Err(err),
				}
			}

			/// [`IMediaSeeking::SetRate`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-setrate)
			/// method.
			pub fn SetRate(&self, rate: f64) -> WinResult<()> {
				hr_to_winresult(
					(self.imediaseeking_vt().SetRate)(self.ppvt, rate),
				)
			}

			/// [`IMediaSeeking::SetTimeFormat`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-settimeformat)
			/// method.
			pub fn SetTimeFormat(&self, format: &GUID) -> WinResult<()> {
				hr_to_winresult(
					(self.imediaseeking_vt().SetTimeFormat)(
						self.ppvt,
						format as *const _ as _,
					),
				)
			}
		}
	};
}

impl_IUnknown!(IMediaSeeking, IMediaSeekingVT);
impl_IMediaSeeking!(IMediaSeeking, IMediaSeekingVT);
