#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPI};
use crate::ffi::{HANDLE, HRESULT};
use crate::structs::IID;

/// [`ITaskbarList`](crate::shell::ITaskbarList) virtual table.
pub struct ITaskbarListVT {
	pub IUnknownVT: IUnknownVT,
	pub HrInit: fn(PPI) -> HRESULT,
	pub AddTab: fn(PPI, HANDLE) -> HRESULT,
	pub DeleteTab: fn(PPI, HANDLE) -> HRESULT,
	pub ActivateTab: fn(PPI, HANDLE) -> HRESULT,
	pub SetActiveAlt: fn(PPI, HANDLE) -> HRESULT,
}

/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
/// COM interface over [`ITaskbarListVT`](crate::shell::vt::ITaskbarListVT).
/// Inherits from [`IUnknown`](crate::IUnknown).
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
/// let obj = CoCreateInstance::<shell::ITaskbarList>(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
pub struct ITaskbarList {
	pub(crate) ppvt: PPI,
}

impl_send_sync_fromppvt!(ITaskbarList);

impl ComInterface for ITaskbarList {
	const IID: IID = IID::new(0x56fdf342, 0xfd6d, 0x11d0, 0x958a, 0x006097c9a090);
}

macro_rules! impl_ITaskbarList {
	($name:ty, $vt:ty) => {
		use crate::handles::HWND;

		impl $name {
			fn itaskbarlist_vt(&self) -> &ITaskbarListVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`ITaskbarList::ActivateTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-activatetab)
			/// method.
			pub fn ActivateTab(&self, hwnd: HWND) -> WinResult<()> {
				hr_to_winresult(
					(self.itaskbarlist_vt().ActivateTab)(self.ppvt, hwnd.ptr),
				)
			}

			/// [`ITaskbarList::AddTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-addtab)
			/// method.
			pub fn AddTab(&self, hwnd: HWND) -> WinResult<()> {
				hr_to_winresult(
					(self.itaskbarlist_vt().AddTab)(self.ppvt, hwnd.ptr),
				)
			}

			/// [`ITaskbarList::DeleteTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-deletetab)
			/// method.
			pub fn DeleteTab(&self, hwnd: HWND) -> WinResult<()> {
				hr_to_winresult(
					(self.itaskbarlist_vt().DeleteTab)(self.ppvt, hwnd.ptr),
				)
			}

			/// [`ITaskbarList::HrInit`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-hrinit)
			/// method.
			pub fn HrInit(&self) -> WinResult<()> {
				hr_to_winresult((self.itaskbarlist_vt().HrInit)(self.ppvt))
			}

			/// [`ITaskbarList::SetActiveAlt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
			/// method.
			pub fn SetActiveAlt(&self, hwnd: HWND) -> WinResult<()> {
				hr_to_winresult(
					(self.itaskbarlist_vt().SetActiveAlt)(self.ppvt, hwnd.ptr),
				)
			}
		}
	};
}

impl_IUnknown!(ITaskbarList, ITaskbarListVT);
impl_ITaskbarList!(ITaskbarList, ITaskbarListVT);
