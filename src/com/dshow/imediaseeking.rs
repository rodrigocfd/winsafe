#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::dshow;
use crate::com::dshow::guid;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{HRESULT, PCVOID, PVOID};
use crate::privs::hr_to_winresult;
use crate::structs::GUID;

/// [`IMediaSeeking`](crate::dshow::IMediaSeeking) virtual table.
#[repr(C)]
pub struct IMediaSeekingVT {
	pub IUnknownVT: IUnknownVT,
	pub GetCapabilities: fn(ComPtr, *mut u32) -> HRESULT,
	pub CheckCapabilities: fn(ComPtr, *mut u32) -> HRESULT,
	pub IsFormatSupported: fn(ComPtr, PCVOID) -> HRESULT,
	pub QueryPreferredFormat: fn(ComPtr, PVOID) -> HRESULT,
	pub GetTimeFormat: fn(ComPtr, PVOID) -> HRESULT,
	pub IsUsingTimeFormat: fn(ComPtr, PCVOID) -> HRESULT,
	pub SetTimeFormat: fn(ComPtr, PCVOID) -> HRESULT,
	pub GetDuration: fn(ComPtr, *mut i64) -> HRESULT,
	pub GetStopPosition: fn(ComPtr, *mut i64) -> HRESULT,
	pub GetCurrentPosition: fn(ComPtr, *mut i64) -> HRESULT,
	pub ConvertTimeFormat: fn(ComPtr, *mut i64, PCVOID, i64, PCVOID) -> HRESULT,
	pub SetPositions: fn(ComPtr, *mut i64, u32, *mut i64, u32) -> HRESULT,
	pub GetPositions: fn(ComPtr, *mut i64, *mut i64) -> HRESULT,
	pub GetAvailable: fn(ComPtr, *mut i64, *mut i64) -> HRESULT,
	pub SetRate: fn(ComPtr, f64) -> HRESULT,
	pub GetRate: fn(ComPtr, *mut f64) -> HRESULT,
	pub GetPreroll: fn(ComPtr, *mut i64) -> HRESULT,
}

/// [`IMediaSeeking`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediaseeking)
/// COM interface over
/// [`IMediaSeekingVT`](crate::dshow::vt::IMediaSeekingVT). Inherits from
/// [`IUnknown`](crate::IUnknown).
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
/// let graph_builder: dshow::IGraphBuilder; // initialized somewhere
///
/// let media_seeking = graph_builder
///     .QueryInterface::<dshow::IMediaSeeking>()?;
/// ```
pub struct IMediaSeeking(ComPtr);

impl_iunknown!(IMediaSeeking, 0x36b73880, 0xc2c8, 0x11cf, 0x8b46, 0x00805f6cef60);
impl IMediaSeekingT for IMediaSeeking {}

/// Exposes the [`IMediaSeeking`](crate::dshow::IMediaSeeking) methods.
pub trait IMediaSeekingT: IUnknownT {
	/// [`IMediaSeeking::ConvertTimeFormat`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-converttimeformat)
	/// method.
	fn ConvertTimeFormat(&self,
		target_format: &GUID,
		source: i64, source_format: &GUID) -> WinResult<i64>
	{
		let mut target = i64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			hr_to_winresult(
				(vt.ConvertTimeFormat)(
					self.ptr(),
					&mut target,
					target_format as *const _ as _,
					source,
					source_format as *const _ as _,
				),
			)
		}.map(|_| target)
	}

	/// [`IMediaSeeking::GetAvailable`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getavailable)
	/// method.
	///
	/// Returns earliest and latest times for efficient seeking.
	fn GetAvailable(&self) -> WinResult<(i64, i64)> {
		let (mut early, mut late) = (i64::default(), i64::default());
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			hr_to_winresult((vt.GetPositions)(self.ptr(), &mut early, &mut late))
		}.map(|_| (early, late))
	}

	/// [`IMediaSeeking::GetCurrentPosition method`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getcurrentposition)
	/// method.
	fn GetCurrentPosition(&self) -> WinResult<i64> {
		let mut pos = i64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			hr_to_winresult((vt.GetCurrentPosition)(self.ptr(), &mut pos))
		}.map(|_| pos)
	}

	/// [`IMediaSeeking::GetDuration`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getduration)
	/// method.
	fn GetDuration(&self) -> WinResult<i64> {
		let mut duration = i64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			hr_to_winresult((vt.GetDuration)(self.ptr(), &mut duration))
		}.map(|_| duration)
	}

	/// [`IMediaSeeking::GetPositions`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getpositions)
	/// method.
	///
	/// Returns current and stop positions.
	fn GetPositions(&self) -> WinResult<(i64, i64)> {
		let (mut current, mut stop) = (i64::default(), i64::default());
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			hr_to_winresult((vt.GetPositions)(self.ptr(), &mut current, &mut stop))
		}.map(|_| (current, stop))
	}

	/// [`IMediaSeeking::GetPreroll`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getpreroll)
	/// method.
	fn GetPreroll(&self) -> WinResult<i64> {
		let mut preroll = i64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			hr_to_winresult((vt.GetPreroll)(self.ptr(), &mut preroll))
		}.map(|_| preroll)
	}

	/// [`IMediaSeeking::GetRate`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getrate)
	/// method.
	fn GetRate(&self) -> WinResult<f64> {
		let mut rate = f64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			hr_to_winresult((vt.GetRate)(self.ptr(), &mut rate))
		}.map(|_| rate)
	}

	/// [`IMediaSeeking::GetStopPosition`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getstopposition)
	/// method.
	fn GetStopPosition(&self) -> WinResult<i64> {
		let mut pos = i64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			hr_to_winresult((vt.GetStopPosition)(self.ptr(), &mut pos))
		}.map(|_| pos)
	}

	/// [`IMediaSeeking::GetTimeFormat`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-gettimeformat)
	/// method.
	fn GetTimeFormat(&self) -> WinResult<GUID> {
		let mut time_guid = guid::TIME_FORMAT_NONE;
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			hr_to_winresult(
				(vt.GetStopPosition)(self.ptr(), &mut time_guid as *mut _ as _),
			)
		}.map(|_| time_guid)
	}

	/// [`IMediaSeeking::SetPositions`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-setpositions)
	/// method.
	fn SetPositions(&self,
		mut current: i64, current_flags: dshow::co::SEEKING_FLAGS,
		mut stop: i64, stop_flags: dshow::co::SEEKING_FLAGS) -> WinResult<()>
	{
		match co::ERROR(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
				(vt.SetPositions)(
					self.ptr(),
					&mut current,
					current_flags.0,
					&mut stop,
					stop_flags.0,
				) as _
			}
		) {
			co::ERROR::S_OK | co::ERROR::S_FALSE => Ok(()),
			err => Err(err),
		}
	}

	/// [`IMediaSeeking::SetRate`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-setrate)
	/// method.
	fn SetRate(&self, rate: f64) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			hr_to_winresult((vt.SetRate)(self.ptr(), rate))
		}
	}

	/// [`IMediaSeeking::SetTimeFormat`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-settimeformat)
	/// method.
	fn SetTimeFormat(&self, format: &GUID) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			hr_to_winresult(
				(vt.SetTimeFormat)(self.ptr(), format as *const _ as _),
			)
		}
	}
}
