#![allow(non_snake_case)]

use crate::co;
use crate::ffi_types::{HANDLE, HRES, PCSTR, PSTR, PVOID};
use crate::kernel::decl::{WIN32_FIND_DATA, WString};
use crate::kernel::privs::MAX_PATH;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::OleIUnknown;
use crate::shell::privs::INFOTIPSIZE;
use crate::user::decl::HWND;
use crate::vt::IUnknownVT;

/// [`IShellLink`](crate::IShellLink) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
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

/// [`IShellLinkW`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishelllinkw)
/// COM interface over [`IShellLinkVT`](crate::vt::IShellLinkVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
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
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub struct IShellLink(ComPtr);

impl_iunknown!(IShellLink, "000214f9-0000-0000-c000-000000000046");
impl ShellIShellLink for IShellLink {}

/// [`IShellLink`](crate::IShellLink) methods from `shell` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait ShellIShellLink: OleIUnknown {
	/// [`IShellLinkW::GetArguments`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getarguments)
	/// method.
	fn GetArguments(&self) -> HrResult<String> {
		let mut buf = WString::new_alloc_buffer(INFOTIPSIZE + 1); // arbitrary
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult(
				(vt.GetArguments)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buffer_size() as _,
				),
			).map(|_| buf.to_string())
		}
	}

	/// [`IShellLinkW::GetDescription`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getdescription)
	/// method.
	fn GetDescription(&self) -> HrResult<String> {
		let mut buf = WString::new_alloc_buffer(INFOTIPSIZE + 1);
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult(
				(vt.GetDescription)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buffer_size() as _,
				),
			).map(|_| buf.to_string())
		}
	}

	/// [`IShellLinkW::GetIconLocation`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-geticonlocation)
	/// method.
	///
	/// Returns the path of the icon and its index within the file.
	fn GetIconLocation(&self) -> HrResult<(String, i32)> {
		let mut buf = WString::new_alloc_buffer(MAX_PATH + 1);
		let mut index: i32 = 0;

		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult(
				(vt.GetIconLocation)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buffer_size() as _,
					&mut index,
				),
			).map(|_| (buf.to_string(), index))
		}
	}

	/// [`IShellLinkW::GetPath`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getpath)
	/// method.
	fn GetPath(&self,
		fd: Option<&mut WIN32_FIND_DATA>,
		flags: co::SLGP) -> HrResult<String>
	{
		let mut buf = WString::new_alloc_buffer(MAX_PATH + 1);
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult(
				(vt.GetPath)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buffer_size() as _,
					fd.map_or(std::ptr::null_mut(), |fd| fd as *mut _ as _),
					flags.0,
				),
			).map(|_| buf.to_string())
		}
	}

	/// [`IShellLinkW::GetShowCmd`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getshowcmd)
	/// method.
	fn GetShowCmd(&self) -> HrResult<co::SW> {
		let mut show_cmd = co::SW::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult((vt.GetShowCmd)(self.ptr(), &mut show_cmd.0))
				.map(|_| show_cmd)
		}
	}

	/// [`IShellLinkW::GetWorkingDirectory`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getworkingdirectory)
	/// method.
	fn GetWorkingDirectory(&self) -> HrResult<String> {
		let mut buf = WString::new_alloc_buffer(MAX_PATH + 1);
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult(
				(vt.GetWorkingDirectory)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buffer_size() as _,
				),
			).map(|_| buf.to_string())
		}
	}

	/// [`IShellLinkW::Resolve`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-resolve)
	/// method.
	fn Resolve(&self, hwnd: HWND, flags: co::SLR) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult((vt.Resolve)(self.ptr(), hwnd.0, flags.0))
		}
	}

	/// [`IShellLinkW::SetArguments`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setarguments)
	/// method.
	fn SetArguments(&self, args: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult(
				(vt.SetArguments)(
					self.ptr(),
					WString::from_str(args).as_ptr(),
				),
			)
		}
	}

	/// [`IShellLinkW::SetDescription`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setdescription)
	/// method.
	fn SetDescription(&self, args: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult(
				(vt.SetDescription)(
					self.ptr(),
					WString::from_str(args).as_ptr(),
				),
			)
		}
	}

	/// [`IShellLinkW::SetIconLocation`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-seticonlocation)
	/// method.
	fn SetIconLocation(&self, path: &str, index: i32) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult(
				(vt.SetIconLocation)(
					self.ptr(),
					WString::from_str(path).as_ptr(),
					index,
				),
			)
		}
	}

	/// [`IShellLinkW::SetPath`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setpath)
	/// method.
	fn SetPath(&self, file: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult(
				(vt.SetPath)(
					self.ptr(),
					WString::from_str(file).as_ptr(),
				),
			)
		}
	}

	/// [`IShellLinkW::SetRelativePath`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setrelativepath)
	/// method.
	fn SetRelativePath(&self, file: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult(
				(vt.SetRelativePath)(
					self.ptr(),
					WString::from_str(file).as_ptr(),
					0,
				),
			)
		}
	}

	/// [`IShellLinkW::SetShowCmd`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setshowcmd)
	/// method.
	fn SetShowCmd(&self, show_cmd: co::SW) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult((vt.SetShowCmd)(self.ptr(), show_cmd.0))
		}
	}

	/// [`IShellLinkW::SetWorkingDirectory`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setworkingdirectory)
	/// method.
	fn SetWorkingDirectory(&self, dir: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellLinkVT);
			ok_to_hrresult(
				(vt.SetWorkingDirectory)(
					self.ptr(),
					WString::from_str(dir).as_ptr(),
				),
			)
		}
	}
}
