#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::dshow::{IFilterGraph, IMediaFilter};
use crate::com::dshow::vt::{IBaseFilterVT, IMediaFilterVT};
use crate::com::funcs::CoTaskMemFree;
use crate::com::PPComVT;
use crate::privs::hr_to_winresult;
use crate::WString;

/// [`IBaseFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ibasefilter)
/// COM interface.
///
/// Virtual table: [`IBaseFilterVT`](crate::dshow::vt::IBaseFilterVT).
///
/// Inherits from:
/// * [`IMediaFilter`](crate::dshow::IMediaFilter);
/// * [`IPersist`](crate::IPersist);
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IBaseFilter {
	/// Methods of base interface [`IMediaFilter`](crate::dshow::IMediaFilter).
	pub IMediaFilter: IMediaFilter,
}

impl From<PPComVT<IBaseFilterVT>> for IBaseFilter {
	fn from(ppv: PPComVT<IBaseFilterVT>) -> Self {
		Self {
			IMediaFilter: IMediaFilter::from(ppv as PPComVT<IMediaFilterVT>)
		}
	}
}

impl IBaseFilter {
	unsafe fn ppv(&self) -> PPComVT<IBaseFilterVT> {
		self.IMediaFilter.IPersist.IUnknown.ppv::<IBaseFilterVT>()
	}

	/// [`IBaseFilter::JoinFilterGraph`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-joinfiltergraph)
	/// method.
	pub fn JoinFilterGraph(&self,
		graph: Option<&IFilterGraph>, name: &str) -> WinResult<()>
	{
		hr_to_winresult(
			unsafe {
				((**self.ppv()).JoinFilterGraph)(
					self.ppv(),
					graph.map_or(std::ptr::null_mut(), |g| g.IUnknown.ppv()),
					WString::from_str(name).as_ptr(),
				)
			},
		)
	}

	/// [`IBaseFilter::QueryVendorInfo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-queryvendorinfo)
	/// method.
	pub fn QueryVendorInfo(&self) -> WinResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		hr_to_winresult(
			unsafe { ((**self.ppv()).QueryVendorInfo)(self.ppv(), &mut pstr) },
		).map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr);
			name.to_string()
		})
	}
}
