#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::shell::vt::{ITaskbarListVT, ITaskbarList2VT, ITaskbarList3VT};
use crate::com::traits::{ComInterface, PPComVT};
use crate::ffi::{HANDLE, HRESULT};
use crate::structs::IID;

type PP = PPComVT<IUnknownVT>;

/// [`ITaskbarList4`](crate::shell::ITaskbarList4) virtual table.
pub struct ITaskbarList4VT {
	pub ITaskbarList3VT: ITaskbarList3VT,
	pub SetTabProperties: fn(PP, HANDLE, u32) -> HRESULT,
}

/// [`ITaskbarList4`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist4)
/// COM interface over
/// [`ITaskbarList4VT`](crate::shell::vt::ITaskbarList4VT). Inherits from
/// [`ITaskbarList3`](crate::shell::ITaskbarList3),
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
/// let obj = CoCreateInstance::<shell::ITaskbarList4>(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
pub struct ITaskbarList4 {
	pub(crate) ppvt: PPComVT<IUnknownVT>,
}

impl_send_sync_fromppvt!(ITaskbarList4);

impl ComInterface for ITaskbarList4 {
	const IID: IID = IID::new(0xc43dc798, 0x95d1, 0x4bea, 0x9030, 0xbb99e2983a1a);
}

macro_rules! impl_ITaskbarList4 {
	($name:ty, $vt:ty) => {
		impl $name {
			fn itaskbarlist4_vt(&self) -> &ITaskbarList4VT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`ITaskbarList4::SetTabProperties`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist4-settabproperties)
			/// method.
			pub fn SetTabProperties(&self,
				hwndTab: HWND, stpFlags: shellco::STPFLAG) -> WinResult<()>
			{
				hr_to_winresult(
					(self.itaskbarlist4_vt().SetTabProperties)(
						self.ppvt,
						hwndTab.ptr,
						stpFlags.0,
					),
				)
			}
		}
	};
}

impl_IUnknown!(ITaskbarList4, ITaskbarList4VT);
impl_ITaskbarList!(ITaskbarList4, ITaskbarList4VT);
impl_ITaskbarList2!(ITaskbarList4, ITaskbarList4VT);
impl_ITaskbarList3!(ITaskbarList4, ITaskbarList4VT);
impl_ITaskbarList4!(ITaskbarList4, ITaskbarList4VT);
