#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::dshow::ifiltergraph::IFilterGraph;
use crate::com::dshow::imediafilter::{IMediaFilterT, IMediaFilterVT};
use crate::com::funcs::CoTaskMemFree;
use crate::com::idl::ipersist::IPersistT;
use crate::com::iunknown::{ComPtr, IUnknownT};
use crate::ffi::{HRES, PCSTR, PSTR, PVOID};
use crate::privs::ok_to_hrresult;
use crate::various::WString;

/// [`IBaseFilter`](crate::dshow::IBaseFilter) virtual table.
#[repr(C)]
pub struct IBaseFilterVT {
	pub IMediaFilterVT: IMediaFilterVT,
	pub EnumPins: fn(ComPtr, *mut ComPtr) -> HRES,
	pub FindPin: fn(ComPtr, PCSTR, *mut ComPtr) -> HRES,
	pub QueryFilterInfo: fn(ComPtr, PVOID) -> HRES,
	pub JoinFilterGraph: fn(ComPtr, ComPtr, PCSTR) -> HRES,
	pub QueryVendorInfo: fn(ComPtr, *mut PSTR) -> HRES,
}

/// [`IBaseFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ibasefilter)
/// COM interface over [`IBaseFilterVT`](crate::dshow::vt::IBaseFilterVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::prelude::*;
/// use winsafe::{co, CoCreateInstance, dshow};
///
/// let vmr = CoCreateInstance::<dshow::IBaseFilter>(
///     &dshow::clsid::EnhancedVideoRenderer,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct IBaseFilter(ComPtr);

impl_iunknown!(IBaseFilter, 0x56a86895, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
impl IPersistT for IBaseFilter {}
impl IMediaFilterT for IBaseFilter {}
impl IBaseFilterT for IBaseFilter {}

/// Exposes the [`IBaseFilter`](crate::dshow::IBaseFilter) methods.
pub trait IBaseFilterT: IMediaFilterT {
	/// [`IBaseFilter::JoinFilterGraph`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-joinfiltergraph)
	/// method.
	fn JoinFilterGraph(&self,
		graph: Option<&IFilterGraph>, name: &str) -> HrResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IBaseFilterVT);
			ok_to_hrresult(
				(vt.JoinFilterGraph)(
					self.ptr(),
					graph.map_or(ComPtr::null(), |g| g.ptr()),
					WString::from_str(name).as_ptr(),
				),
			)
		}
	}

	/// [`IBaseFilter::QueryVendorInfo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-queryvendorinfo)
	/// method.
	fn QueryVendorInfo(&self) -> HrResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IBaseFilterVT);
			ok_to_hrresult((vt.QueryVendorInfo)(self.ptr(), &mut pstr))
		}.map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr);
			name.to_string()
		})
	}
}
