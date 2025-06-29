#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dshow::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IBaseFilter: "56a86895-0ad4-11ce-b03a-0020af0ba770";
	/// [`IBaseFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ibasefilter)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let vmr = w::CoCreateInstance::<w::IBaseFilter>(
	///     &co::CLSID::EnhancedVideoRenderer,
	///     None::<&w::IUnknown>,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # w::HrResult::Ok(())
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
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IBaseFilter: dshow_IMediaFilter {
	fn_com_interface_get! { EnumPins: IBaseFilterVT => IEnumPins;
		/// [`IBaseFilter::EnumPins`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-enumpins)
		/// method.
	}

	/// [`IBaseFilter::FindPin`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-findpin)
	/// method.
	#[must_use]
	fn FindPin(&self, id: &str) -> HrResult<IPin> {
		let mut queried = unsafe { IPin::null() };
		ok_to_hrresult(unsafe {
			(vt::<IBaseFilterVT>(self).FindPin)(
				self.ptr(),
				WString::from_str(id).as_ptr(),
				queried.as_mut(),
			)
		})
		.map(|_| queried)
	}

	/// [`IBaseFilter::JoinFilterGraph`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-joinfiltergraph)
	/// method.
	fn JoinFilterGraph(&self, graph: Option<&impl dshow_IFilterGraph>, name: &str) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IBaseFilterVT>(self).JoinFilterGraph)(
				self.ptr(),
				graph.map_or(std::ptr::null_mut(), |g| g.ptr()),
				WString::from_str(name).as_ptr(),
			)
		})
	}

	/// [`IBaseFilter::QueryFilterInfo`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-queryfilterinfo)
	/// method.
	fn QueryFilterInfo(&self, info: &mut FILTER_INFO) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IBaseFilterVT>(self).QueryFilterInfo)(self.ptr(), pvoid(info))
		})
	}

	/// [`IBaseFilter::QueryVendorInfo`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ibasefilter-queryvendorinfo)
	/// method.
	#[must_use]
	fn QueryVendorInfo(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		ok_to_hrresult(unsafe {
			(vt::<IBaseFilterVT>(self).QueryVendorInfo)(self.ptr(), &mut pstr)
		})
		.map(|_| htaskmem_ptr_to_str(pstr))
	}
}
