#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::shell::vt::ITaskbarListVT;
use crate::handles::HWND;
use crate::privs::hr_to_winresult;

macro_rules! ITaskbarList_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`ITaskbarList::ActivateTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-activatetab)
			/// method.
			pub fn ActivateTab(&self, hwnd: HWND) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<ITaskbarListVT>() };
				hr_to_winresult( unsafe { ((**ppvt).ActivateTab)(ppvt, hwnd.ptr) })
			}

			/// [`ITaskbarList::AddTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-addtab)
			/// method.
			pub fn AddTab(&self, hwnd: HWND) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<ITaskbarListVT>() };
				hr_to_winresult(unsafe { ((**ppvt).AddTab)(ppvt, hwnd.ptr) })
			}

			/// [`ITaskbarList::DeleteTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-deletetab)
			/// method.
			pub fn DeleteTab(&self, hwnd: HWND) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<ITaskbarListVT>() };
				hr_to_winresult( unsafe { ((**ppvt).DeleteTab)(ppvt, hwnd.ptr) })
			}

			/// [`ITaskbarList::HrInit`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-hrinit)
			/// method.
			pub fn HrInit(&self) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<ITaskbarListVT>() };
				hr_to_winresult(unsafe { ((**ppvt).HrInit)(ppvt) })
			}

			/// [`ITaskbarList::SetActiveAlt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
			/// method.
			pub fn SetActiveAlt(&self, hwnd: HWND) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<ITaskbarListVT>() };
				hr_to_winresult(
					unsafe { ((**ppvt).SetActiveAlt)(ppvt, hwnd.ptr) },
				)
			}
		}
	};
}

ITaskbarList_impl! {
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
	/// let obj: shell::ITaskbarList = CoCreateInstance(
	///     &shell::clsid::TaskbarList,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// ).unwrap();
	/// ```
	ITaskbarList, ITaskbarListVT
}
