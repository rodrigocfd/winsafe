#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknownVT, PPComVT};
use crate::ffi::{HRESULT, PCVOID, PVOID};
use crate::privs::hr_to_winresult;

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

macro_rules! IDispatch_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IDispatch::GetTypeInfoCount`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfocount)
			/// method.
			pub fn GetTypeInfoCount(&self) -> WinResult<u32> {
				let ppvt = unsafe { self.ppvt::<IDispatchVT>() };
				let mut count: u32 = 0;
				hr_to_winresult(
					unsafe { ((**ppvt).GetTypeInfoCount)(ppvt, &mut count) },
				).map(|_| count)
			}
		}
	};
}

IDispatch_impl! {
	/// [`IDispatch`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-idispatch)
	/// COM interface over [`IDispatchVT`](crate::IDispatchVT). Inherits from
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IDispatch, IDispatchVT
}
