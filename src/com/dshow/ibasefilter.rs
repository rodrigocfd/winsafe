#![allow(non_snake_case)]

use crate::com::dshow::ifiltergraph::IFilterGraph;
use crate::com::dshow::imediafilter::IMediaFilterVT;
use crate::com::ipersist::IPersistVT;
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::{HRESULT, PCSTR, PSTR, PVOID};
use crate::structs::IID;

/// [`IBaseFilter`](crate::dshow::IBaseFilter) virtual table.
pub struct IBaseFilterVT {
	pub IMediaFilterVT: IMediaFilterVT,
	pub EnumPins: fn(PPVT, *mut PPVT) -> HRESULT,
	pub FindPin: fn(PPVT, PCSTR, *mut PPVT) -> HRESULT,
	pub QueryFilterInfo: fn(PPVT, PVOID) -> HRESULT,
	pub JoinFilterGraph: fn(PPVT, PPVT, PCSTR) -> HRESULT,
	pub QueryVendorInfo: fn(PPVT, *mut PSTR) -> HRESULT,
}

/// [`IBaseFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ibasefilter)
/// COM interface over [`IBaseFilterVT`](crate::dshow::vt::IBaseFilterVT).
/// Inherits from [`IMediaFilter`](crate::dshow::IMediaFilter),
/// [`IPersist`](crate::IPersist),
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IBaseFilter {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for IBaseFilter {
	const IID: IID = IID::new(0x56a86895, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
}

macro_rules! impl_IBaseFilter {
	($name:ty, $vt:ty) => {
		use crate::com::funcs::CoTaskMemFree;
		use crate::various::WString;

		impl $name {
			fn ibasefilter_vt(&self) -> &IBaseFilterVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
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

impl_IUnknown!(IBaseFilter, IBaseFilterVT);
impl_IPersist!(IBaseFilter, IBaseFilterVT);
impl_IMediaFilter!(IBaseFilter, IBaseFilterVT);
impl_IBaseFilter!(IBaseFilter, IBaseFilterVT);
