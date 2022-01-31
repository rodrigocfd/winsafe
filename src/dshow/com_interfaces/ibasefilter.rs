#![allow(non_snake_case)]

use crate::dshow::decl::IFilterGraph;
use crate::ffi_types::{HRES, PCSTR, PSTR, PVOID};
use crate::kernel::decl::WString;
use crate::ole::decl::{ComPtr, CoTaskMemFree, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{DshowIMediaFilter, OleIUnknown, ShlwapiIPersist};
use crate::vt::IMediaFilterVT;

/// [`IBaseFilter`](crate::IBaseFilter) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
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
/// COM interface over [`IBaseFilterVT`](crate::vt::IBaseFilterVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, CoCreateInstance, IBaseFilter};
///
/// let vmr = CoCreateInstance::<IBaseFilter>(
///     &co::CLSID::EnhancedVideoRenderer,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// # Ok::<_, co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub struct IBaseFilter(ComPtr);

impl_iunknown!(IBaseFilter, "56a86895-0ad4-11ce-b03a-0020af0ba770");
impl ShlwapiIPersist for IBaseFilter {}
impl DshowIMediaFilter for IBaseFilter {}
impl DshowIBaseFilter for IBaseFilter {}

/// [`IBaseFilter`](crate::IBaseFilter) methods from `dshow` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait DshowIBaseFilter: DshowIMediaFilter {
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
