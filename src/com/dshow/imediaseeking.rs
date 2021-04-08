#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknown, IUnknownVT, PPComVT};
use crate::com::dshow::vt::IMediaSeekingVT;
use crate::com::funcs::hr_to_winresult;

/// [`IMediaSeeking`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediaseeking)
/// COM interface. Backed by [`IMediaSeekingVT`](crate::dshow::IMediaSeekingVT)
/// virtual table.
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IMediaSeeking {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<IMediaSeekingVT>> for IMediaSeeking {
	fn from(ppv: PPComVT<IMediaSeekingVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl IMediaSeeking {
	unsafe fn ppv(&self) -> PPComVT<IMediaSeekingVT> {
		self.IUnknown.ppv::<IMediaSeekingVT>()
	}

	/// [`IMediaSeeking::GetDuration`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getduration)
	/// method.
	pub fn GetDuration(&self) -> WinResult<i64> {
		let mut duration: i64 = 0;
		hr_to_winresult(
			unsafe { ((**self.ppv()).GetDuration)(self.ppv(), &mut duration) },
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
			unsafe {
				((**self.ppv()).GetPositions)(self.ppv(), &mut current, &mut stop)
			},
		).map(|_| (current, stop))
	}

	/// [`IMediaSeeking::GetPreroll`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getpreroll)
	/// method.
	pub fn GetPreroll(&self) -> WinResult<i64> {
		let mut preroll: i64 = 0;
		hr_to_winresult(
			unsafe { ((**self.ppv()).GetPreroll)(self.ppv(), &mut preroll) },
		).map(|_| preroll)
	}

	/// [`IMediaSeeking::GetRate`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getrate)
	/// method.
	pub fn GetRate(&self) -> WinResult<f64> {
		let mut rate: f64 = 0.0;
		hr_to_winresult(
			unsafe { ((**self.ppv()).GetRate)(self.ppv(), &mut rate) },
		).map(|_| rate)
	}

	/// [`IMediaSeeking::GetStopPosition`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getstopposition)
	/// method.
	pub fn GetStopPosition(&self) -> WinResult<i64> {
		let mut pos: i64 = 0;
		hr_to_winresult(
			unsafe { ((**self.ppv()).GetStopPosition)(self.ppv(), &mut pos) },
		).map(|_| pos)
	}
}
