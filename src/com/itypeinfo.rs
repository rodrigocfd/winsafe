#![allow(non_snake_case)]

macro_rules! ITypeInfo_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::IUnknown;
		use crate::com::vt::ITypeInfoVT;

		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			ppvt_conv!(itypeinfo_vt, ITypeInfoVT);

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
						ref_as_pcvoid(&VT::IID()),
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| RetInterf::from(ppvQueried))
			}
		}
	};
}

ITypeInfo_impl! {
	/// [`ITypeInfo`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-itypeinfo)
	/// COM interface over [`ITypeInfoVT`](crate::ITypeInfoVT). Inherits from
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	ITypeInfo, crate::com::vt::ITypeInfoVT
}
