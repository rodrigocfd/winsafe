#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dshow::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMediaControl: "56a868b1-0ad4-11ce-b03a-0020af0ba770";
	/// [`IMediaControl`](https://learn.microsoft.com/en-us/windows/win32/api/control/nn-control-imediacontrol)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let graph_builder: w::IGraphBuilder; // initialized somewhere
	/// # let graph_builder = unsafe { w::IGraphBuilder::null() };
	///
	/// let media_control = graph_builder
	///     .QueryInterface::<w::IMediaControl>()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl oleaut_IDispatch for IMediaControl {}
impl dshow_IMediaControl for IMediaControl {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IMediaControl`](crate::IMediaControl).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IMediaControl: oleaut_IDispatch {
	/// [`IMediaControl::AddSourceFilter`](https://learn.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-addsourcefilter)
	/// method.
	#[must_use]
	fn AddSourceFilter(&self, file_name: &str) -> HrResult<IDispatch> {
		let mut queried = unsafe { IDispatch::null() };
		HrRet(unsafe {
			(vt::<IMediaControlVT>(self).AddSourceFilter)(
				self.ptr(),
				WString::from_str(file_name).as_mut_ptr(), // BSTR
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IMediaControl::GetState`](https://learn.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-getstate)
	/// method.
	#[must_use]
	fn GetState(&self, ms_timeout: Option<i32>) -> HrResult<co::FILTER_STATE> {
		let mut state = co::FILTER_STATE::default();
		HrRet(unsafe {
			(vt::<IMediaControlVT>(self).GetState)(
				self.ptr(),
				ms_timeout.unwrap_or(INFINITE as _),
				state.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| state)
	}

	/// [`IMediaControl::Pause`](https://learn.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-pause)
	/// method.
	fn Pause(&self) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IMediaControlVT>(self).Pause)(self.ptr()) }).to_bool_hrresult()
	}

	fn_com_bstr_set! { RenderFile: IMediaControlVT, file_name;
		/// [`IMediaControl::RenderFile`](https://learn.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-renderfile)
		/// method.
	}

	/// [`IMediaControl::Run`](https://learn.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-run)
	/// method.
	fn Run(&self) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IMediaControlVT>(self).Run)(self.ptr()) }).to_bool_hrresult()
	}

	fn_com_noparm! { Stop: IMediaControlVT;
		/// [`IMediaControl::Stop`](https://learn.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stop)
		/// method.
	}

	/// [`IMediaControl::StopWhenReady`](https://learn.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stopwhenready)
	/// method.
	fn StopWhenReady(&self) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IMediaControlVT>(self).StopWhenReady)(self.ptr()) }).to_bool_hrresult()
	}
}
