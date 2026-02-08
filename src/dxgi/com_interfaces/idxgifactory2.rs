#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dxgi::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIFactory2: "50c83a1c-e072-4c48-87b0-3630fa36a6d0";
	/// [`IDXGIFactory2`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nn-dxgi1_2-idxgifactory2)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with [`CreateDXGIFactory2`](crate::CreateDXGIFactory2)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, co, prelude::*};
	///
	/// let factory2 = w::CreateDXGIFactory2(co::DXGI_CREATE_FACTORY::NoValue)?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl dxgi_IDXGIObject for IDXGIFactory2 {}
impl dxgi_IDXGIFactory for IDXGIFactory2 {}
impl dxgi_IDXGIFactory1 for IDXGIFactory2 {}
impl dxgi_IDXGIFactory2 for IDXGIFactory2 {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIFactory2`](crate::IDXGIFactory2).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIFactory2: dxgi_IDXGIFactory1 {
	/// [`IsWindowedStereoEnabled`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-iswindowedstereoenabled)
	/// method.
	fn IsWindowedStereoEnabled(&self) -> bool {
		unsafe { (vt::<IDXGIFactory2VT>(self).IsWindowedStereoEnabled)(self.ptr()) != 0 }
	}

	/// [`RegisterOcclusionStatusEvent`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-registerocclusionstatusevent)
	/// method.
	fn RegisterOcclusionStatusEvent(&self, hevent: &HEVENT) -> HrResult<u32> {
		let mut cookie = 0u32;
		HrRet(unsafe {
			(vt::<IDXGIFactory2VT>(self).RegisterOcclusionStatusEvent)(
				self.ptr(),
				hevent.ptr(),
				&mut cookie,
			)
		})
		.to_hrresult()
		.map(|_| cookie)
	}

	/// [`RegisterOcclusionStatusWindow`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-registerocclusionstatuswindow)
	/// method.
	fn RegisterOcclusionStatusWindow(&self, hwnd: &HWND, msg: co::WM) -> HrResult<u32> {
		let mut cookie = 0u32;
		HrRet(unsafe {
			(vt::<IDXGIFactory2VT>(self).RegisterOcclusionStatusWindow)(
				self.ptr(),
				hwnd.ptr(),
				msg.raw(),
				&mut cookie,
			)
		})
		.to_hrresult()
		.map(|_| cookie)
	}

	/// [`RegisterStereoStatusEvent`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-registerstereostatusevent)
	/// method.
	fn RegisterStereoStatusEvent(&self, hevent: &HEVENT) -> HrResult<u32> {
		let mut cookie = 0u32;
		HrRet(unsafe {
			(vt::<IDXGIFactory2VT>(self).RegisterStereoStatusEvent)(
				self.ptr(),
				hevent.ptr(),
				&mut cookie,
			)
		})
		.to_hrresult()
		.map(|_| cookie)
	}

	/// [`RegisterStereoStatusWindow`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-registerstereostatuswindow)
	/// method.
	fn RegisterStereoStatusWindow(&self, hwnd: &HWND, msg: co::WM) -> HrResult<u32> {
		let mut cookie = 0u32;
		HrRet(unsafe {
			(vt::<IDXGIFactory2VT>(self).RegisterStereoStatusWindow)(
				self.ptr(),
				hwnd.ptr(),
				msg.raw(),
				&mut cookie,
			)
		})
		.to_hrresult()
		.map(|_| cookie)
	}

	/// [`UnregisterOcclusionStatus`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-unregisterocclusionstatus)
	/// method.
	fn UnregisterOcclusionStatus(&self, cookie: u32) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGIFactory2VT>(self).UnregisterOcclusionStatus)(self.ptr(), cookie)
		})
		.to_hrresult()
	}

	/// [`UnregisterStereoStatus`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-unregisterstereostatus)
	/// method.
	fn UnregisterStereoStatus(&self, cookie: u32) -> HrResult<()> {
		HrRet(unsafe { (vt::<IDXGIFactory2VT>(self).UnregisterStereoStatus)(self.ptr(), cookie) })
			.to_hrresult()
	}
}
