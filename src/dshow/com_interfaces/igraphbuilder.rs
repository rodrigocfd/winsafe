#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dshow::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IGraphBuilder: "56a868a9-0ad4-11ce-b03a-0020af0ba770";
	/// [`IGraphBuilder`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-igraphbuilder)
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
	/// let obj = w::CoCreateInstance::<w::IGraphBuilder>(
	///     &co::CLSID::FilterGraph,
	///     None::<&w::IUnknown>,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl dshow_IFilterGraph for IGraphBuilder {}
impl dshow_IGraphBuilder for IGraphBuilder {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IGraphBuilder`](crate::IGraphBuilder).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IGraphBuilder: dshow_IFilterGraph {
	fn_com_noparm! { Abort: IGraphBuilderVT;
		/// [`IGraphBuilder::Abort`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-abort)
		/// method.
	}

	/// [`IGraphBuilder::AddSourceFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-addsourcefilter)
	/// method.
	#[must_use]
	fn AddSourceFilter(&self, file_name: &str, filter_name: &str) -> HrResult<IBaseFilter> {
		let mut queried = unsafe { IBaseFilter::null() };
		ok_to_hrresult(unsafe {
			(vt::<IGraphBuilderVT>(self).AddSourceFilter)(
				self.ptr(),
				WString::from_str(file_name).as_ptr(),
				WString::from_str(filter_name).as_ptr(),
				queried.as_mut(),
			)
		})
		.map(|_| queried)
	}

	/// [`IGraphBuilder::Connect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-connect)
	/// method.
	fn Connect(&self, pin_out: &impl dshow_IPin, pin_in: &impl dshow_IPin) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IGraphBuilderVT>(self).Connect)(self.ptr(), pin_out.ptr(), pin_in.ptr())
		})
	}

	/// [`IGraphBuilder::Render`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-render)
	/// method.
	fn Render(&self, pin_out: &impl dshow_IPin) -> HrResult<()> {
		ok_to_hrresult(unsafe { (vt::<IGraphBuilderVT>(self).Render)(self.ptr(), pin_out.ptr()) })
	}

	/// [`IGraphBuilder::RenderFile`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-renderfile)
	/// method.
	fn RenderFile(&self, file: &str) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IGraphBuilderVT>(self).RenderFile)(
				self.ptr(),
				WString::from_str(file).as_ptr(),
				std::ptr::null(),
			)
		})
	}

	/// [`IGraphBuilder::SetLogFile`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-setlogfile)
	/// method.
	fn SetLogFile(&self, hfile: Option<&HFILE>) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IGraphBuilderVT>(self).SetLogFile)(
				self.ptr(),
				hfile.map_or(std::ptr::null_mut(), |h| h.ptr()),
			)
		})
	}

	/// [`IGraphBuilder::ShouldOperationContinue`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-shouldoperationcontinue)
	/// method.
	#[must_use]
	fn ShouldOperationContinue(&self) -> HrResult<bool> {
		okfalse_to_hrresult(unsafe {
			(vt::<IGraphBuilderVT>(self).ShouldOperationContinue)(self.ptr())
		})
	}
}
