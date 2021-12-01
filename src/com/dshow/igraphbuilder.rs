#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::dshow::ibasefilter::IBaseFilter;
use crate::com::dshow::ifiltergraph::{IFilterGraphT, IFilterGraphVT};
use crate::com::dshow::ipin::IPin;
use crate::com::iunknown::{ComPtr, IUnknownT};
use crate::ffi::{HANDLE, HRES, PCSTR};
use crate::handles::HFILE;
use crate::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::various::WString;

/// [`IGraphBuilder`](crate::dshow::IGraphBuilder) virtual table.
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

/// [`IGraphBuilder`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-igraphbuilder)
/// COM interface over [`IGraphBuilderVT`](crate::dshow::vt::IGraphBuilderVT).
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
/// let obj = CoCreateInstance::<dshow::IGraphBuilder>(
///     &dshow::clsid::FilterGraph,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct IGraphBuilder(ComPtr);

impl_iunknown!(IGraphBuilder, 0x56a868a9, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
impl IFilterGraphT for IGraphBuilder {}
impl IGraphBuilderT for IGraphBuilder {}

/// Exposes the [`IGraphBuilder`](crate::dshow::IGraphBuilder) methods.
pub trait IGraphBuilderT: IFilterGraphT {
	/// [`IGraphBuilder::Abort`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-abort)
	/// method.
	fn Abort(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			ok_to_hrresult((vt.Abort)(self.ptr()))
		}
	}

	/// [`IGraphBuilder::AddSourceFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-addsourcefilter)
	/// method.
	fn AddSourceFilter(&self,
		file_name: &str, filter_name: &str) -> HrResult<IBaseFilter>
	{
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			ok_to_hrresult(
				(vt.AddSourceFilter)(
					self.ptr(),
					WString::from_str(file_name).as_ptr(),
					WString::from_str(filter_name).as_ptr(),
					&mut ppv_queried,
				),
			)
		}.map(|_| IBaseFilter::from(ppv_queried))
	}

	/// [`IGraphBuilder::Connect`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-connect)
	/// method.
	fn Connect(&self, pin_out: &IPin, pin_in: &IPin) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			ok_to_hrresult((vt.Connect)(self.ptr(), pin_out.ptr(), pin_in.ptr()))
		}
	}

	/// [`IGraphBuilder::RenderFile`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-renderfile)
	/// method.
	fn RenderFile(&self, file: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			ok_to_hrresult(
				(vt.RenderFile)(
					self.ptr(),
					WString::from_str(file).as_ptr(),
					std::ptr::null(),
				),
			)
		}
	}

	/// [`IGraphBuilder::SetLogFile`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-setlogfile)
	/// method.
	fn SetLogFile(&self, hfile: Option<HFILE>) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			ok_to_hrresult(
				(vt.SetLogFile)(
					self.ptr(),
					hfile.map_or(std::ptr::null_mut(), |h| h.0),
				),
			)
		}
	}

	/// [`IGraphBuilder::ShouldOperationContinue`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-shouldoperationcontinue)
	/// method.
	fn ShouldOperationContinue(&self) -> HrResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			okfalse_to_hrresult((vt.ShouldOperationContinue)(self.ptr()))
		}
	}
}
