#![allow(non_snake_case)]

macro_rules! ITaskbarList3_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::co;
		use crate::com::shell::vt::ITaskbarList3VT;

		ITaskbarList2_impl!{
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn itaskbarlist3_vt(&self) -> &ITaskbarList3VT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`ITaskbarList3::RegisterTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-registertab)
			/// method.
			pub fn RegisterTab(&self,
				hwndTab: HWND, hwndMDI: HWND) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist3_vt().RegisterTab)(
						self.ppvt,
						hwndTab.ptr,
						hwndMDI.ptr,
					),
				)
			}

			/// [`ITaskbarList3::SetProgressState`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
			/// method.
			pub fn SetProgressState(&self,
				hwnd: HWND, tbpfFlags: co::TBPF) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist3_vt().SetProgressState)(
						self.ppvt,
						hwnd.ptr,
						tbpfFlags.0,
					),
				)
			}

			/// [`ITaskbarList3::SetProgressValue`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressvalue)
			/// method.
			pub fn SetProgressValue(&self,
				hwnd: HWND, ullCompleted: u64, ullTotal: u64) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist3_vt().SetProgressValue)(
						self.ppvt,
						hwnd.ptr,
						ullCompleted,
						ullTotal,
					),
				)
			}

			/// [`ITaskbarList3::SetTabActive`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
			/// method.
			pub fn SetTabActive(&self,
				hwndTab: HWND, hwndMDI: HWND) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist3_vt().SetTabActive)(
						self.ppvt,
						hwndTab.ptr,
						hwndMDI.ptr,
						0,
					),
				)
			}

			/// [`ITaskbarList3::SetTabOrder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
			/// method.
			pub fn SetTabOrder(&self,
				hwndTab: HWND, hwndInsertBefore: HWND) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist3_vt().SetTabOrder)(
						self.ppvt,
						hwndTab.ptr,
						hwndInsertBefore.ptr,
					),
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
	ITaskbarList3, crate::com::shell::vt::ITaskbarList3VT
}
