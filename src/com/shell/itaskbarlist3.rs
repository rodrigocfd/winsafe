#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::PPComVT;
use crate::com::shell::co;
use crate::com::shell::ITaskbarList2;
use crate::com::shell::vt::{ITaskbarList2VT, ITaskbarList3VT};
use crate::handles::HWND;
use crate::privs::hr_to_winresult;

/// [`ITaskbarList3`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3)
/// COM interface.
///
/// Virtual table: [`ITaskbarList3VT`](crate::shell::vt::ITaskbarList3VT).
///
/// Inherits from:
/// * [`ITaskbarList2`](crate::shell::ITaskbarList2);
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
/// let obj: shell::ITaskbarList3 = CoCreateInstance(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
#[derive(Clone)]
pub struct ITaskbarList3 {
	/// Methods of base interface [`ITaskbarList2`](crate::shell::ITaskbarList2).
	pub ITaskbarList2: ITaskbarList2,
}

impl From<PPComVT<ITaskbarList3VT>> for ITaskbarList3 {
	fn from(ppv: PPComVT<ITaskbarList3VT>) -> Self {
		Self {
			ITaskbarList2: ITaskbarList2::from(ppv as PPComVT<ITaskbarList2VT>),
		}
	}
}

impl ITaskbarList3 {
	unsafe fn ppv(&self) -> PPComVT<ITaskbarList3VT> {
		self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3VT>()
	}

	/// [`ITaskbarList3::RegisterTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-registertab)
	/// method.
	pub fn RegisterTab(&self,
		hwndTab: HWND, hwndMDI: HWND) -> WinResult<()>
	{
		hr_to_winresult(
			unsafe {
				((**self.ppv()).RegisterTab)(self.ppv(), hwndTab.ptr, hwndMDI.ptr)
			},
		)
	}

	/// [`ITaskbarList3::SetProgressState`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
	/// method.
	pub fn SetProgressState(&self,
		hwnd: HWND, tbpfFlags: co::TBPF) -> WinResult<()>
	{
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetProgressState)(self.ppv(), hwnd.ptr, tbpfFlags.0)
			},
		)
	}

	/// [`ITaskbarList3::SetProgressValue`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressvalue)
	/// method.
	pub fn SetProgressValue(&self,
		hwnd: HWND, ullCompleted: u64, ullTotal: u64) -> WinResult<()>
	{
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetProgressValue)(
					self.ppv(), hwnd.ptr, ullCompleted, ullTotal,
				)
			},
		)
	}

	/// [`ITaskbarList3::SetTabActive`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
	/// method.
	pub fn SetTabActive(&self,
		hwndTab: HWND, hwndMDI: HWND) -> WinResult<()>
	{
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetTabActive)(
					self.ppv(), hwndTab.ptr, hwndMDI.ptr, 0,
				)
			},
		)
	}

	/// [`ITaskbarList3::SetTabOrder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
	/// method.
	pub fn SetTabOrder(&self,
		hwndTab: HWND, hwndInsertBefore: HWND) -> WinResult<()>
	{
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetTabOrder)(
					self.ppv(), hwndTab.ptr, hwndInsertBefore.ptr,
				)
			},
		)
	}
}
