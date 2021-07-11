#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPComVT};
use crate::ffi::{HRESULT, PCSTR, PCVOID};
use crate::structs::IID;

type PP = PPComVT<IUnknownVT>;

/// [`IFilterGraph`](crate::dshow::IFilterGraph) virtual table.
pub struct IFilterGraphVT {
	pub IUnknownVT: IUnknownVT,
	pub AddFilter: fn(PP, PP, PCSTR) -> HRESULT,
	pub RemoveFilter: fn(PP, PP) -> HRESULT,
	pub EnumFilters: fn(PP, *mut PP) -> HRESULT,
	pub FindFilterByName: fn(PP, PCSTR, *mut PP) -> HRESULT,
	pub ConnectDirect: fn(PP, PP, PP, PCVOID) -> HRESULT,
	pub Reconnect: fn(PP, PP) -> HRESULT,
	pub Disconnect: fn(PP, PP) -> HRESULT,
	pub SetDefaultSyncSource: fn(PP) -> HRESULT,
}

/// [`IFilterGraph`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifiltergraph)
/// COM interface over [`IFilterGraphVT`](crate::dshow::vt::IFilterGraphVT).
/// Inherits from [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IFilterGraph {
	pub(crate) ppvt: PPComVT<IUnknownVT>,
}

impl_send_sync_fromppvt!(IFilterGraph);

impl ComInterface for IFilterGraph {
	const IID: IID = IID::new(0x56a8689f, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
}

macro_rules! impl_IFilterGraph {
	($name:ty, $vt:ty) => {
		use crate::com::dshow::{IBaseFilter, IEnumFilters};
		use crate::various::WString;

		impl $name {
			fn ifiltergraph_vt(&self) -> &IFilterGraphVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IFilterGraph::AddFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-addfilter)
			/// method.
			pub fn AddFilter(&self,
				filter: &IBaseFilter, name: &str) -> WinResult<()>
			{
				hr_to_winresult(
					(self.ifiltergraph_vt().AddFilter)(
						self.ppvt,
						filter.ppvt,
						unsafe { WString::from_str(name).as_ptr() },
					),
				)
			}

			/// [`IFilterGraph::EnumFilters`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-enumfilters)
			/// method.
			pub fn EnumFilters(&self) -> WinResult<IEnumFilters> {
				let mut ppvQueried: PPComVT<IUnknownVT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.ifiltergraph_vt().EnumFilters)(
						self.ppvt,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| IEnumFilters::from(ppvQueried))
			}

			/// [`IFilterGraph::FindFilterByName`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-findfilterbyname)
			/// method.
			pub fn FindFilterByName(&self, name: &str) -> WinResult<IBaseFilter> {
				let mut ppvQueried: PPComVT<IUnknownVT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.ifiltergraph_vt().FindFilterByName)(
						self.ppvt,
						unsafe { WString::from_str(name).as_ptr() },
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| IBaseFilter::from(ppvQueried))
			}

			/// [`IFilterGraph::RemoveFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-removefilter)
			/// method.
			pub fn RemoveFilter(&self, filter: &IBaseFilter) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiltergraph_vt().RemoveFilter)(self.ppvt, filter.ppvt),
				)
			}

			/// [`IFilterGraph::SetDefaultSyncSource`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-setdefaultsyncsource)
			/// method.
			pub fn SetDefaultSyncSource(&self) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiltergraph_vt().SetDefaultSyncSource)(self.ppvt),
				)
			}
		}
	};
}

impl_IUnknown!(IFilterGraph, IFilterGraphVT);
impl_IFilterGraph!(IFilterGraph, IFilterGraphVT);
