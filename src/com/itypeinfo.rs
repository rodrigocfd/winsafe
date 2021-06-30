#![allow(non_snake_case)]

macro_rules! pub_struct_ITypeInfo {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::IUnknown;
		use crate::com::vt::ITypeInfoVT;

		pub_struct_IUnknown! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn itypeinfo_vt(&self) -> &ITypeInfoVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`ITypeInfo::CreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-itypeinfo-createinstance)
			/// method.
			pub fn CreateInstance<VT: ComVT, RetInterf: From<PPComVT<VT>>>(&self,
				pUnkOuter: Option<&mut IUnknown>) -> WinResult<RetInterf>
			{
				let mut ppvQueried: PPComVT<VT> = std::ptr::null_mut();
				let mut ppvOuter: PPComVT<IUnknownVT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.itypeinfo_vt().CreateInstance)(
						self.ppvt,
						pUnkOuter.as_ref()
							.map_or(std::ptr::null_mut(), |_| &mut ppvOuter as *mut _ as _),
						&VT::IID as *const _ as _,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| RetInterf::from(ppvQueried))
			}
		}
	};
}

pub_struct_ITypeInfo! {
	/// [`ITypeInfo`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-itypeinfo)
	/// COM interface over [`ITypeInfoVT`](crate::ITypeInfoVT). Inherits from
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	ITypeInfo, crate::com::vt::ITypeInfoVT
}
