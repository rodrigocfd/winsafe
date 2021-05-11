#![allow(non_snake_case)]

macro_rules! IGraphBuilder_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::dshow::IPin;
		use crate::com::dshow::vt::IGraphBuilderVT;
		use crate::handles::HFILE;
		use crate::privs::hr_to_winresult_bool;

		IFilterGraph_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn igraphbuilder_vt(&self) -> &IGraphBuilderVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IGraphBuilder::Abort`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-abort)
			/// method.
			pub fn Abort(&self) -> WinResult<()> {
				hr_to_winresult((self.igraphbuilder_vt().Abort)(self.ppvt))
			}

			/// [`IGraphBuilder::AddSourceFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-addsourcefilter)
			/// method.
			pub fn AddSourceFilter(&self,
				fileName: &str, filterName: &str) -> WinResult<IBaseFilter>
			{
				let mut ppvQueried: PPComVT<IBaseFilterVT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.igraphbuilder_vt().AddSourceFilter)(
						self.ppvt,
						unsafe { WString::from_str(fileName).as_ptr() },
						unsafe { WString::from_str(filterName).as_ptr() },
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| IBaseFilter::from(ppvQueried))
			}

			/// [`IGraphBuilder::Connect`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-connect)
			/// method.
			pub fn Connect(&self, pinOut: &IPin, pinIn: &IPin) -> WinResult<()> {
				hr_to_winresult(
					(self.igraphbuilder_vt().Connect)(
						self.ppvt,
						pinOut.ppvt,
						pinIn.ppvt,
					),
				)
			}

			/// [`IGraphBuilder::RenderFile`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-renderfile)
			/// method.
			pub fn RenderFile(&self, file: &str) -> WinResult<()> {
				hr_to_winresult(
					(self.igraphbuilder_vt().RenderFile)(
						self.ppvt,
						unsafe { WString::from_str(file).as_ptr() },
						std::ptr::null(),
					),
				)
			}

			/// [`IGraphBuilder::SetLogFile`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-setlogfile)
			/// method.
			pub fn SetLogFile(&self, hFile: Option<HFILE>) -> WinResult<()> {
				hr_to_winresult(
					(self.igraphbuilder_vt().SetLogFile)(
						self.ppvt,
						hFile.map_or(std::ptr::null_mut(), |h| h.ptr),
					),
				)
			}

			/// [`IGraphBuilder::ShouldOperationContinue`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-shouldoperationcontinue)
			/// method.
			pub fn ShouldOperationContinue(&self) -> WinResult<bool> {
				hr_to_winresult_bool(
					(self.igraphbuilder_vt().ShouldOperationContinue)(self.ppvt),
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
	IGraphBuilder, crate::com::dshow::vt::IGraphBuilderVT
}
