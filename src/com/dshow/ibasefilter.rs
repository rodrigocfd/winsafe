#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::dshow::ifiltergraph::IFilterGraph;
use crate::com::dshow::imediafilter::{IMediaFilterT, IMediaFilterVT};
use crate::com::funcs::CoTaskMemFree;
use crate::com::idl::ipersist::IPersistT;
use crate::com::iunknown::{ComPtr, IUnknownT};
use crate::ffi::{HRESULT, PCSTR, PSTR, PVOID};
use crate::privs::hr_to_winresult;
use crate::various::WString;

/// [`IBaseFilter`](crate::dshow::IBaseFilter) virtual table.
pub struct IBaseFilterVT {
	pub IMediaFilterVT: IMediaFilterVT,
	pub EnumPins: fn(ComPtr, *mut ComPtr) -> HRESULT,
	pub FindPin: fn(ComPtr, PCSTR, *mut ComPtr) -> HRESULT,
	pub QueryFilterInfo: fn(ComPtr, PVOID) -> HRESULT,
	pub JoinFilterGraph: fn(ComPtr, ComPtr, PCSTR) -> HRESULT,
	pub QueryVendorInfo: fn(ComPtr, *mut PSTR) -> HRESULT,
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
		graph: Option<&IFilterGraph>, name: &str) -> WinResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IBaseFilterVT);
			hr_to_winresult(
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
	fn QueryVendorInfo(&self) -> WinResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IBaseFilterVT);
			hr_to_winresult((vt.QueryVendorInfo)(self.ptr(), &mut pstr))
		}.map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr);
			name.to_string()
		})
	}
}
