#![allow(non_snake_case)]

macro_rules! IDispatch_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::ITypeInfo;
		use crate::com::vt::{IDispatchVT, ITypeInfoVT};
		use crate::structs::LCID;

		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn idispatch_vt(&self) -> &IDispatchVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IDispatch::GetTypeInfoCount`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfocount)
			/// method.
			pub fn GetTypeInfoCount(&self) -> WinResult<u32> {
				let mut count: u32 = 0;
				hr_to_winresult(
					(self.idispatch_vt().GetTypeInfoCount)(self.ppvt, &mut count),
				).map(|_| count)
			}

			/// [`IDispatch::GetTypeInfo`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfo)
			/// method.
			pub fn GetTypeInfo(&self, iTInfo: u32, lcid: LCID) -> WinResult<ITypeInfo> {
				let mut ppvQueried: PPComVT<ITypeInfoVT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.idispatch_vt().GetTypeInfo)(
						self.ppvt,
						iTInfo,
						lcid.0,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| ITypeInfo::from(ppvQueried))
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
	IDispatch, crate::com::vt::IDispatchVT
}
