#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co::ERROR;
use crate::com::{PPVtbl, Vtbl};
use crate::com::shell::{ITaskbarList, ITaskbarListVtbl};
use crate::ffi::{BOOL, HANDLE};
use crate::handles::HWND;
use crate::structs::IID;

/// [`ITaskbarList2`](crate::shell::ITaskbarList2) virtual table.
#[repr(C)]
pub struct ITaskbarList2Vtbl {
	iTaskbarListVtbl: ITaskbarListVtbl,

	MarkFullscreenWindow: fn(PPVtbl<Self>, HANDLE, BOOL) -> u32,
}

impl_iid!(ITaskbarList2Vtbl, 0x602d4995, 0xb13a, 0x429b, 0xa66e, 0x1935e44f4317);

//------------------------------------------------------------------------------

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
/// let mut obj: shell::ITaskbarList2 = CoCreateInstance(
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

impl From<PPVtbl<ITaskbarList2Vtbl>> for ITaskbarList2 {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: PPVtbl<ITaskbarList2Vtbl>) -> Self {
		Self {
			ITaskbarList: ITaskbarList::from(ppv as PPVtbl<ITaskbarListVtbl>),
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
			let ppv = self.ITaskbarList.IUnknown.ppv::<ITaskbarList2Vtbl>();
			into_result!(
				((**ppv).MarkFullscreenWindow)(ppv, hwnd.ptr, fFullscreen as i32)
			)
		}
	}
}
