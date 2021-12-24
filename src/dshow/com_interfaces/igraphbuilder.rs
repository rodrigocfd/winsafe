#![allow(non_snake_case)]

use crate::dshow::decl::{IBaseFilter, IPin};
use crate::ffi_types::{HANDLE, HRES, PCSTR};
use crate::kernel::decl::{HFILE, WString};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::{DshowIFilterGraph, OleIUnknown};
use crate::vt::IFilterGraphVT;

/// [`IGraphBuilder`](crate::IGraphBuilder) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
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
/// COM interface over [`IGraphBuilderVT`](crate::vt::IGraphBuilderVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{CLSID, co, CoCreateInstance, IGraphBuilder};
///
/// let obj = CoCreateInstance::<IGraphBuilder>(
///     &CLSID::FilterGraph,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// # Ok::<_, co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub struct IGraphBuilder(ComPtr);

impl_iunknown!(IGraphBuilder, 0x56a868a9, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
impl DshowIFilterGraph for IGraphBuilder {}
impl DshowIGraphBuilder for IGraphBuilder {}

/// [`IGraphBuilder`](crate::IGraphBuilder) methods methods from `dshow`
/// feature.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait DshowIGraphBuilder: DshowIFilterGraph {
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
