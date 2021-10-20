#![allow(non_snake_case)]

use crate::com::dshow::vt::IFilterGraphVT;
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::{HANDLE, HRESULT, PCSTR};
use crate::structs::IID;

/// [`IGraphBuilder`](crate::dshow::IGraphBuilder) virtual table.
pub struct IGraphBuilderVT {
	pub IFilterGraphVT: IFilterGraphVT,
	pub Connect: fn(PPVT, PPVT, PPVT) -> HRESULT,
	pub Render: fn(PPVT, PPVT) -> HRESULT,
	pub RenderFile: fn(PPVT, PCSTR, PCSTR) -> HRESULT,
	pub AddSourceFilter: fn(PPVT, PCSTR, PCSTR, *mut PPVT) -> HRESULT,
	pub SetLogFile: fn(PPVT, HANDLE) -> HRESULT,
	pub Abort: fn(PPVT) -> HRESULT,
	pub ShouldOperationContinue: fn(PPVT) -> HRESULT,
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
/// use winsafe::prelude::*;
/// use winsafe::{co, CoCreateInstance, dshow};
///
/// let obj = CoCreateInstance::<dshow::IGraphBuilder>(
///     &dshow::clsid::FilterGraph,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct IGraphBuilder {
	pub(crate) ppvt: PPVT,
}

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
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`IGraphBuilder::Abort`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-abort)
			/// method.
			pub fn Abort(&self) -> WinResult<()> {
				hr_to_winresult((self.igraphbuilder_vt().Abort)(self.ppvt))
			}

			/// [`IGraphBuilder::AddSourceFilter`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-addsourcefilter)
			/// method.
			pub fn AddSourceFilter(&self,
				file_name: &str, filter_name: &str) -> WinResult<IBaseFilter>
			{
				let mut ppv_queried: PPVT = std::ptr::null_mut();
				hr_to_winresult(
					(self.igraphbuilder_vt().AddSourceFilter)(
						self.ppvt,
						unsafe { WString::from_str(file_name).as_ptr() },
						unsafe { WString::from_str(filter_name).as_ptr() },
						&mut ppv_queried as *mut _ as _,
					),
				).map(|_| IBaseFilter::from(ppv_queried))
			}

			/// [`IGraphBuilder::Connect`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-igraphbuilder-connect)
			/// method.
			pub fn Connect(&self, pin_out: &IPin, pin_in: &IPin) -> WinResult<()> {
				hr_to_winresult(
					(self.igraphbuilder_vt().Connect)(
						self.ppvt,
						pin_out.ppvt,
						pin_in.ppvt,
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
			pub fn SetLogFile(&self, hfile: Option<HFILE>) -> WinResult<()> {
				hr_to_winresult(
					(self.igraphbuilder_vt().SetLogFile)(
						self.ppvt,
						hfile.map_or(std::ptr::null_mut(), |h| h.ptr),
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
