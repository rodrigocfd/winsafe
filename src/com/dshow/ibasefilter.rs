#![allow(non_snake_case)]

macro_rules! IBaseFilter_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::dshow::IFilterGraph;
		use crate::com::dshow::vt::IBaseFilterVT;
		use crate::com::funcs::CoTaskMemFree;
		use crate::WString;

		IMediaFilter_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn ibasefilter_vt(&self) -> &IBaseFilterVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IBaseFilter::JoinFilterGraph`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-joinfiltergraph)
			/// method.
			pub fn JoinFilterGraph(&self,
				graph: Option<&IFilterGraph>, name: &str) -> WinResult<()>
			{
				hr_to_winresult(
					(self.ibasefilter_vt().JoinFilterGraph)(
						self.ppvt,
						graph.map_or(std::ptr::null_mut(), |g| g.ppvt),
						unsafe { WString::from_str(name).as_ptr() },
					),
				)
			}

			/// [`IBaseFilter::QueryVendorInfo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-queryvendorinfo)
			/// method.
			pub fn QueryVendorInfo(&self) -> WinResult<String> {
				let mut pstr: *mut u16 = std::ptr::null_mut();
				hr_to_winresult(
					(self.ibasefilter_vt().QueryVendorInfo)(self.ppvt, &mut pstr),
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
	IBaseFilter, crate::com::dshow::vt::IBaseFilterVT
}
