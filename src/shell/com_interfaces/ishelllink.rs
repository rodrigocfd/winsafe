#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::decl::{WIN32_FIND_DATA, WString};
use crate::kernel::ffi_types::{HANDLE, HRES, PCSTR, PSTR, PVOID};
use crate::kernel::privs::MAX_PATH;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{Handle, ole_IUnknown};
use crate::shell::privs::INFOTIPSIZE;
use crate::user::decl::HWND;
use crate::vt::IUnknownVT;

/// [`IShellLink`](crate::IShellLink) virtual table.
#[repr(C)]
pub struct IShellLinkVT {
	pub IUnknownVT: IUnknownVT,
	pub GetPath: fn(ComPtr, PCSTR, i32, PVOID, u32) -> HRES,
	pub GetIDList: fn(ComPtr, PVOID) -> HRES,
	pub SetIDList: fn(ComPtr, PVOID) -> HRES,
	pub GetDescription: fn(ComPtr, PSTR, i32) -> HRES,
	pub SetDescription: fn(ComPtr, PCSTR) -> HRES,
	pub GetWorkingDirectory: fn(ComPtr, PSTR, i32) -> HRES,
	pub SetWorkingDirectory: fn(ComPtr, PCSTR) -> HRES,
	pub GetArguments: fn(ComPtr, PSTR, i32) -> HRES,
	pub SetArguments: fn(ComPtr, PCSTR) -> HRES,
	pub GetHotkey: fn(ComPtr, *mut u16) -> HRES,
	pub SetHotkey: fn(ComPtr, u16) -> HRES,
	pub GetShowCmd: fn(ComPtr, *mut i32) -> HRES,
	pub SetShowCmd: fn(ComPtr, i32) -> HRES,
	pub GetIconLocation: fn(ComPtr, PSTR, i32, *mut i32) -> HRES,
	pub SetIconLocation: fn(ComPtr, PCSTR, i32) -> HRES,
	pub SetRelativePath: fn(ComPtr, PCSTR, u32) -> HRES,
	pub Resolve: fn(ComPtr, HANDLE, u32) -> HRES,
	pub SetPath: fn(ComPtr, PCSTR) -> HRES,
}

com_interface! { IShellLink: "000214f9-0000-0000-c000-000000000046";
	/// [`IShellLink`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishelllinkw)
	/// COM interface over [`IShellLinkVT`](crate::vt::IShellLinkVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, CoCreateInstance, IShellLink};
	///
	/// let obj = CoCreateInstance::<IShellLink>(
	///     &co::CLSID::ShellLink,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl shell_IShellLink for IShellLink {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellLink`](crate::IShellLink).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IShellLink: ole_IUnknown {
	/// [`IShellLinkW::GetArguments`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getarguments)
	/// method.
	#[must_use]
	fn GetArguments(&self) -> HrResult<String> {
		let mut buf = WString::new_alloc_buf(INFOTIPSIZE + 1); // arbitrary
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult(
				(vt.GetArguments)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
				),
			).map(|_| buf.to_string())
		}
	}

	/// [`IShellLinkW::GetDescription`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getdescription)
	/// method.
	#[must_use]
	fn GetDescription(&self) -> HrResult<String> {
		let mut buf = WString::new_alloc_buf(INFOTIPSIZE + 1);
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult(
				(vt.GetDescription)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
				),
			).map(|_| buf.to_string())
		}
	}

	/// [`IShellLinkW::GetIconLocation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-geticonlocation)
	/// method.
	///
	/// Returns the path of the icon and its index within the file.
	#[must_use]
	fn GetIconLocation(&self) -> HrResult<(String, i32)> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		let mut index: i32 = 0;

		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult(
				(vt.GetIconLocation)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
					&mut index,
				),
			).map(|_| (buf.to_string(), index))
		}
	}

	/// [`IShellLinkW::GetPath`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getpath)
	/// method.
	#[must_use]
	fn GetPath(&self,
		fd: Option<&mut WIN32_FIND_DATA>, flags: co::SLGP) -> HrResult<String>
	{
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult(
				(vt.GetPath)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
					fd.map_or(std::ptr::null_mut(), |fd| fd as *mut _ as _),
					flags.0,
				),
			).map(|_| buf.to_string())
		}
	}

	/// [`IShellLinkW::GetShowCmd`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getshowcmd)
	/// method.
	#[must_use]
	fn GetShowCmd(&self) -> HrResult<co::SW> {
		let mut show_cmd = co::SW::default();
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult((vt.GetShowCmd)(self.ptr(), &mut show_cmd.0))
				.map(|_| show_cmd)
		}
	}

	/// [`IShellLinkW::GetWorkingDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getworkingdirectory)
	/// method.
	#[must_use]
	fn GetWorkingDirectory(&self) -> HrResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult(
				(vt.GetWorkingDirectory)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
				),
			).map(|_| buf.to_string())
		}
	}

	/// [`IShellLinkW::Resolve`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-resolve)
	/// method.
	fn Resolve(&self, hwnd: &HWND, flags: co::SLR) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult((vt.Resolve)(self.ptr(), hwnd.as_ptr(), flags.0))
		}
	}

	/// [`IShellLinkW::SetArguments`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setarguments)
	/// method.
	fn SetArguments(&self, args: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult(
				(vt.SetArguments)(
					self.ptr(),
					WString::from_str(args).as_ptr(),
				),
			)
		}
	}

	/// [`IShellLinkW::SetDescription`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setdescription)
	/// method.
	fn SetDescription(&self, args: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult(
				(vt.SetDescription)(
					self.ptr(),
					WString::from_str(args).as_ptr(),
				),
			)
		}
	}

	/// [`IShellLinkW::SetIconLocation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-seticonlocation)
	/// method.
	fn SetIconLocation(&self, path: &str, index: i32) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult(
				(vt.SetIconLocation)(
					self.ptr(),
					WString::from_str(path).as_ptr(),
					index,
				),
			)
		}
	}

	/// [`IShellLinkW::SetPath`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setpath)
	/// method.
	fn SetPath(&self, file: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult(
				(vt.SetPath)(
					self.ptr(),
					WString::from_str(file).as_ptr(),
				),
			)
		}
	}

	/// [`IShellLinkW::SetRelativePath`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setrelativepath)
	/// method.
	fn SetRelativePath(&self, file: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult(
				(vt.SetRelativePath)(
					self.ptr(),
					WString::from_str(file).as_ptr(),
					0,
				),
			)
		}
	}

	/// [`IShellLinkW::SetShowCmd`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setshowcmd)
	/// method.
	fn SetShowCmd(&self, show_cmd: co::SW) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult((vt.SetShowCmd)(self.ptr(), show_cmd.0))
		}
	}

	/// [`IShellLinkW::SetWorkingDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setworkingdirectory)
	/// method.
	fn SetWorkingDirectory(&self, dir: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IShellLinkVT>();
			ok_to_hrresult(
				(vt.SetWorkingDirectory)(
					self.ptr(),
					WString::from_str(dir).as_ptr(),
				),
			)
		}
	}
}
