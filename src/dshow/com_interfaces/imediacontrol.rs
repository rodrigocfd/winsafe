#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::ffi_types::{HRES, PSTR};
use crate::kernel::decl::WString;
use crate::kernel::privs::INFINITE;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::oleaut::decl::IDispatch;
use crate::prelude::oleaut_IDispatch;
use crate::vt::IDispatchVT;

/// [`IMediaControl`](crate::IMediaControl) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
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
/// COM interface over [`IMediaControlVT`](crate::vt::IMediaControlVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{IGraphBuilder, IMediaControl};
///
/// let graph_builder: IGraphBuilder; // initialized somewhere
/// # use winsafe::{co::CLSID, co::CLSCTX, CoCreateInstance};
/// # let graph_builder = CoCreateInstance::<IGraphBuilder>(&CLSID::new("00000000-0000-0000-0000-000000000000"), None, CLSCTX::INPROC_SERVER)?;
///
/// let media_control = graph_builder
///     .QueryInterface::<IMediaControl>()?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub struct IMediaControl(ComPtr);

impl_iunknown!(IMediaControl, "56a868b1-0ad4-11ce-b03a-0020af0ba770");
impl oleaut_IDispatch for IMediaControl {}
impl dshow_IMediaControl for IMediaControl {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IMediaControl`](crate::IMediaControl).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait dshow_IMediaControl: oleaut_IDispatch {
	/// [`IMediaControl::AddSourceFilter`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-addsourcefilter)
	/// method.
	#[must_use]
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
	#[must_use]
	fn GetState(&self,
		ms_timeout: Option<i32>) -> HrResult<co::FILTER_STATE>
	{
		let mut state = co::FILTER_STATE::Stopped;
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
