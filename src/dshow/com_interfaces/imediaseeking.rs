#![allow(non_snake_case)]

use crate::co;
use crate::ffi_types::{HRES, PCVOID, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::OleIUnknown;
use crate::vt::IUnknownVT;

/// [`IMediaSeeking`](crate::IMediaSeeking) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
#[repr(C)]
pub struct IMediaSeekingVT {
	pub IUnknownVT: IUnknownVT,
	pub GetCapabilities: fn(ComPtr, *mut u32) -> HRES,
	pub CheckCapabilities: fn(ComPtr, *mut u32) -> HRES,
	pub IsFormatSupported: fn(ComPtr, PCVOID) -> HRES,
	pub QueryPreferredFormat: fn(ComPtr, PVOID) -> HRES,
	pub GetTimeFormat: fn(ComPtr, PVOID) -> HRES,
	pub IsUsingTimeFormat: fn(ComPtr, PCVOID) -> HRES,
	pub SetTimeFormat: fn(ComPtr, PCVOID) -> HRES,
	pub GetDuration: fn(ComPtr, *mut i64) -> HRES,
	pub GetStopPosition: fn(ComPtr, *mut i64) -> HRES,
	pub GetCurrentPosition: fn(ComPtr, *mut i64) -> HRES,
	pub ConvertTimeFormat: fn(ComPtr, *mut i64, PCVOID, i64, PCVOID) -> HRES,
	pub SetPositions: fn(ComPtr, *mut i64, u32, *mut i64, u32) -> HRES,
	pub GetPositions: fn(ComPtr, *mut i64, *mut i64) -> HRES,
	pub GetAvailable: fn(ComPtr, *mut i64, *mut i64) -> HRES,
	pub SetRate: fn(ComPtr, f64) -> HRES,
	pub GetRate: fn(ComPtr, *mut f64) -> HRES,
	pub GetPreroll: fn(ComPtr, *mut i64) -> HRES,
}

/// [`IMediaSeeking`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediaseeking)
/// COM interface over [`IMediaSeekingVT`](crate::vt::IMediaSeekingVT). Inherits
/// from [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{IGraphBuilder, IMediaSeeking};
///
/// let graph_builder: IGraphBuilder; // initialized somewhere
/// # use winsafe::{co::CLSID, co::CLSCTX, CoCreateInstance};
/// # let graph_builder = CoCreateInstance::<IGraphBuilder>(&CLSID::new(0,0,0,0,0), None, CLSCTX::INPROC_SERVER)?;
///
/// let media_seeking = graph_builder
///     .QueryInterface::<IMediaSeeking>()?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub struct IMediaSeeking(ComPtr);

impl_iunknown!(IMediaSeeking, 0x36b73880, 0xc2c8, 0x11cf, 0x8b46, 0x00805f6cef60);
impl DshowIMediaSeeking for IMediaSeeking {}

/// [`IMediaSeeking`](crate::IMediaSeeking) methods methods from `dshow`
/// feature.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait DshowIMediaSeeking: OleIUnknown {
	/// [`IMediaSeeking::ConvertTimeFormat`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-converttimeformat)
	/// method.
	fn ConvertTimeFormat(&self,
		target_format: &co::TIME_FORMAT,
		source: i64, source_format: &co::TIME_FORMAT) -> HrResult<i64>
	{
		let mut target = i64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			ok_to_hrresult(
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
	fn GetAvailable(&self) -> HrResult<(i64, i64)> {
		let (mut early, mut late) = (i64::default(), i64::default());
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			ok_to_hrresult((vt.GetPositions)(self.ptr(), &mut early, &mut late))
		}.map(|_| (early, late))
	}

	/// [`IMediaSeeking::GetCurrentPosition method`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getcurrentposition)
	/// method.
	fn GetCurrentPosition(&self) -> HrResult<i64> {
		let mut pos = i64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			ok_to_hrresult((vt.GetCurrentPosition)(self.ptr(), &mut pos))
		}.map(|_| pos)
	}

	/// [`IMediaSeeking::GetDuration`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getduration)
	/// method.
	fn GetDuration(&self) -> HrResult<i64> {
		let mut duration = i64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			ok_to_hrresult((vt.GetDuration)(self.ptr(), &mut duration))
		}.map(|_| duration)
	}

	/// [`IMediaSeeking::GetPositions`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getpositions)
	/// method.
	///
	/// Returns current and stop positions.
	fn GetPositions(&self) -> HrResult<(i64, i64)> {
		let (mut current, mut stop) = (i64::default(), i64::default());
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			ok_to_hrresult((vt.GetPositions)(self.ptr(), &mut current, &mut stop))
		}.map(|_| (current, stop))
	}

	/// [`IMediaSeeking::GetPreroll`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getpreroll)
	/// method.
	fn GetPreroll(&self) -> HrResult<i64> {
		let mut preroll = i64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			ok_to_hrresult((vt.GetPreroll)(self.ptr(), &mut preroll))
		}.map(|_| preroll)
	}

	/// [`IMediaSeeking::GetRate`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getrate)
	/// method.
	fn GetRate(&self) -> HrResult<f64> {
		let mut rate = f64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			ok_to_hrresult((vt.GetRate)(self.ptr(), &mut rate))
		}.map(|_| rate)
	}

	/// [`IMediaSeeking::GetStopPosition`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getstopposition)
	/// method.
	fn GetStopPosition(&self) -> HrResult<i64> {
		let mut pos = i64::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			ok_to_hrresult((vt.GetStopPosition)(self.ptr(), &mut pos))
		}.map(|_| pos)
	}

	/// [`IMediaSeeking::GetTimeFormat`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-gettimeformat)
	/// method.
	fn GetTimeFormat(&self) -> HrResult<co::TIME_FORMAT> {
		let mut time_guid = co::TIME_FORMAT::NONE;
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			ok_to_hrresult(
				(vt.GetStopPosition)(self.ptr(), &mut time_guid as *mut _ as _),
			)
		}.map(|_| time_guid)
	}

	/// [`IMediaSeeking::SetPositions`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-setpositions)
	/// method.
	fn SetPositions(&self,
		current: i64, current_flags: co::SEEKING_FLAGS,
		stop: i64, stop_flags: co::SEEKING_FLAGS) -> HrResult<()>
	{
		let (mut current, mut stop) = (current, stop);
		match co::HRESULT(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
				(vt.SetPositions)(
					self.ptr(),
					&mut current,
					current_flags.0,
					&mut stop,
					stop_flags.0,
				) as _
			},
		) {
			co::HRESULT::S_OK
			| co::HRESULT::S_FALSE => Ok(()),
			hr => Err(hr),
		}
	}

	/// [`IMediaSeeking::SetRate`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-setrate)
	/// method.
	fn SetRate(&self, rate: f64) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			ok_to_hrresult((vt.SetRate)(self.ptr(), rate))
		}
	}

	/// [`IMediaSeeking::SetTimeFormat`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-settimeformat)
	/// method.
	fn SetTimeFormat(&self, format: &co::TIME_FORMAT) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaSeekingVT);
			ok_to_hrresult(
				(vt.SetTimeFormat)(self.ptr(), format as *const _ as _),
			)
		}
	}
}
