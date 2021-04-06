#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co::ERROR;
use crate::com::PPComVT;
use crate::com::shell::ITaskbarList;
use crate::com::shell::vt::{ITaskbarListVT, ITaskbarList2VT};
use crate::handles::HWND;

/// [`ITaskbarList2`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist2)
/// COM interface.
///
/// Inherits from:
/// * [`ITaskbarList`](crate::shell::ITaskbarList);
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls [`IUnknown::Release`](crate::IUnknown::Release) when the
/// object goes out of scope.
///
/// # Examples
///
/// Usually instantiated with [`CoCreateInstance`](crate::CoCreateInstance):
/// ```rust,ignore
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj: shell::ITaskbarList2 = CoCreateInstance(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
pub struct ITaskbarList2 {
	/// Methods of base interface
	/// [`ITaskbarList`](crate::shell::ITaskbarList).
	pub ITaskbarList: ITaskbarList,
}

impl From<PPComVT<ITaskbarList2VT>> for ITaskbarList2 {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: PPComVT<ITaskbarList2VT>) -> Self {
		Self {
			ITaskbarList: ITaskbarList::from(ppv as PPComVT<ITaskbarListVT>),
		}
	}
}

impl ITaskbarList2 {
	/// [`ITaskbarList2::MarkFullscreenWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist2-markfullscreenwindow)
	/// method.
	pub fn MarkFullscreenWindow(&self,
		hwnd: HWND, fFullscreen: bool) -> WinResult<()>
	{
		unsafe {
			let ppv = self.ITaskbarList.IUnknown.ppv::<ITaskbarList2VT>();
			into_result!(
				((**ppv).MarkFullscreenWindow)(ppv, hwnd.ptr, fFullscreen as i32)
			)
		}
	}
}
