#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::autom::idispatch::{IDispatch, IDispatchT, IDispatchVT};
use crate::com::dshow;
use crate::com::iunknown::ComPtr;
use crate::ffi::{HRESULT, PSTR};
use crate::privs::{hr_to_winresult, hr_to_winresult_bool, INFINITE};
use crate::various::WString;

/// [`IMediaControl`](crate::dshow::IMediaControl) virtual table.
pub struct IMediaControlVT {
	pub IDispatchVT: IDispatchVT,
	pub Run: fn(ComPtr) -> HRESULT,
	pub Pause: fn(ComPtr) -> HRESULT,
	pub Stop: fn(ComPtr) -> HRESULT,
	pub GetState: fn(ComPtr, i32, *mut u32) -> HRESULT,
	pub RenderFile: fn(ComPtr, PSTR) -> HRESULT,
	pub AddSourceFilter: fn(ComPtr, PSTR, *mut ComPtr) -> HRESULT,
	pub GetFilterCollection: fn(ComPtr, *mut ComPtr) -> HRESULT,
	pub GetRegFilterCollection: fn(ComPtr, *mut ComPtr) -> HRESULT,
	pub StopWhenReady: fn(ComPtr) -> HRESULT,
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
	fn AddSourceFilter(&self, file_name: &str) -> WinResult<IDispatch> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			hr_to_winresult(
				(vt.AddSourceFilter)(
					self.ptr(),
					WString::from_str(file_name).as_mut_ptr(), // BSTR
					&mut ppv_queried as *mut _ as _,
				),
			)
		}.map(|_| IDispatch::from(ppv_queried))
	}

	/// [`IMediaControl::GetState`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-getstate)
	/// method.
	fn GetState(&self,
		ms_timeout: Option<i32>) -> WinResult<dshow::co::FILTER_STATE>
	{
		let mut state = dshow::co::FILTER_STATE::Stopped;
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			hr_to_winresult(
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
	fn Pause(&self) -> WinResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			hr_to_winresult_bool((vt.Pause)(self.ptr()))
		}
	}

	/// [`IMediaControl::RenderFile`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-renderfile)
	/// method.
	fn RenderFile(&self, file_name: &str) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			hr_to_winresult(
				(vt.RenderFile)(
					self.ptr(),
					WString::from_str(file_name).as_mut_ptr(), // BSTR
				),
			)
		}
	}

	/// [`IMediaControl::Run`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-run)
	/// method.
	fn Run(&self) -> WinResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			hr_to_winresult_bool((vt.Run)(self.ptr()))
		}
	}

	/// [`IMediaControl::Stop`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stop)
	/// method.
	fn Stop(&self) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			hr_to_winresult((vt.Stop)(self.ptr()))
		}
	}

	/// [`IMediaControl::StopWhenReady`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stopwhenready)
	/// method.
	fn StopWhenReady(&self) -> WinResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IMediaControlVT);
			hr_to_winresult_bool((vt.StopWhenReady)(self.ptr()))
		}
	}
}
