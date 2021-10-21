#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::dshow::ibasefilter::IBaseFilter;
use crate::com::dshow::ifiltergraph::{IFilterGraphT, IFilterGraphVT};
use crate::com::dshow::ipin::IPin;
use crate::com::iunknown::{ComPtr, IUnknownT};
use crate::ffi::{HANDLE, HRESULT, PCSTR};
use crate::handles::HFILE;
use crate::privs::{hr_to_winresult, hr_to_winresult_bool};
use crate::various::WString;

/// [`IGraphBuilder`](crate::dshow::IGraphBuilder) virtual table.
pub struct IGraphBuilderVT {
	pub IFilterGraphVT: IFilterGraphVT,
	pub Connect: fn(ComPtr, ComPtr, ComPtr) -> HRESULT,
	pub Render: fn(ComPtr, ComPtr) -> HRESULT,
	pub RenderFile: fn(ComPtr, PCSTR, PCSTR) -> HRESULT,
	pub AddSourceFilter: fn(ComPtr, PCSTR, PCSTR, *mut ComPtr) -> HRESULT,
	pub SetLogFile: fn(ComPtr, HANDLE) -> HRESULT,
	pub Abort: fn(ComPtr) -> HRESULT,
	pub ShouldOperationContinue: fn(ComPtr) -> HRESULT,
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
	fn Abort(&self) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			hr_to_winresult((vt.Abort)(self.ptr()))
		}
	}

	/// [`IGraphBuilder::AddSourceFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-addsourcefilter)
	/// method.
	fn AddSourceFilter(&self,
		file_name: &str, filter_name: &str) -> WinResult<IBaseFilter>
	{
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			hr_to_winresult(
				(vt.AddSourceFilter)(
					self.ptr(),
					WString::from_str(file_name).as_ptr(),
					WString::from_str(filter_name).as_ptr(),
					&mut ppv_queried as *mut _ as _,
				),
			)
		}.map(|_| IBaseFilter::from(ppv_queried))
	}

	/// [`IGraphBuilder::Connect`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-connect)
	/// method.
	fn Connect(&self, pin_out: &IPin, pin_in: &IPin) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			hr_to_winresult((vt.Connect)(self.ptr(), pin_out.ptr(), pin_in.ptr()))
		}
	}

	/// [`IGraphBuilder::RenderFile`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-renderfile)
	/// method.
	fn RenderFile(&self, file: &str) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			hr_to_winresult(
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
	fn SetLogFile(&self, hfile: Option<HFILE>) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			hr_to_winresult(
				(vt.SetLogFile)(
					self.ptr(),
					hfile.map_or(std::ptr::null_mut(), |h| h.ptr),
				),
			)
		}
	}

	/// [`IGraphBuilder::ShouldOperationContinue`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-shouldoperationcontinue)
	/// method.
	fn ShouldOperationContinue(&self) -> WinResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IGraphBuilderVT);
			hr_to_winresult_bool((vt.ShouldOperationContinue)(self.ptr()))
		}
	}
}
