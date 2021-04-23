#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{ComVT, IUnknown, IUnknownVT, PPComVT};
use crate::com::funcs::hr_to_winresult;
use crate::ffi::{HRESULT, PCVOID, PVOID};
use crate::structs::IID;

com_virtual_table! { IDispatchVT,
	/// [`IDispatch`](crate::IDispatch) virtual table.
	->
	0x00020400, 0x0000, 0x0000, 0xc000, 0x000000000046,
	IUnknownVT, IUnknownVT

	GetTypeInfoCount, fn(PPComVT<Self>, *mut u32) -> HRESULT
	GetTypeInfo, fn(PPComVT<Self>, u32, u32, *mut PPComVT<IUnknownVT>) -> HRESULT
	GetIDsOfNames, fn(PPComVT<Self>, PCVOID, PVOID, u32, u32, PVOID) -> HRESULT
	Invoke, fn(PPComVT<Self>, i32, PCVOID, u32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRESULT
}

/// [`IDispatch`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-idispatch)
/// COM interface. Backed by [`IDispatchVT`](crate::IDispatchVT) virtual table.
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IDispatch {
/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<IDispatchVT>> for IDispatch {
	fn from(ppv: PPComVT<IDispatchVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl IDispatch {
	unsafe fn ppv(&self) -> PPComVT<IDispatchVT> {
		self.IUnknown.ppv::<IDispatchVT>()
	}

	/// [`IDispatch::GetTypeInfoCount`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfocount)
	/// method.
	pub fn GetTypeInfoCount(&self) -> WinResult<u32> {
		let mut count: u32 = 0;
		hr_to_winresult(
			unsafe { ((**self.ppv()).GetTypeInfoCount)(self.ppv(), &mut count) },
		).map(|_| count)
	}
}
