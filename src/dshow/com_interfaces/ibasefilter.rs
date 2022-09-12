#![allow(non_camel_case_types, non_snake_case)]

use crate::dshow::decl::IFilterGraph;
use crate::kernel::decl::WString;
use crate::kernel::ffi_types::{HRES, PCSTR, PSTR, PVOID};
use crate::ole::decl::{ComPtr, CoTaskMemFree, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{dshow_IMediaFilter, ole_IPersist, ole_IUnknown};
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

com_interface! { IBaseFilter: "dshow";
	"56a86895-0ad4-11ce-b03a-0020af0ba770";
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
}

impl ole_IPersist for IBaseFilter {}
impl dshow_IMediaFilter for IBaseFilter {}
impl dshow_IBaseFilter for IBaseFilter {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IBaseFilter`](crate::IBaseFilter).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait dshow_IBaseFilter: dshow_IMediaFilter {
	/// [`IBaseFilter::JoinFilterGraph`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-joinfiltergraph)
	/// method.
	fn JoinFilterGraph(&self,
		graph: Option<&IFilterGraph>, name: &str) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<IBaseFilterVT>();
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
	#[must_use]
	fn QueryVendorInfo(&self) -> HrResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = self.vt_ref::<IBaseFilterVT>();
			ok_to_hrresult((vt.QueryVendorInfo)(self.ptr(), &mut pstr))
		}.map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr as _);
			name.to_string()
		})
	}
}
