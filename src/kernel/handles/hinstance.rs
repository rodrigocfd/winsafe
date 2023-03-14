#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	GetLastError, HRSRC, HRSRCMEM, IdStr, LANGID, RtStr, SysResult, WString,
};
use crate::kernel::ffi_types::BOOL;
use crate::kernel::guard::FreeLibraryGuard;
use crate::kernel::privs::{
	bool_to_sysresult, MAX_PATH, ptr_to_sysresult, str_to_iso88591,
};
use crate::prelude::Handle;

impl_handle! { HINSTANCE;
	/// Handle to an
	/// [instance](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance),
	/// same as `HMODULE`.
}

impl kernel_Hinstance for HINSTANCE {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HINSTANCE`](crate::HINSTANCE).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hinstance: Handle {
	/// [`EnumResourceLanguages`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-enumresourcelanguagesw)
	/// method.
	fn EnumResourceLanguages<F>(&self,
		resource_type: RtStr,
		resource_id: IdStr,
		func: F,
	) -> SysResult<()>
		where F: Fn(LANGID) -> bool,
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::EnumResourceLanguagesW(
					self.as_ptr(),
					resource_type.as_ptr(),
					resource_id.as_ptr(),
					enum_resource_languages_proc::<F> as _,
					&func as *const _ as _,
				)
			},
		)
	}

	/// [`EnumResourceNames`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-enumresourcenamesw)
	/// method.
	fn EnumResourceNames<F>(&self,
		resource_type: RtStr,
		func: F,
	) -> SysResult<()>
		where F: Fn(IdStr) -> bool,
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::EnumResourceNamesW(
					self.as_ptr(),
					resource_type.as_ptr(),
					enum_resource_names_proc::<F> as _,
					&func as *const _ as _,
				)
			},
		)
	}

	/// [`EnumResourceTypes`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-enumresourcetypesw)
	/// method.
	fn EnumResourceTypes<F>(&self, func: F) -> SysResult<()>
		where F: Fn(RtStr) -> bool,
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::EnumResourceTypesW(
					self.as_ptr(),
					enum_resource_types_proc::<F> as _,
					&func as *const _ as _,
				)
			},
		)
	}

	/// [`FindResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-findresourcew)
	/// method.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::prelude::kernel_Hinstance::LockResource).
	#[must_use]
	fn FindResource(&self,
		resource_id: IdStr, resource_type: RtStr) -> SysResult<HRSRC>
	{
		unsafe {
			ptr_to_sysresult(
				kernel::ffi::FindResourceW(
					self.as_ptr(),
					resource_id.as_ptr(),
					resource_type.as_ptr(),
				),
				|ptr| HRSRC::from_ptr(ptr),
			)
		}
	}

	/// [`FindResourceEx`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-findresourceexw)
	/// method.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::prelude::kernel_Hinstance::LockResource).
	#[must_use]
	fn FindResourceEx(&self,
		resource_id: IdStr,
		resource_type: RtStr,
		language: Option<LANGID>,
	) -> SysResult<HRSRC>
	{
		unsafe {
			ptr_to_sysresult(
				kernel::ffi::FindResourceExW(
					self.as_ptr(),
					resource_id.as_ptr(),
					resource_type.as_ptr(),
					language.unwrap_or(LANGID::new(co::LANG::NEUTRAL, co::SUBLANG::NEUTRAL)).0,
				),
				|ptr| HRSRC::from_ptr(ptr),
			)
		}
	}

	/// [`GetModuleFileName`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulefilenamew)
	/// method.
	///
	/// # Examples
	///
	/// Retrieving the full path of currently running .exe file:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::HINSTANCE;
	///
	/// let exe_name = HINSTANCE::NULL.GetModuleFileName()?;
	///
	/// println!("EXE: {}", exe_name);
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn GetModuleFileName(&self) -> SysResult<String> {
		let mut buf = [0; MAX_PATH];
		match unsafe {
			kernel::ffi::GetModuleFileNameW(
				self.as_ptr(),
				buf.as_mut_ptr(),
				buf.len() as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(WString::from_wchars_slice(&buf).to_string()),
		}
	}

	/// [`GetModuleHandle`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
	/// static method.
	///
	/// # Examples
	///
	/// Retrieving current module instance:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::HINSTANCE;
	///
	/// let hinstance = HINSTANCE::GetModuleHandle(None)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn GetModuleHandle(module_name: Option<&str>) -> SysResult<HINSTANCE> {
		ptr_to_sysresult(
			unsafe {
				kernel::ffi::GetModuleHandleW(
					WString::from_opt_str(module_name).as_ptr(),
				)
			},
			|ptr| HINSTANCE(ptr),
		)
	}

	/// [`GetProcAddress`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)
	/// method.
	#[must_use]
	fn GetProcAddress(&self,
		proc_name: &str) -> SysResult<*const std::ffi::c_void>
	{
		ptr_to_sysresult(
			unsafe {
				kernel::ffi::GetProcAddress(
					self.as_ptr(),
					str_to_iso88591(proc_name).as_ptr(),
				) as _
			},
			|ptr| ptr as _,
		)
	}

	/// [`LoadLibrary`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)
	/// static method.
	#[must_use]
	fn LoadLibrary(lib_file_name: &str) -> SysResult<FreeLibraryGuard> {
		unsafe {
			ptr_to_sysresult(
				kernel::ffi::LoadLibraryW(
					WString::from_str(lib_file_name).as_ptr()),
				|ptr| FreeLibraryGuard::new(HINSTANCE::from_ptr(ptr)),
			)
		}
	}

	/// [`LoadResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadresource)
	/// method.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::prelude::kernel_Hinstance::LockResource).
	#[must_use]
	fn LoadResource(&self, res_info: &HRSRC) -> SysResult<HRSRCMEM> {
		unsafe {
			ptr_to_sysresult(
				kernel::ffi::LoadResource(self.as_ptr(), res_info.as_ptr()),
				|ptr| HRSRCMEM::from_ptr(ptr),
			)
		}
	}

	/// [`LockResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-lockresource)
	/// method.
	///
	/// This method should belong to [`HRSRCMEM`](crate::HRSRCMEM), but in order
	/// to make it safe, we automatically call
	/// [`HINSTANCE::SizeofResource`](crate::prelude::kernel_Hinstance::SizeofResource),
	/// so it's implemented here.
	///
	/// # Examples
	///
	/// The
	/// [Updating Resources](https://learn.microsoft.com/en-us/windows/win32/menurc/using-resources#updating-resources)
	/// example:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{
	///     co, HINSTANCE, HUPDATERSRC, IdStr, LANGID, RtStr,
	/// };
	///
	/// const IDD_HAND_ABOUTBOX: u16 = 103;
	/// const IDD_FOOT_ABOUTBOX: u16 = 110;
	///
	/// let hExe = HINSTANCE::LoadLibrary("hand.exe")?;
	///
	/// let hRes = hExe.FindResource(
	///     IdStr::Id(IDD_HAND_ABOUTBOX),
	///     RtStr::Rt(co::RT::DIALOG),
	/// )?;
	///
	/// let hResLoad = hExe.LoadResource(&hRes)?;
	/// let lpResLock = hExe.LockResource(&hRes, &hResLoad)?;
	/// let hUpdateRes = HUPDATERSRC::BeginUpdateResource("foot.exe", false)?;
	///
	/// hUpdateRes.UpdateResource(
	///     RtStr::Rt(co::RT::DIALOG),
	///     IdStr::Id(IDD_FOOT_ABOUTBOX),
	///     LANGID::new(co::LANG::NEUTRAL, co::SUBLANG::NEUTRAL),
	///     lpResLock,
	/// )?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn LockResource(&self,
		res_info: &HRSRC, hres_loaded: &HRSRCMEM) -> SysResult<&[u8]>
	{
		let sz = self.SizeofResource(res_info)?;
		unsafe {
			ptr_to_sysresult(
				kernel::ffi::LockResource(hres_loaded.as_ptr()),
				|ptr| std::slice::from_raw_parts(ptr as *const _ as _, sz as _),
			)
		}
	}

	/// [`SizeofResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-sizeofresource)
	/// method.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::prelude::kernel_Hinstance::LockResource).
	#[must_use]
	fn SizeofResource(&self, res_info: &HRSRC) -> SysResult<u32> {
		match unsafe {
			kernel::ffi::SizeofResource(self.as_ptr(), res_info.as_ptr())
		} {
			0 => Err(GetLastError()),
			sz => Ok(sz)
		}
	}
}

//------------------------------------------------------------------------------

extern "system" fn enum_resource_languages_proc<F>(
	_: HINSTANCE, _: *const u16, _: *const u16,
	language_id: u16, lparam: isize) -> BOOL
	where F: Fn(LANGID) -> bool,
{
	let func = unsafe { &*(lparam as *const F) };
	func(LANGID(language_id)) as _
}

extern "system" fn enum_resource_names_proc<F>(
	_: HINSTANCE, _: *const u16, resource_id: *mut u16, lparam: isize) -> BOOL
	where F: Fn(IdStr) -> bool,
{
	let func = unsafe { &*(lparam as *const F) };
	func(IdStr::from_ptr(resource_id)) as _
}

extern "system" fn enum_resource_types_proc<F>(
	_: HINSTANCE, resource_type: *const u16, lparam: isize) -> BOOL
	where F: Fn(RtStr) -> bool,
{
	let func = unsafe { &*(lparam as *const F) };
	func(RtStr::from_ptr(resource_type)) as _
}
