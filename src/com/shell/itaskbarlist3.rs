#![allow(non_snake_case)]

use crate::com::shell::vt::{ITaskbarListVT, ITaskbarList2VT};
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::{HANDLE, HRESULT, PCSTR, PVOID};
use crate::structs::IID;

/// [`ITaskbarList3`](crate::shell::ITaskbarList3) virtual table.
pub struct ITaskbarList3VT {
	pub ITaskbarList2VT: ITaskbarList2VT,
	pub SetProgressValue: fn(PPVT, HANDLE, u64, u64) -> HRESULT,
	pub SetProgressState: fn(PPVT, HANDLE, u32) -> HRESULT,
	pub RegisterTab: fn(PPVT, HANDLE, HANDLE) -> HRESULT,
	pub UnregisterTab: fn(PPVT, HANDLE) -> HRESULT,
	pub SetTabOrder: fn(PPVT, HANDLE, HANDLE) -> HRESULT,
	pub SetTabActive: fn(PPVT, HANDLE, HANDLE, u32) -> HRESULT,
	pub ThumbBarAddButtons: fn(PPVT, HANDLE, u32, PVOID) -> HRESULT,
	pub ThumbBarUpdateButtons: fn(PPVT, HANDLE, u32, PVOID) -> HRESULT,
	pub ThumbBarSetImageList: fn(PPVT, HANDLE, HANDLE) -> HRESULT,
	pub SetOverlayIcon: fn(PPVT, HANDLE, HANDLE, PCSTR) -> HRESULT,
	pub SetThumbnailTooltip: fn(PPVT, HANDLE, PCSTR) -> HRESULT,
	pub SetThumbnailClip: fn(PPVT, HANDLE, PVOID) -> HRESULT,
}

/// [`ITaskbarList3`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3)
/// COM interface over [`ITaskbarList3VT`](crate::shell::vt::ITaskbarList3VT).
/// Inherits from [`ITaskbarList2`](crate::shell::ITaskbarList2),
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
/// let obj = CoCreateInstance::<shell::ITaskbarList3>(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct ITaskbarList3 {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for ITaskbarList3 {
	const IID: IID = IID::new(0xea1afb91, 0x9e28, 0x4b86, 0x90e9, 0x9e9f8a5eefaf);
}

macro_rules! impl_ITaskbarList3 {
	($name:ty, $vt:ty) => {
		use crate::com::shell::co as shellco;

		impl $name {
			fn itaskbarlist3_vt(&self) -> &ITaskbarList3VT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`ITaskbarList3::RegisterTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-registertab)
			/// method.
			pub fn RegisterTab(&self,
				hwnd_tab: HWND, hwnd_mdi: HWND) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist3_vt().RegisterTab)(
						self.ppvt,
						hwnd_tab.ptr,
						hwnd_mdi.ptr,
					),
				)
			}

			/// [`ITaskbarList3::SetProgressState`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
			/// method.
			pub fn SetProgressState(&self,
				hwnd: HWND, tbpf_flags: shellco::TBPF) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist3_vt().SetProgressState)(
						self.ppvt,
						hwnd.ptr,
						tbpf_flags.0,
					),
				)
			}

			/// [`ITaskbarList3::SetProgressValue`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressvalue)
			/// method.
			///
			/// # Examples
			///
			/// Setting progress to 50%:
			///
			/// ```rust,ignore
			/// use winsafe::{HWND, shell};
			///
			/// let obj: shell::ITaskbarList3; // initialized somewhere
			/// let hwnd: HWND;
			///
			/// obj.SetProgressValue(hwnd, 50, 100)?;
			/// ```
			pub fn SetProgressValue(&self,
				hwnd: HWND, completed: u64, total: u64) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist3_vt().SetProgressValue)(
						self.ppvt,
						hwnd.ptr,
						completed,
						total,
					),
				)
			}

			/// [`ITaskbarList3::SetTabActive`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
			/// method.
			pub fn SetTabActive(&self,
				hwnd_tab: HWND, hwnd_mdi: HWND) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist3_vt().SetTabActive)(
						self.ppvt,
						hwnd_tab.ptr,
						hwnd_mdi.ptr,
						0,
					),
				)
			}

			/// [`ITaskbarList3::SetTabOrder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
			/// method.
			pub fn SetTabOrder(&self,
				hwnd_tab: HWND, hwnd_insert_before: HWND) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist3_vt().SetTabOrder)(
						self.ppvt,
						hwnd_tab.ptr,
						hwnd_insert_before.ptr,
					),
				)
			}
		}
	};
}

impl_IUnknown!(ITaskbarList3, ITaskbarList3VT);
impl_ITaskbarList!(ITaskbarList3, ITaskbarList3VT);
impl_ITaskbarList2!(ITaskbarList3, ITaskbarList3VT);
impl_ITaskbarList3!(ITaskbarList3, ITaskbarList3VT);
