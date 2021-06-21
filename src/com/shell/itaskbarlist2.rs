#![allow(non_snake_case)]

macro_rules! pub_struct_ITaskbarList2 {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::shell::vt::ITaskbarList2VT;

		pub_struct_ITaskbarList!{
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn itaskbarlist2_vt(&self) -> &ITaskbarList2VT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`ITaskbarList2::MarkFullscreenWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist2-markfullscreenwindow)
			/// method.
			pub fn MarkFullscreenWindow(&self,
				hwnd: HWND, fFullscreen: bool) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist2_vt().MarkFullscreenWindow)(
						self.ppvt,
						hwnd.ptr,
						fFullscreen as _,
					),
				)
			}
		}
	};
}

pub_struct_ITaskbarList2! {
	/// [`ITaskbarList2`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist2)
	/// COM interface over
	/// [`ITaskbarList2VT`](crate::shell::vt::ITaskbarList2VT). Inherits from
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
	/// let obj: shell::ITaskbarList2 = CoCreateInstance(
	///     &shell::clsid::TaskbarList,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// ).unwrap();
	/// ```
	ITaskbarList2, crate::com::shell::vt::ITaskbarList2VT
}