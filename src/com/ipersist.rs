#![allow(non_snake_case)]

macro_rules! pub_struct_IPersist {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::vt::IPersistVT;
		use crate::structs::CLSID;

		pub_struct_IUnknown! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn ipersist_vt(&self) -> &IPersistVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IPersist::GetClassID`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersist-getclassid)
			/// method.
			pub fn GetClassID(&self) -> WinResult<CLSID> {
				let mut clsid = CLSID::new(0, 0, 0, 0, 0);
				hr_to_winresult(
					(self.ipersist_vt().GetClassID)(
						self.ppvt,
						&mut clsid as *mut _ as _,
					),
				).map(|_| clsid)
			}
		}
	};
}

pub_struct_IPersist! {
	/// [`IPersist`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist)
	/// COM interface over [`IPersistVT`](crate::IPersistVT). Inherits from
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IPersist, crate::com::vt::IPersistVT
}
