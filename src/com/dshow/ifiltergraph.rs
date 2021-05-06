#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::dshow::{IBaseFilter, IEnumFilters};
use crate::com::dshow::vt::{IBaseFilterVT, IEnumFiltersVT, IFilterGraphVT};
use crate::privs::hr_to_winresult;
use crate::WString;

macro_rules! IFilterGraph_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IFilterGraph::AddFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-addfilter)
			/// method.
			pub fn AddFilter(&self,
				filter: &IBaseFilter, name: &str) -> WinResult<()>
			{
				let ppvt = unsafe { self.ppvt::<IFilterGraphVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).AddFilter)(
							ppvt,
							filter.ppvt(),
							WString::from_str(name).as_ptr(),
						)
					},
				)
			}

			/// [`IFilterGraph::EnumFilters`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-enumfilters)
			/// method.
			pub fn EnumFilters(&self) -> WinResult<IEnumFilters> {
				let ppvt = unsafe { self.ppvt::<IFilterGraphVT>() };
				let mut ppvQueried: PPComVT<IEnumFiltersVT> = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						((**ppvt).EnumFilters)(ppvt, &mut ppvQueried as *mut _ as _)
					},
				).map(|_| IEnumFilters::from(ppvQueried))
			}

			/// [`IFilterGraph::FindFilterByName`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-findfilterbyname)
			/// method.
			pub fn FindFilterByName(&self, name: &str) -> WinResult<IBaseFilter> {
				let ppvt = unsafe { self.ppvt::<IFilterGraphVT>() };
				let mut ppvQueried: PPComVT<IBaseFilterVT> = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						((**ppvt).FindFilterByName)(
							ppvt,
							WString::from_str(name).as_ptr(),
							&mut ppvQueried as *mut _ as _,
						)
					},
				).map(|_| IBaseFilter::from(ppvQueried))
			}

			/// [`IFilterGraph::RemoveFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-removefilter)
			/// method.
			pub fn RemoveFilter(&self, filter: &IBaseFilter) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFilterGraphVT>() };
				hr_to_winresult(
					unsafe { ((**ppvt).RemoveFilter)(ppvt, filter.ppvt()) },
				)
			}

			/// [`IFilterGraph::SetDefaultSyncSource`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-setdefaultsyncsource)
			/// method.
			pub fn SetDefaultSyncSource(&self) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFilterGraphVT>() };
				hr_to_winresult(unsafe { ((**ppvt).SetDefaultSyncSource)(ppvt) })
			}
		}
	};
}

IFilterGraph_impl! {
	/// [`IFilterGraph`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifiltergraph)
	/// COM interface.
	///
	/// Virtual table: [`IFilterGraphVT`](crate::dshow::vt::IFilterGraphVT).
	///
	/// Inherits from:
	/// * [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IFilterGraph, IFilterGraphVT
}
