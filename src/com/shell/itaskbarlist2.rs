#![allow(non_snake_case)]

use crate::com::shell::vt::ITaskbarListVT;
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::{BOOL, HANDLE, HRESULT};
use crate::structs::IID;

/// [`ITaskbarList2`](crate::shell::ITaskbarList2) virtual table.
pub struct ITaskbarList2VT {
	pub ITaskbarListVT: ITaskbarListVT,
	pub MarkFullscreenWindow: fn(PPVT, HANDLE, BOOL) -> HRESULT,
}

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
/// use winsafe::prelude::*;
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj = CoCreateInstance::<shell::ITaskbarList2>(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct ITaskbarList2 {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for ITaskbarList2 {
	const IID: IID = IID::new(0x602d4995, 0xb13a, 0x429b, 0xa66e, 0x1935e44f4317);
}

macro_rules! impl_ITaskbarList2 {
	($name:ty, $vt:ty) => {
		impl $name {
			fn itaskbarlist2_vt(&self) -> &ITaskbarList2VT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`ITaskbarList2::MarkFullscreenWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist2-markfullscreenwindow)
			/// method.
			pub fn MarkFullscreenWindow(&self,
				hwnd: HWND, full_screen: bool) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist2_vt().MarkFullscreenWindow)(
						self.ppvt,
						hwnd.ptr,
						full_screen as _,
					),
				)
			}
		}
	};
}

impl_IUnknown!(ITaskbarList2, ITaskbarList2VT);
impl_ITaskbarList!(ITaskbarList2, ITaskbarList2VT);
impl_ITaskbarList2!(ITaskbarList2, ITaskbarList2VT);
