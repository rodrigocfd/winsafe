#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::dshow::ibasefilter::IBaseFilter;
use crate::com::dshow::ienumfilters::IEnumFilters;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{HRESULT, PCSTR, PCVOID};
use crate::privs::hr_to_winresult;
use crate::various::WString;

/// [`IFilterGraph`](crate::dshow::IFilterGraph) virtual table.
pub struct IFilterGraphVT {
	pub IUnknownVT: IUnknownVT,
	pub AddFilter: fn(ComPtr, ComPtr, PCSTR) -> HRESULT,
	pub RemoveFilter: fn(ComPtr, ComPtr) -> HRESULT,
	pub EnumFilters: fn(ComPtr, *mut ComPtr) -> HRESULT,
	pub FindFilterByName: fn(ComPtr, PCSTR, *mut ComPtr) -> HRESULT,
	pub ConnectDirect: fn(ComPtr, ComPtr, ComPtr, PCVOID) -> HRESULT,
	pub Reconnect: fn(ComPtr, ComPtr) -> HRESULT,
	pub Disconnect: fn(ComPtr, ComPtr) -> HRESULT,
	pub SetDefaultSyncSource: fn(ComPtr) -> HRESULT,
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
	fn AddFilter(&self, filter: &IBaseFilter, name: &str) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFilterGraphVT);
			hr_to_winresult(
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
	fn EnumFilters(&self) -> WinResult<IEnumFilters> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFilterGraphVT);
			hr_to_winresult(
				(vt.EnumFilters)(self.ptr(), &mut ppv_queried as *mut _ as _),
			)
		}.map(|_| IEnumFilters::from(ppv_queried))
	}

	/// [`IFilterGraph::FindFilterByName`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-findfilterbyname)
	/// method.
	fn FindFilterByName(&self, name: &str) -> WinResult<IBaseFilter> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFilterGraphVT);
			hr_to_winresult(
				(vt.FindFilterByName)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
					&mut ppv_queried as *mut _ as _,
				),
			)
		}.map(|_| IBaseFilter::from(ppv_queried))
	}

	/// [`IFilterGraph::RemoveFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-removefilter)
	/// method.
	fn RemoveFilter(&self, filter: &IBaseFilter) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFilterGraphVT);
			hr_to_winresult((vt.RemoveFilter)(self.ptr(), filter.ptr()))
		}
	}

	/// [`IFilterGraph::SetDefaultSyncSource`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-setdefaultsyncsource)
	/// method.
	fn SetDefaultSyncSource(&self) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFilterGraphVT);
			hr_to_winresult((vt.SetDefaultSyncSource)(self.ptr()))
		}
	}
}
