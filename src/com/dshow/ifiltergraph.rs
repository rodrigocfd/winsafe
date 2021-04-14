#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknown, IUnknownVT, PPComVT};
use crate::com::dshow::IBaseFilter;
use crate::com::dshow::vt::IFilterGraphVT;
use crate::com::funcs::hr_to_winresult;
use crate::WString;

/// [`IFilterGraph`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist)
/// COM interface.
///
/// Virtual table: [`IFilterGraphVT`](crate::dshow::vt::IFilterGraphVT).
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IFilterGraph {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<IFilterGraphVT>> for IFilterGraph {
	fn from(ppv: PPComVT<IFilterGraphVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl IFilterGraph {
	unsafe fn ppv(&self) -> PPComVT<IFilterGraphVT> {
		self.IUnknown.ppv::<IFilterGraphVT>()
	}

	/// [`IFilterGraph::AddFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-addfilter)
	/// method.
	pub fn AddFilter(&self, filter: &IBaseFilter, name: &str) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).AddFilter)(
					self.ppv(),
					filter.IMediaFilter.IPersist.IUnknown.ppv(),
					WString::from_str(name).as_ptr(),
				)
			},
		)
	}

	/// [`IFilterGraph::RemoveFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-removefilter)
	/// method.
	pub fn RemoveFilter(&self, filter: &IBaseFilter) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).RemoveFilter)(
					self.ppv(),
					filter.IMediaFilter.IPersist.IUnknown.ppv(),
				)
			},
		)
	}

	/// [`IFilterGraph::SetDefaultSyncSource`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-setdefaultsyncsource)
	/// method.
	pub fn SetDefaultSyncSource(&self) -> WinResult<()> {
		hr_to_winresult(
			unsafe { ((**self.ppv()).SetDefaultSyncSource)(self.ppv()) },
		)
	}
}
