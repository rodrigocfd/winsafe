#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dshow::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IFilterGraph: "56a8689f-0ad4-11ce-b03a-0020af0ba770";
	/// [`IFilterGraph`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifiltergraph)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IFilterGraph for IFilterGraph {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IFilterGraph`](crate::IFilterGraph).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IFilterGraph: ole_IUnknown {
	/// [`IFilterGraph::AddFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-addfilter)
	/// method.
	fn AddFilter(&self, filter: &impl dshow_IBaseFilter, name: &str) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFilterGraphVT>(self).AddFilter)(
				self.ptr(),
				filter.ptr(),
				WString::from_str(name).as_ptr(),
			)
		})
		.to_hrresult()
	}

	/// [`IFilterGraph::ConnectDirect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-connectdirect)
	/// method.
	fn ConnectDirect(
		&self,
		pin_out: &impl dshow_IPin,
		pin_in: &impl dshow_IPin,
		mt: Option<&AM_MEDIA_TYPE>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFilterGraphVT>(self).ConnectDirect)(
				self.ptr(),
				pin_out.ptr(),
				pin_in.ptr(),
				pcvoid_or_null(mt),
			)
		})
		.to_hrresult()
	}

	/// [`IFilterGraph::Disconnect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-disconnect)
	/// method.
	fn Disconnect(&self, pin: &impl dshow_IPin) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFilterGraphVT>(self).Disconnect)(self.ptr(), pin.ptr()) })
			.to_hrresult()
	}

	fn_com_interface_get! { EnumFilters: IFilterGraphVT => IEnumFilters;
		/// [`IFilterGraph::EnumFilters`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-enumfilters)
		/// method.
	}

	/// [`IFilterGraph::FindFilterByName`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-findfilterbyname)
	/// method.
	#[must_use]
	fn FindFilterByName(&self, name: &str) -> HrResult<IBaseFilter> {
		let mut queried = unsafe { IBaseFilter::null() };
		HrRet(unsafe {
			(vt::<IFilterGraphVT>(self).FindFilterByName)(
				self.ptr(),
				WString::from_str(name).as_ptr(),
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IFilterGraph::Reconnect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-reconnect)
	/// method.
	fn Reconnect(&self, pin: &impl dshow_IPin) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFilterGraphVT>(self).Reconnect)(self.ptr(), pin.ptr()) })
			.to_hrresult()
	}

	/// [`IFilterGraph::RemoveFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-removefilter)
	/// method.
	fn RemoveFilter(&self, filter: &impl dshow_IBaseFilter) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFilterGraphVT>(self).RemoveFilter)(self.ptr(), filter.ptr()) })
			.to_hrresult()
	}

	fn_com_noparm! { SetDefaultSyncSource: IFilterGraphVT;
		/// [`IFilterGraph::SetDefaultSyncSource`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-setdefaultsyncsource)
		/// method.
	}
}
