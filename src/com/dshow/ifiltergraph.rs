#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::dshow::ibasefilter::IBaseFilter;
use crate::com::dshow::ienumfilters::IEnumFilters;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{HRES, PCSTR, PCVOID};
use crate::privs::ok_to_hrresult;
use crate::various::WString;

/// [`IFilterGraph`](crate::dshow::IFilterGraph) virtual table.
#[repr(C)]
pub struct IFilterGraphVT {
	pub IUnknownVT: IUnknownVT,
	pub AddFilter: fn(ComPtr, ComPtr, PCSTR) -> HRES,
	pub RemoveFilter: fn(ComPtr, ComPtr) -> HRES,
	pub EnumFilters: fn(ComPtr, *mut ComPtr) -> HRES,
	pub FindFilterByName: fn(ComPtr, PCSTR, *mut ComPtr) -> HRES,
	pub ConnectDirect: fn(ComPtr, ComPtr, ComPtr, PCVOID) -> HRES,
	pub Reconnect: fn(ComPtr, ComPtr) -> HRES,
	pub Disconnect: fn(ComPtr, ComPtr) -> HRES,
	pub SetDefaultSyncSource: fn(ComPtr) -> HRES,
}

/// [`IFilterGraph`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifiltergraph)
/// COM interface over [`IFilterGraphVT`](crate::dshow::vt::IFilterGraphVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IFilterGraph(ComPtr);

impl_iunknown!(IFilterGraph, 0x56a8689f, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
impl IFilterGraphT for IFilterGraph {}

/// Exposes the [`IFilterGraph`](crate::dshow::IFilterGraph) methods.
pub trait IFilterGraphT: IUnknownT {
	/// [`IFilterGraph::AddFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-addfilter)
	/// method.
	fn AddFilter(&self, filter: &IBaseFilter, name: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFilterGraphVT);
			ok_to_hrresult(
				(vt.AddFilter)(
					self.ptr(),
					filter.ptr(),
					WString::from_str(name).as_ptr(),
				),
			)
		}
	}

	/// [`IFilterGraph::EnumFilters`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-enumfilters)
	/// method.
	fn EnumFilters(&self) -> HrResult<IEnumFilters> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFilterGraphVT);
			ok_to_hrresult(
				(vt.EnumFilters)(self.ptr(), &mut ppv_queried),
			)
		}.map(|_| IEnumFilters::from(ppv_queried))
	}

	/// [`IFilterGraph::FindFilterByName`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-findfilterbyname)
	/// method.
	fn FindFilterByName(&self, name: &str) -> HrResult<IBaseFilter> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFilterGraphVT);
			ok_to_hrresult(
				(vt.FindFilterByName)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
					&mut ppv_queried,
				),
			)
		}.map(|_| IBaseFilter::from(ppv_queried))
	}

	/// [`IFilterGraph::RemoveFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-removefilter)
	/// method.
	fn RemoveFilter(&self, filter: &IBaseFilter) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFilterGraphVT);
			ok_to_hrresult((vt.RemoveFilter)(self.ptr(), filter.ptr()))
		}
	}

	/// [`IFilterGraph::SetDefaultSyncSource`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-setdefaultsyncsource)
	/// method.
	fn SetDefaultSyncSource(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFilterGraphVT);
			ok_to_hrresult((vt.SetDefaultSyncSource)(self.ptr()))
		}
	}
}
