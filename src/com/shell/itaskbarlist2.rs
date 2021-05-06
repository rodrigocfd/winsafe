#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::shell::vt::{ITaskbarListVT, ITaskbarList2VT};
use crate::handles::HWND;
use crate::privs::hr_to_winresult;

macro_rules! ITaskbarList2_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		ITaskbarList_impl!{
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`ITaskbarList2::MarkFullscreenWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist2-markfullscreenwindow)
			/// method.
			pub fn MarkFullscreenWindow(&self,
				hwnd: HWND, fFullscreen: bool) -> WinResult<()>
			{
				let ppvt = unsafe { self.ppvt::<ITaskbarList2VT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).MarkFullscreenWindow)(
							ppvt, hwnd.ptr, fFullscreen as _,
						)
					},
				)
			}
		}
	};
}

ITaskbarList2_impl! {
	/// [`ITaskbarList2`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist2)
	/// COM interface.
	///
	/// Virtual table: [`ITaskbarList2VT`](crate::shell::vt::ITaskbarList2VT).
	///
	/// Inherits from:
	/// * [`ITaskbarList`](crate::shell::ITaskbarList);
	/// * [`IUnknown`](crate::IUnknown).
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
	ITaskbarList2, ITaskbarList2VT
}
