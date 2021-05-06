#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::dshow::{IBaseFilter, IEnumFilters, IPin};
use crate::com::dshow::vt::{
	IBaseFilterVT,
	IEnumFiltersVT,
	IFilterGraphVT,
	IGraphBuilderVT,
};
use crate::handles::HFILE;
use crate::privs::{hr_to_winresult, hr_to_winresult_bool};
use crate::WString;

macro_rules! IGraphBuilder_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IFilterGraph_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IGraphBuilder::Abort`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-abort)
			/// method.
			pub fn Abort(&self) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IGraphBuilderVT>() };
				hr_to_winresult(unsafe { ((**ppvt).Abort)(ppvt) })
			}

			/// [`IGraphBuilder::AddSourceFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-addsourcefilter)
			/// method.
			pub fn AddSourceFilter(&self,
				fileName: &str, filterName: &str) -> WinResult<IBaseFilter>
			{
				let mut ppvQueried: PPComVT<IBaseFilterVT> = std::ptr::null_mut();
				let ppvt = unsafe { self.ppvt::<IGraphBuilderVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).AddSourceFilter)(
							ppvt,
							WString::from_str(fileName).as_ptr(),
							WString::from_str(filterName).as_ptr(),
							&mut ppvQueried as *mut _ as _,
						)
					},
				).map(|_| IBaseFilter::from(ppvQueried))
			}

			/// [`IGraphBuilder::Connect`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-connect)
			/// method.
			pub fn Connect(&self, pinOut: &IPin, pinIn: &IPin) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IGraphBuilderVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).Connect)(ppvt, pinOut.ppvt(), pinIn.ppvt())
					},
				)
			}

			/// [`IGraphBuilder::RenderFile`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-renderfile)
			/// method.
			pub fn RenderFile(&self, file: &str) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IGraphBuilderVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).RenderFile)(
							ppvt,
							WString::from_str(file).as_ptr(),
							std::ptr::null(),
						)
					},
				)
			}

			/// [`IGraphBuilder::SetLogFile`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-setlogfile)
			/// method.
			pub fn SetLogFile(&self, hFile: Option<HFILE>) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IGraphBuilderVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetLogFile)(
							ppvt,
							match hFile {
								Some(hFile) => hFile.ptr,
								None => std::ptr::null_mut(),
							},
						)
					},
				)
			}

			/// [`IGraphBuilder::ShouldOperationContinue`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-shouldoperationcontinue)
			/// method.
			pub fn ShouldOperationContinue(&self) -> WinResult<bool> {
				let ppvt = unsafe { self.ppvt::<IGraphBuilderVT>() };
				hr_to_winresult_bool(
					unsafe { ((**ppvt).ShouldOperationContinue)(ppvt) },
				)
			}
		}
	};
}

IGraphBuilder_impl! {
	/// [`IGraphBuilder`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-igraphbuilder)
	/// COM interface over
	/// [`IGraphBuilderVT`](crate::dshow::vt::IGraphBuilderVT). Inherits from
	/// [`IFilterGraph`](crate::dshow::IFilterGraph),
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, CoCreateInstance, dshow};
	///
	/// let obj: dshow::IGraphBuilder = CoCreateInstance(
	///     &dshow::clsid::FilterGraph,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// ).unwrap();
	/// ```
	IGraphBuilder, IGraphBuilderVT
}
