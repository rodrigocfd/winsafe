#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::shell::vt::{ITaskbarListVT, ITaskbarList2VT};
use crate::com::traits::{ComInterface, PPComVT};
use crate::ffi::{HANDLE, HRESULT, PCSTR, PVOID};
use crate::structs::IID;

type PP = PPComVT<IUnknownVT>;

/// [`ITaskbarList3`](crate::shell::ITaskbarList3) virtual table.
pub struct ITaskbarList3VT {
	pub ITaskbarList2VT: ITaskbarList2VT,
	pub SetProgressValue: fn(PP, HANDLE, u64, u64) -> HRESULT,
	pub SetProgressState: fn(PP, HANDLE, u32) -> HRESULT,
	pub RegisterTab: fn(PP, HANDLE, HANDLE) -> HRESULT,
	pub UnregisterTab: fn(PP, HANDLE) -> HRESULT,
	pub SetTabOrder: fn(PP, HANDLE, HANDLE) -> HRESULT,
	pub SetTabActive: fn(PP, HANDLE, HANDLE, u32) -> HRESULT,
	pub ThumbBarAddButtons: fn(PP, HANDLE, u32, PVOID) -> HRESULT,
	pub ThumbBarUpdateButtons: fn(PP, HANDLE, u32, PVOID) -> HRESULT,
	pub ThumbBarSetImageList: fn(PP, HANDLE, HANDLE) -> HRESULT,
	pub SetOverlayIcon: fn(PP, HANDLE, HANDLE, PCSTR) -> HRESULT,
	pub SetThumbnailTooltip: fn(PP, HANDLE, PCSTR) -> HRESULT,
	pub SetThumbnailClip: fn(PP, HANDLE, PVOID) -> HRESULT,
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
/// ).unwrap();
/// ```
pub struct ITaskbarList3 {
	pub(crate) ppvt: PPComVT<IUnknownVT>,
}

impl_send_sync_fromppvt!(ITaskbarList3);

impl ComInterface for ITaskbarList3 {
	const IID: IID = IID::new(0xea1afb91, 0x9e28, 0x4b86, 0x90e9, 0x9e9f8a5eefaf);
}

macro_rules! impl_ITaskbarList3 {
	($name:ty, $vt:ty) => {
		use crate::com::shell::co as shellco;

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
				hwnd: HWND, tbpfFlags: shellco::TBPF) -> WinResult<()>
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
			/// obj.SetProgressValue(hwnd, 50, 100).unwrap();
			/// ```
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

impl_IUnknown!(ITaskbarList3, ITaskbarList3VT);
impl_ITaskbarList!(ITaskbarList3, ITaskbarList3VT);
impl_ITaskbarList2!(ITaskbarList3, ITaskbarList3VT);
impl_ITaskbarList3!(ITaskbarList3, ITaskbarList3VT);
