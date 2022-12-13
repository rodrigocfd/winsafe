#![allow(non_camel_case_types, non_snake_case)]

use crate::dshow::decl::{IBaseFilter, IPin};
use crate::kernel::decl::{HFILE, WString};
use crate::kernel::ffi_types::{HANDLE, HRES, PCSTR};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::{dshow_IFilterGraph, ole_IUnknown};
use crate::vt::IFilterGraphVT;

/// [`IGraphBuilder`](crate::IGraphBuilder) virtual table.
#[repr(C)]
pub struct IGraphBuilderVT {
	pub IFilterGraphVT: IFilterGraphVT,
	pub Connect: fn(ComPtr, ComPtr, ComPtr) -> HRES,
	pub Render: fn(ComPtr, ComPtr) -> HRES,
	pub RenderFile: fn(ComPtr, PCSTR, PCSTR) -> HRES,
	pub AddSourceFilter: fn(ComPtr, PCSTR, PCSTR, *mut ComPtr) -> HRES,
	pub SetLogFile: fn(ComPtr, HANDLE) -> HRES,
	pub Abort: fn(ComPtr) -> HRES,
	pub ShouldOperationContinue: fn(ComPtr) -> HRES,
}

com_interface! { IGraphBuilder: "56a868a9-0ad4-11ce-b03a-0020af0ba770";
	/// [`IGraphBuilder`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-igraphbuilder)
	/// COM interface over [`IGraphBuilderVT`](crate::vt::IGraphBuilderVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, CoCreateInstance, IGraphBuilder};
	///
	/// let obj = CoCreateInstance::<IGraphBuilder>(
	///     &co::CLSID::FilterGraph,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl dshow_IFilterGraph for IGraphBuilder {}
impl dshow_IGraphBuilder for IGraphBuilder {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IGraphBuilder`](crate::IGraphBuilder).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IGraphBuilder: dshow_IFilterGraph {
	/// [`IGraphBuilder::Abort`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-abort)
	/// method.
	fn Abort(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IGraphBuilderVT>();
			ok_to_hrresult((vt.Abort)(self.ptr()))
		}
	}

	/// [`IGraphBuilder::AddSourceFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-addsourcefilter)
	/// method.
	#[must_use]
	fn AddSourceFilter(&self,
		file_name: &str, filter_name: &str) -> HrResult<IBaseFilter>
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IGraphBuilderVT>();
			ok_to_hrresult(
				(vt.AddSourceFilter)(
					self.ptr(),
					WString::from_str(file_name).as_ptr(),
					WString::from_str(filter_name).as_ptr(),
					&mut ppv_queried,
				),
			).map(|_| IBaseFilter::from(ppv_queried))
		}
	}

	/// [`IGraphBuilder::Connect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-connect)
	/// method.
	fn Connect(&self, pin_out: &IPin, pin_in: &IPin) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IGraphBuilderVT>();
			ok_to_hrresult((vt.Connect)(self.ptr(), pin_out.ptr(), pin_in.ptr()))
		}
	}

	/// [`IGraphBuilder::RenderFile`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-renderfile)
	/// method.
	fn RenderFile(&self, file: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IGraphBuilderVT>();
			ok_to_hrresult(
				(vt.RenderFile)(
					self.ptr(),
					WString::from_str(file).as_ptr(),
					std::ptr::null(),
				),
			)
		}
	}

	/// [`IGraphBuilder::SetLogFile`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-setlogfile)
	/// method.
	fn SetLogFile(&self, hfile: Option<&HFILE>) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IGraphBuilderVT>();
			ok_to_hrresult(
				(vt.SetLogFile)(
					self.ptr(),
					hfile.map_or(std::ptr::null_mut(), |h| h.0),
				),
			)
		}
	}

	/// [`IGraphBuilder::ShouldOperationContinue`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-shouldoperationcontinue)
	/// method.
	#[must_use]
	fn ShouldOperationContinue(&self) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IGraphBuilderVT>();
			okfalse_to_hrresult((vt.ShouldOperationContinue)(self.ptr()))
		}
	}
}
