#![allow(non_snake_case)]

use crate::com::dshow::vt::IFilterGraphVT;
use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPComVT};
use crate::ffi::{HANDLE, HRESULT, PCSTR};
use crate::structs::IID;

type PP = PPComVT<IUnknownVT>;

/// [`IGraphBuilder`](crate::dshow::IGraphBuilder) virtual table.
pub struct IGraphBuilderVT {
	pub IFilterGraphVT: IFilterGraphVT,
	pub Connect: fn(PP, PP, PP) -> HRESULT,
	pub Render: fn(PP, PP) -> HRESULT,
	pub RenderFile: fn(PP, PCSTR, PCSTR) -> HRESULT,
	pub AddSourceFilter: fn(PP, PCSTR, PCSTR, *mut PP) -> HRESULT,
	pub SetLogFile: fn(PP, HANDLE) -> HRESULT,
	pub Abort: fn(PP) -> HRESULT,
	pub ShouldOperationContinue: fn(PP) -> HRESULT,
}

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
/// let obj = CoCreateInstance::<dshow::IGraphBuilder>(
///     &dshow::clsid::FilterGraph,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
pub struct IGraphBuilder {
	pub(crate) ppvt: PPComVT<IUnknownVT>,
}

impl_send_sync_fromppvt!(IGraphBuilder);

impl ComInterface for IGraphBuilder {
	const IID: IID = IID::new(0x56a868a9, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
}

macro_rules! impl_IGraphBuilder {
	($name:ty, $vt:ty) => {
		use crate::com::dshow::IPin;
		use crate::handles::HFILE;
		use crate::privs::hr_to_winresult_bool;

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
				let mut ppvQueried: PPComVT<IUnknownVT> = std::ptr::null_mut();
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

impl_IUnknown!(IGraphBuilder, IGraphBuilderVT);
impl_IFilterGraph!(IGraphBuilder, IGraphBuilderVT);
impl_IGraphBuilder!(IGraphBuilder, IGraphBuilderVT);
