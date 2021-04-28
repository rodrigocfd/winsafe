#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknown, IUnknownVT, PPComVT};
use crate::com::dshow::vt::IPinVT;
use crate::com::funcs::CoTaskMemFree;
use crate::privs::hr_to_winresult;
use crate::WString;

/// [`IPin`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ipin)
/// COM interface.
///
/// Virtual table: [`IPinVT`](crate::dshow::vt::IPinVT).
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IPin {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<IPinVT>> for IPin {
	fn from(ppv: PPComVT<IPinVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl IPin {
	unsafe fn ppv(&self) -> PPComVT<IPinVT> {
		self.IUnknown.ppv::<IPinVT>()
	}

	/// [`IPin::BeginFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-beginflush)
	/// method.
	pub fn BeginFlush(&self) -> WinResult<()> {
		hr_to_winresult( unsafe { ((**self.ppv()).BeginFlush)(self.ppv()) })
	}

	/// [`IPin::ConnectedTo`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectedto)
	/// method.
	pub fn ConnectedTo(&self) -> WinResult<IPin> {
		let mut ppvQueried: PPComVT<IPinVT> = std::ptr::null_mut();
		hr_to_winresult(
			unsafe {
				((**self.ppv()).ConnectedTo)(
					self.ppv(),
					&mut ppvQueried as *mut _ as _,
				)
			},
		).map(|_| IPin::from(ppvQueried))
	}

	/// [`IPin::Disconnect`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-disconnect)
	/// method.
	pub fn Disconnect(&self) -> WinResult<()> {
		hr_to_winresult( unsafe { ((**self.ppv()).Disconnect)(self.ppv()) })
	}

	/// [`IPin::EndFlush`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endflush)
	/// method.
	pub fn EndFlush(&self) -> WinResult<()> {
		hr_to_winresult( unsafe { ((**self.ppv()).EndFlush)(self.ppv()) })
	}

	/// [`IPin::EndOfStream`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endofstream)
	/// method.
	pub fn EndOfStream(&self) -> WinResult<()> {
		hr_to_winresult( unsafe { ((**self.ppv()).EndOfStream)(self.ppv()) })
	}

	/// [`IPin::QueryId`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryid)
	/// method.
	pub fn QueryId(&self) -> WinResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		hr_to_winresult(
			unsafe { ((**self.ppv()).QueryId)(self.ppv(), &mut pstr) },
		).map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr);
			name.to_string()
		})
	}
}
