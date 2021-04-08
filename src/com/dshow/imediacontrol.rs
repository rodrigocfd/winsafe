#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::{IDispatch, IDispatchVT, PPComVT};
use crate::com::dshow::vt::IMediaControlVT;
use crate::com::funcs::{hr_to_winresult, hr_to_winresult_bool};
use crate::privs::INFINITE;
use crate::WString;

/// [`IMediaControl`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ibasefilter)
/// COM interface. Backed by
/// [`IMediaControlVT`](crate::dshow::vt::IMediaControlVT) virtual table.
///
/// Inherits from:
/// * [`IDispatch`](crate::IDispatch);
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IMediaControl {
	/// Methods of base interface [`IDispatch`](crate::IDispatch).
	pub IDispatch: IDispatch,
}

impl From<PPComVT<IMediaControlVT>> for IMediaControl {
	fn from(ppv: PPComVT<IMediaControlVT>) -> Self {
		Self {
			IDispatch: IDispatch::from(ppv as PPComVT<IDispatchVT>)
		}
	}
}

impl IMediaControl {
	unsafe fn ppv(&self) -> PPComVT<IMediaControlVT> {
		self.IDispatch.IUnknown.ppv::<IMediaControlVT>()
	}

	/// [`IMediaControl::AddSourceFilter`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-addsourcefilter)
	/// method.
	pub fn AddSourceFilter(&self, fileName: &str) -> WinResult<IDispatch> {
		let mut ppvQueried: PPComVT<IDispatchVT> = std::ptr::null_mut();
		hr_to_winresult(
			unsafe {
				((**self.ppv()).AddSourceFilter)(
					self.ppv(),
					WString::from_str(fileName).as_mut_ptr(), // BSTR
					&mut ppvQueried as *mut _ as *mut _,
				)
			},
		).map(|_| IDispatch::from(ppvQueried))
	}

	/// [`IMediaControl::GetState`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-getstate)
	/// method.
	pub fn GetState(&self,
		msTimeout: Option<i32>) -> WinResult<co::FILTER_STATE>
	{
		let mut state = co::FILTER_STATE::Stopped;
		hr_to_winresult(
			unsafe {
				((**self.ppv()).GetState)(
					self.ppv(),
					msTimeout.unwrap_or(INFINITE as i32),
					&mut state.0,
				)
			},
		).map(|_| state)
	}

	/// [`IMediaControl::Pause`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-pause)
	/// method.
	pub fn Pause(&self) -> WinResult<bool> {
		hr_to_winresult_bool(unsafe { ((**self.ppv()).Pause)(self.ppv()) })
	}

	/// [`IMediaControl::RenderFile`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-renderfile)
	/// method.
	pub fn RenderFile(&self, fileName: &str) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).RenderFile)(
					self.ppv(),
					WString::from_str(fileName).as_mut_ptr(), // BSTR
				)
			},
		)
	}

	/// [`IMediaControl::Run`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-run)
	/// method.
	pub fn Run(&self) -> WinResult<bool> {
		hr_to_winresult_bool(unsafe { ((**self.ppv()).Run)(self.ppv()) })
	}

	/// [`IMediaControl::Stop`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stop)
	/// method.
	pub fn Stop(&self) -> WinResult<()> {
		hr_to_winresult(unsafe { ((**self.ppv()).Stop)(self.ppv()) })
	}

	/// [`IMediaControl::StopWhenReady`](https://docs.microsoft.com/en-us/windows/win32/api/control/nf-control-imediacontrol-stopwhenready)
	/// method.
	pub fn StopWhenReady(&self) -> WinResult<bool> {
		hr_to_winresult_bool(
			unsafe { ((**self.ppv()).StopWhenReady)(self.ppv()) },
		)
	}
}
