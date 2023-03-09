#![allow(non_camel_case_types, non_snake_case)]

use crate::dshow::decl::AM_MEDIA_TYPE;
use crate::kernel::ffi_types::{HRES, PCSTR, PCVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{dshow_IFilterGraph, dshow_IGraphBuilder, dshow_IPin};
use crate::vt::IGraphBuilderVT;

/// [`IFilterGraph2`](crate::IFilterGraph2) virtual table.
#[repr(C)]
pub struct IFilterGraph2VT {
	pub IGraphBuilderVT: IGraphBuilderVT,
	pub AddSourceFilterForMoniker: fn(ComPtr, ComPtr, ComPtr, PCSTR, *mut ComPtr) -> HRES,
	pub ReconnectEx: fn(ComPtr, ComPtr, PCVOID) -> HRES,
	pub RenderEx: fn(ComPtr, ComPtr, u32, *mut u32) -> HRES,
}

com_interface! { IFilterGraph2: "36b73882-c2c8-11cf-8b46-00805f6cef60";
	/// [`IFilterGraph2`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifiltergraph2)
	/// COM interface over [`IFilterGraph2VT`](crate::vt::IFilterGraph2VT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IFilterGraph for IFilterGraph2 {}
impl dshow_IGraphBuilder for IFilterGraph2 {}
impl dshow_IFilterGraph2 for IFilterGraph2 {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IFilterGraph2`](crate::IFilterGraph2).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IFilterGraph2: dshow_IGraphBuilder {
	/// [`IFilterGraph2::ReconnectEx`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph2-reconnectex)
	/// method.
	fn ReconnectEx(&self,
		pin: &impl dshow_IPin, mt: Option<&AM_MEDIA_TYPE>) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<IFilterGraph2VT>();
			ok_to_hrresult(
				(vt.ReconnectEx)(
					self.ptr(),
					pin.ptr(),
					mt.map_or(std::ptr::null_mut(), |mt| mt as *const _ as _),
				),
			)
		}
	}
}
