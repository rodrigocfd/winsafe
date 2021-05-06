#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IPersistVT, IUnknownVT};
use crate::com::dshow::IFilterGraph;
use crate::com::dshow::vt::{IBaseFilterVT, IMediaFilterVT};
use crate::com::funcs::CoTaskMemFree;
use crate::com::PPComVT;
use crate::privs::{hr_to_winresult, hr_to_winresult_bool, ref_as_pvoid};
use crate::structs::CLSID;
use crate::WString;

macro_rules! IBaseFilter_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IMediaFilter_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IBaseFilter::JoinFilterGraph`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-joinfiltergraph)
			/// method.
			pub fn JoinFilterGraph(&self,
				graph: Option<&IFilterGraph>, name: &str) -> WinResult<()>
			{
				let ppvt = unsafe { self.ppvt::<IBaseFilterVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).JoinFilterGraph)(
							ppvt,
							graph.map_or(std::ptr::null_mut(), |g| g.ppvt()),
							WString::from_str(name).as_ptr(),
						)
					},
				)
			}

			/// [`IBaseFilter::QueryVendorInfo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-queryvendorinfo)
			/// method.
			pub fn QueryVendorInfo(&self) -> WinResult<String> {
				let mut pstr: *mut u16 = std::ptr::null_mut();
				let ppvt = unsafe { self.ppvt::<IBaseFilterVT>() };
				hr_to_winresult(
					unsafe { ((**ppvt).QueryVendorInfo)(ppvt, &mut pstr) },
				).map(|_| {
					let name = WString::from_wchars_nullt(pstr);
					CoTaskMemFree(pstr);
					name.to_string()
				})
			}
		}
	};
}

IBaseFilter_impl! {
	/// [`IBaseFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ibasefilter)
	/// COM interface over [`IBaseFilterVT`](crate::dshow::vt::IBaseFilterVT).
	/// Inherits from [`IMediaFilter`](crate::dshow::IMediaFilter),
	/// [`IPersist`](crate::IPersist), [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IBaseFilter, IBaseFilterVT
}
