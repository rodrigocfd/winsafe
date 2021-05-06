#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::shell::vt::{ITaskbarListVT, ITaskbarList2VT, ITaskbarList3VT};
use crate::handles::HWND;
use crate::privs::hr_to_winresult;

macro_rules! ITaskbarList3_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		ITaskbarList2_impl!{
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`ITaskbarList3::RegisterTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-registertab)
			/// method.
			pub fn RegisterTab(&self,
				hwndTab: HWND, hwndMDI: HWND) -> WinResult<()>
			{
				let ppvt = unsafe { self.ppvt::<ITaskbarList3VT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).RegisterTab)(ppvt, hwndTab.ptr, hwndMDI.ptr)
					},
				)
			}

			/// [`ITaskbarList3::SetProgressState`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
			/// method.
			pub fn SetProgressState(&self,
				hwnd: HWND, tbpfFlags: co::TBPF) -> WinResult<()>
			{
				let ppvt = unsafe { self.ppvt::<ITaskbarList3VT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetProgressState)(ppvt, hwnd.ptr, tbpfFlags.0)
					},
				)
			}

			/// [`ITaskbarList3::SetProgressValue`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressvalue)
			/// method.
			pub fn SetProgressValue(&self,
				hwnd: HWND, ullCompleted: u64, ullTotal: u64) -> WinResult<()>
			{
				let ppvt = unsafe { self.ppvt::<ITaskbarList3VT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetProgressValue)(
							ppvt, hwnd.ptr, ullCompleted, ullTotal,
						)
					},
				)
			}

			/// [`ITaskbarList3::SetTabActive`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
			/// method.
			pub fn SetTabActive(&self,
				hwndTab: HWND, hwndMDI: HWND) -> WinResult<()>
			{
				let ppvt = unsafe { self.ppvt::<ITaskbarList3VT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetTabActive)(ppvt, hwndTab.ptr, hwndMDI.ptr, 0)
					},
				)
			}

			/// [`ITaskbarList3::SetTabOrder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
			/// method.
			pub fn SetTabOrder(&self,
				hwndTab: HWND, hwndInsertBefore: HWND) -> WinResult<()>
			{
				let ppvt = unsafe { self.ppvt::<ITaskbarList3VT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetTabOrder)(
							ppvt, hwndTab.ptr, hwndInsertBefore.ptr,
						)
					},
				)
			}
		}
	};
}

ITaskbarList3_impl! {
	/// [`ITaskbarList3`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3)
	/// COM interface over
	/// [`ITaskbarList3VT`](crate::shell::vt::ITaskbarList3VT). Inherits from
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
	/// let obj: shell::ITaskbarList3 = CoCreateInstance(
	///     &shell::clsid::TaskbarList,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// ).unwrap();
	/// ```
	ITaskbarList3, ITaskbarList3VT
}
