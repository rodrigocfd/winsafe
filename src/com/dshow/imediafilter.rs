#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IPersist, IPersistVT};
use crate::com::dshow::vt::IMediaFilterVT;
use crate::com::funcs::hr_to_winresult_bool;
use crate::com::PPComVT;

/// [`IMediaFilter`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist)
/// COM interface. Backed by [`IMediaFilterVT`](crate::dshow::IMediaFilterVT)
/// virtual table.
///
/// Inherits from:
/// * [`IPersist`](crate::IPersist);
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IMediaFilter {
	/// Methods of base interface [`IPersist`](crate::IPersist).
	pub IPersist: IPersist,
}

impl From<PPComVT<IMediaFilterVT>> for IMediaFilter {
	fn from(ppv: PPComVT<IMediaFilterVT>) -> Self {
		Self {
			IPersist: IPersist::from(ppv as PPComVT<IPersistVT>)
		}
	}
}

impl IMediaFilter {
	unsafe fn ppv(&self) -> PPComVT<IMediaFilterVT> {
		self.IPersist.IUnknown.ppv::<IMediaFilterVT>()
	}

	/// [`IMediaFilter::Pause`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-pause)
	/// method.
	pub fn Pause(&self) -> WinResult<bool> {
		hr_to_winresult_bool(unsafe { ((**self.ppv()).Pause)(self.ppv()) })
	}

	/// [`IMediaFilter::Run`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-run)
	/// method.
	pub fn Run(&self, tStart: i64) -> WinResult<bool> {
		hr_to_winresult_bool(unsafe { ((**self.ppv()).Run)(self.ppv(), tStart) })
	}

	/// [`IMediaFilter::Stop`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-stop)
	/// method.
	pub fn Stop(&self) -> WinResult<bool> {
		hr_to_winresult_bool(unsafe { ((**self.ppv()).Stop)(self.ppv()) })
	}
}
