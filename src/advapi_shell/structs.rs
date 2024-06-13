#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::prelude::*;

/// [`SHELLEXECUTEINFO`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-shellexecuteinfow)
/// struct.
///
/// Used with [`ShellExecuteEx`](crate::ShellExecuteEx) function.
///
/// Not all `mask` constants are available, some of them are automatically set
/// as you fill other parameters.
#[derive(Default)]
pub struct SHELLEXECUTEINFO<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {
	pub mask: co::SEE_MASK,
	pub hwnd: Option<&'a HWND>,
	pub verb: Option<&'b str>,
	pub file: &'c str,
	pub parameters: Option<&'d str>,
	pub directory: Option<&'e str>,
	pub show: co::SW,
	pub id_list: Option<&'f [u8]>,
	pub class: Option<&'g str>,
	pub hkey_class: Option<&'h HKEY>,
	pub hot_key: Option<(co::VK, co::HOTKEYF)>,
	pub hicon_hmonitor: IcoMon<'i>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i>
	SHELLEXECUTEINFO<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i>
{
	pub(in crate::advapi_shell) fn to_raw(&self) -> SHELLEXECUTEINFO_buf {
		let mut raw = SHELLEXECUTEINFO_raw::default();
		raw.fMask = self.mask;
		raw.hwnd = unsafe { self.hwnd.unwrap_or(&HWND::NULL).raw_copy() };

		let w_verb = self.verb
			.map_or(WString::new(), |s| WString::from_str_force_heap(s));
		raw.lpVerb = w_verb.as_ptr();

		let w_file = WString::from_str_force_heap(self.file);
		raw.lpFile = w_file.as_ptr();

		let w_parms = self.parameters
			.map_or(WString::new(), |s| WString::from_str_force_heap(s));
		raw.lpParameters = w_parms.as_ptr();

		let w_dir = self.directory
			.map_or(WString::new(), |s| WString::from_str_force_heap(s));
		raw.lpDirectory = w_dir.as_ptr();

		raw.nShow = self.show;

		self.id_list.map(|l| {
			raw.lpIDList = l.as_ptr() as _;
			raw.fMask |= co::SEE_MASK::IDLIST;
		});

		let w_class = match self.class {
			Some(c) => {
				let w_class = WString::from_str_force_heap(c);
				raw.lpClass = w_class.as_ptr();
				raw.fMask |= co::SEE_MASK::CLASSNAME;
				w_class
			},
			None => WString::new(),
		};

		self.hkey_class.map(|h| {
			raw.hkeyClass = unsafe { h.raw_copy() };
			raw.fMask |= co::SEE_MASK::CLASSKEY;
		});

		self.hot_key.as_ref().map(|hk| {
			raw.dwHotKey = MAKEDWORD(hk.0.raw(), hk.1.raw());
			raw.fMask |= co::SEE_MASK::HOTKEY;
		});

		match self.hicon_hmonitor {
			IcoMon::None => {},
			IcoMon::Ico(i) => {
				raw.hIcon_hMonitor = i.ptr();
				raw.fMask |= co::SEE_MASK::ICON;
			},
			IcoMon::Mon(m) => {
				raw.hIcon_hMonitor = m.ptr();
				raw.fMask |= co::SEE_MASK::HMONITOR;
			},
		}

		SHELLEXECUTEINFO_buf { raw, w_verb, w_file, w_parms, w_dir, w_class }
	}
}

#[allow(unused)]
pub(in crate::advapi_shell) struct SHELLEXECUTEINFO_buf {
	pub raw: SHELLEXECUTEINFO_raw,
	w_verb: WString,
	w_file: WString,
	w_parms: WString,
	w_dir: WString,
	w_class: WString,
}

#[repr(C)]
pub(in crate::advapi_shell) struct SHELLEXECUTEINFO_raw {
	cbSize: u32,
	pub fMask: co::SEE_MASK,
	pub hwnd: HWND,
	pub lpVerb: *const u16,
	pub lpFile: *const u16,
	pub lpParameters: *const u16,
	pub lpDirectory: *const u16,
	pub nShow: co::SW,
	hInstApp: HINSTANCE,
	pub lpIDList: *const std::ffi::c_void,
	pub lpClass: *const u16,
	pub hkeyClass: HKEY,
	pub dwHotKey: u32,
	pub hIcon_hMonitor: HANDLE, // union HICON and HMONITOR
	pub hProcess: HPROCESS,
}

impl_default_with_size!(SHELLEXECUTEINFO_raw, cbSize);
