#![allow(non_snake_case)]

macro_rules! pub_struct_ITaskbarList4 {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::shell::vt::ITaskbarList4VT;

		pub_struct_ITaskbarList3!{
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn itaskbarlist4_vt(&self) -> &ITaskbarList4VT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`ITaskbarList4::SetTabProperties`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist4-settabproperties)
			/// method.
			pub fn SetTabProperties(&self,
				hwndTab: HWND, stpFlags: shellco::STPFLAG) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist4_vt().SetTabProperties)(
						self.ppvt,
						hwndTab.ptr,
						stpFlags.0,
					),
				)
			}
		}
	};
}

pub_struct_ITaskbarList4! {
	/// [`ITaskbarList4`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist4)
	/// COM interface over
	/// [`ITaskbarList4VT`](crate::shell::vt::ITaskbarList4VT). Inherits from
	/// [`ITaskbarList3`](crate::shell::ITaskbarList3),
	/// [`ITaskbarList2`](crate::shell::ITaskbarList2),
	/// [`ITaskbarList`](crate::shell::ITaskbarList),
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, CoCreateInstance, shell};
	///
	/// let obj: shell::ITaskbarList4 = CoCreateInstance(
	///     &shell::clsid::TaskbarList,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// ).unwrap();
	/// ```
	ITaskbarList4, crate::com::shell::vt::ITaskbarList4VT
}
