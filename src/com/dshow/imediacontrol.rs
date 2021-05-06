#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::{IDispatch, IDispatchVT, IUnknownVT, PPComVT};
use crate::com::dshow::vt::IMediaControlVT;
use crate::privs::{hr_to_winresult, hr_to_winresult_bool, INFINITE};
use crate::WString;

macro_rules! IMediaControl_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IDispatch_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IMediaControl::AddSourceFilter`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-addsourcefilter)
			/// method.
			pub fn AddSourceFilter(&self, fileName: &str) -> WinResult<IDispatch> {
				let ppvt = unsafe { self.ppvt::<IMediaControlVT>() };
				let mut ppvQueried: PPComVT<IDispatchVT> = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						((**ppvt).AddSourceFilter)(
							ppvt,
							WString::from_str(fileName).as_mut_ptr(), // BSTR
							&mut ppvQueried as *mut _ as _,
						)
					},
				).map(|_| IDispatch::from(ppvQueried))
			}

			/// [`IMediaControl::GetState`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-getstate)
			/// method.
			pub fn GetState(&self,
				msTimeout: Option<i32>) -> WinResult<co::FILTER_STATE>
			{
				let ppvt = unsafe { self.ppvt::<IMediaControlVT>() };
				let mut state = co::FILTER_STATE::Stopped;
				hr_to_winresult(
					unsafe {
						((**ppvt).GetState)(
							ppvt,
							msTimeout.unwrap_or(INFINITE as _),
							&mut state.0,
						)
					},
				).map(|_| state)
			}

			/// [`IMediaControl::Pause`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-pause)
			/// method.
			pub fn Pause(&self) -> WinResult<bool> {
				let ppvt = unsafe { self.ppvt::<IMediaControlVT>() };
				hr_to_winresult_bool(unsafe { ((**ppvt).Pause)(ppvt) })
			}

			/// [`IMediaControl::RenderFile`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-renderfile)
			/// method.
			pub fn RenderFile(&self, fileName: &str) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IMediaControlVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).RenderFile)(
							ppvt,
							WString::from_str(fileName).as_mut_ptr(), // BSTR
						)
					},
				)
			}

			/// [`IMediaControl::Run`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-run)
			/// method.
			pub fn Run(&self) -> WinResult<bool> {
				let ppvt = unsafe { self.ppvt::<IMediaControlVT>() };
				hr_to_winresult_bool(unsafe { ((**ppvt).Run)(ppvt) })
			}

			/// [`IMediaControl::Stop`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stop)
			/// method.
			pub fn Stop(&self) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IMediaControlVT>() };
				hr_to_winresult(unsafe { ((**ppvt).Stop)(ppvt) })
			}

			/// [`IMediaControl::StopWhenReady`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stopwhenready)
			/// method.
			pub fn StopWhenReady(&self) -> WinResult<bool> {
				let ppvt = unsafe { self.ppvt::<IMediaControlVT>() };
				hr_to_winresult_bool(unsafe { ((**ppvt).StopWhenReady)(ppvt) })
			}
		}
	};
}

IMediaControl_impl! {
	/// [`IMediaControl`](https://docs.microsoft.com/en-us/windows/win32/api/control/nn-control-imediacontrol)
	/// COM interface over
	/// [`IMediaControlVT`](crate::dshow::vt::IMediaControlVT). Inherits from
	/// [`IDispatch`](crate::IDispatch) [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IMediaControl, IMediaControlVT
}
