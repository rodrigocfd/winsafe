#![allow(non_snake_case)]

use crate::com::dshow::ifiltergraph::IFilterGraph;
use crate::com::dshow::imediafilter::IMediaFilterVT;
use crate::com::ipersist::IPersistVT;
use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPComVT};
use crate::ffi::{HRESULT, PCSTR, PSTR, PVOID};
use crate::structs::IID;

type PP = PPComVT<IUnknownVT>;

/// [`IBaseFilter`](crate::dshow::IBaseFilter) virtual table.
pub struct IBaseFilterVT {
	pub IMediaFilterVT: IMediaFilterVT,
	pub EnumPins: fn(PP, *mut PP) -> HRESULT,
	pub FindPin: fn(PP, PCSTR, *mut PP) -> HRESULT,
	pub QueryFilterInfo: fn(PP, PVOID) -> HRESULT,
	pub JoinFilterGraph: fn(PP, PP, PCSTR) -> HRESULT,
	pub QueryVendorInfo: fn(PP, *mut PSTR) -> HRESULT,
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
	pub(crate) ppvt: PPComVT<IUnknownVT>,
}

impl_send_sync_fromppvt!(IBaseFilter);

impl ComInterface for IBaseFilter {
	const IID: IID = IID::new(0x56a86895, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
}

macro_rules! impl_IBaseFilter {
	($name:ty, $vt:ty) => {
		use crate::com::funcs::CoTaskMemFree;
		use crate::various::WString;

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

impl_IUnknown!(IBaseFilter, IBaseFilterVT);
impl_IPersist!(IBaseFilter, IBaseFilterVT);
impl_IMediaFilter!(IBaseFilter, IBaseFilterVT);
impl_IBaseFilter!(IBaseFilter, IBaseFilterVT);
