#![allow(non_snake_case)]

macro_rules! pub_struct_IFilterGraph {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::dshow::{IBaseFilter, IEnumFilters};
		use crate::com::dshow::vt::{IBaseFilterVT, IEnumFiltersVT, IFilterGraphVT};
		use crate::WString;

		pub_struct_IUnknown! {
			$(#[$doc])*
			$name, $vt
		}

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
				let mut ppvQueried: PPComVT<IEnumFiltersVT> = std::ptr::null_mut();
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
				let mut ppvQueried: PPComVT<IBaseFilterVT> = std::ptr::null_mut();
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

pub_struct_IFilterGraph! {
	/// [`IFilterGraph`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifiltergraph)
	/// COM interface over [`IFilterGraphVT`](crate::dshow::vt::IFilterGraphVT).
	/// Inherits from [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IFilterGraph, crate::com::dshow::vt::IFilterGraphVT
}
