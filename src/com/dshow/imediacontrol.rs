#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::autom::idispatch::{IDispatch, IDispatchT, IDispatchVT};
use crate::com::dshow;
use crate::com::iunknown::ComPtr;
use crate::ffi::{HRES, PSTR};
use crate::privs::{ok_to_hrresult, okfalse_to_hrresult, INFINITE};
use crate::various::WString;

/// [`IMediaControl`](crate::dshow::IMediaControl) virtual table.
#[repr(C)]
pub struct IMediaControlVT {
	pub IDispatchVT: IDispatchVT,
	pub Run: fn(ComPtr) -> HRES,
	pub Pause: fn(ComPtr) -> HRES,
	pub Stop: fn(ComPtr) -> HRES,
	pub GetState: fn(ComPtr, i32, *mut u32) -> HRES,
	pub RenderFile: fn(ComPtr, PSTR) -> HRES,
	pub AddSourceFilter: fn(ComPtr, PSTR, *mut ComPtr) -> HRES,
	pub GetFilterCollection: fn(ComPtr, *mut ComPtr) -> HRES,
	pub GetRegFilterCollection: fn(ComPtr, *mut ComPtr) -> HRES,
	pub StopWhenReady: fn(ComPtr) -> HRES,
}

/// [`IMediaControl`](https://docs.microsoft.com/en-us/windows/win32/api/control/nn-control-imediacontrol)
/// COM interface over [`IMediaControlVT`](crate::dshow::vt::IMediaControlVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::prelude::*;
/// use winsafe::dshow;
///
/// let graph_builder: dshow::IGraphBuilder; // initialized somewhere
///
/// let media_control = graph_builder
///     .QueryInterface::<dshow::IMediaControl>()?;
/// ```
pub struct IMediaControl(ComPtr);

impl_iunknown!(IMediaControl, 0x56a868b1, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
impl IDispatchT for IMediaControl {}
impl IMediaControlT for IMediaControl {}

/// Exposes the [`IMediaControl`](crate::dshow::IMediaControl) methods.
pub trait IMediaControlT: IDispatchT {
	/// [`IMediaControl::AddSourceFilter`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-addsourcefilter)
	/// method.
	fn AddSourceFilter(&self, file_name: &str) -> HrResult<IDispatch> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			ok_to_hrresult(
				(vt.AddSourceFilter)(
					self.ptr(),
					WString::from_str(file_name).as_mut_ptr(), // BSTR
					&mut ppv_queried,
				),
			)
		}.map(|_| IDispatch::from(ppv_queried))
	}

	/// [`IMediaControl::GetState`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-getstate)
	/// method.
	fn GetState(&self,
		ms_timeout: Option<i32>) -> HrResult<dshow::co::FILTER_STATE>
	{
		let mut state = dshow::co::FILTER_STATE::Stopped;
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			ok_to_hrresult(
				(vt.GetState)(
					self.ptr(),
					ms_timeout.unwrap_or(INFINITE as _),
					&mut state.0,
				),
			)
		}.map(|_| state)
	}

	/// [`IMediaControl::Pause`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-pause)
	/// method.
	fn Pause(&self) -> HrResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			okfalse_to_hrresult((vt.Pause)(self.ptr()))
		}
	}

	/// [`IMediaControl::RenderFile`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-renderfile)
	/// method.
	fn RenderFile(&self, file_name: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			ok_to_hrresult(
				(vt.RenderFile)(
					self.ptr(),
					WString::from_str(file_name).as_mut_ptr(), // BSTR
				),
			)
		}
	}

	/// [`IMediaControl::Run`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-run)
	/// method.
	fn Run(&self) -> HrResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			okfalse_to_hrresult((vt.Run)(self.ptr()))
		}
	}

	/// [`IMediaControl::Stop`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stop)
	/// method.
	fn Stop(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			ok_to_hrresult((vt.Stop)(self.ptr()))
		}
	}

	/// [`IMediaControl::StopWhenReady`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stopwhenready)
	/// method.
	fn StopWhenReady(&self) -> HrResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			okfalse_to_hrresult((vt.StopWhenReady)(self.ptr()))
		}
	}
}
