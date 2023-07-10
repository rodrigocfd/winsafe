#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	GetLastError, HRSRC, HRSRCMEM, IdStr, LANGID, RtStr, SysResult, WString,
};
use crate::kernel::ffi_types::BOOL;
use crate::kernel::guard::FreeLibraryGuard;
use crate::kernel::privs::{
	bool_to_sysresult, MAX_PATH, ptr_to_sysresult, ptr_to_sysresult_handle,
	str_to_iso88591,
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
	/// function.
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
					self.ptr(),
					resource_type.as_ptr(),
					resource_id.as_ptr(),
					enum_resource_languages_proc::<F> as _,
					&func as *const _ as _,
				)
			},
		)
	}

	/// [`EnumResourceNames`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-enumresourcenamesw)
	/// function.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HINSTANCE, IdStr, RtStr};
	///
	/// let hexe = HINSTANCE::LoadLibrary("hand.exe")?;
	///
	/// hexe.EnumResourceTypes(|res_type: RtStr| -> bool {
	///     let res_type2 = res_type.clone();
	///     hexe.EnumResourceNames(res_type, |name: IdStr| -> bool {
	///         println!("Type: {}, name: {}", res_type2, name);
	///         true
	///     }).unwrap();
	///     true
	/// })?;
	///
	/// // FreeLibrary() called automatically
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn EnumResourceNames<F>(&self,
		resource_type: RtStr,
		func: F,
	) -> SysResult<()>
		where F: Fn(IdStr) -> bool,
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::EnumResourceNamesW(
					self.ptr(),
					resource_type.as_ptr(),
					enum_resource_names_proc::<F> as _,
					&func as *const _ as _,
				)
			},
		)
	}

	/// [`EnumResourceTypes`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-enumresourcetypesw)
	/// function.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HINSTANCE, RtStr};
	///
	/// let hexe = HINSTANCE::LoadLibrary("hand.exe")?;
	///
	/// hexe.EnumResourceTypes(|res_type: RtStr| -> bool {
	///     println!("Type {}", res_type);
	///     true
	/// })?;
	///
	/// // FreeLibrary() called automatically
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn EnumResourceTypes<F>(&self, func: F) -> SysResult<()>
		where F: Fn(RtStr) -> bool,
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::EnumResourceTypesW(
					self.ptr(),
					enum_resource_types_proc::<F> as _,
					&func as *const _ as _,
				)
			},
		)
	}

	/// [`FindResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-findresourcew)
	/// function.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::prelude::kernel_Hinstance::LockResource).
	#[must_use]
	fn FindResource(&self,
		resource_id: IdStr, resource_type: RtStr) -> SysResult<HRSRC>
	{
		ptr_to_sysresult_handle(
			unsafe {
				kernel::ffi::FindResourceW(
					self.ptr(),
					resource_id.as_ptr(),
					resource_type.as_ptr(),
				)
			},
		)
	}

	/// [`FindResourceEx`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-findresourceexw)
	/// function.
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
		ptr_to_sysresult_handle(
			unsafe {
				kernel::ffi::FindResourceExW(
					self.ptr(),
					resource_id.as_ptr(),
					resource_type.as_ptr(),
					language.unwrap_or(LANGID::new(co::LANG::NEUTRAL, co::SUBLANG::NEUTRAL)).into(),
				)
			},
		)
	}

	/// [`GetModuleFileName`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulefilenamew)
	/// function.
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
		bool_to_sysresult(
			unsafe {
				kernel::ffi::GetModuleFileNameW(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.len() as _,
				)
			} as _,
		).map(|_| WString::from_wchars_slice(&buf).to_string())
	}

	/// [`GetModuleHandle`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
	/// function.
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
		ptr_to_sysresult_handle(
			unsafe {
				kernel::ffi::GetModuleHandleW(
					WString::from_opt_str(module_name).as_ptr(),
				)
			},
		)
	}

	/// [`GetProcAddress`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)
	/// function.
	#[must_use]
	fn GetProcAddress(&self,
		proc_name: &str) -> SysResult<*const std::ffi::c_void>
	{
		ptr_to_sysresult(
			unsafe {
				kernel::ffi::GetProcAddress(
					self.ptr(),
					str_to_iso88591(proc_name).as_ptr(),
				) as _
			},
		).map(|ptr| ptr as _)
	}

	/// [`LoadLibrary`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)
	/// function.
	#[must_use]
	fn LoadLibrary(lib_file_name: &str) -> SysResult<FreeLibraryGuard> {
		unsafe {
			ptr_to_sysresult_handle(
				kernel::ffi::LoadLibraryW(
					WString::from_str(lib_file_name).as_ptr()),
			).map(|h| FreeLibraryGuard::new(h))
		}
	}

	/// [`LoadResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadresource)
	/// function.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::prelude::kernel_Hinstance::LockResource).
	#[must_use]
	fn LoadResource(&self, res_info: &HRSRC) -> SysResult<HRSRCMEM> {
		ptr_to_sysresult_handle(
			unsafe { kernel::ffi::LoadResource(self.ptr(), res_info.ptr()) },
		)
	}

	/// [`LockResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-lockresource)
	/// function.
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
	/// let hexe = HINSTANCE::LoadLibrary("hand.exe")?;
	///
	/// let hres = hexe.FindResource(
	///     IdStr::Id(IDD_HAND_ABOUTBOX),
	///     RtStr::Rt(co::RT::DIALOG),
	/// )?;
	///
	/// let hres_load = hexe.LoadResource(&hres)?;
	/// let hres_slice_lock = hexe.LockResource(&hres, &hres_load)?;
	/// let hres_update = HUPDATERSRC::BeginUpdateResource("foot.exe", false)?;
	///
	/// hres_update.UpdateResource(
	///     RtStr::Rt(co::RT::DIALOG),
	///     IdStr::Id(IDD_FOOT_ABOUTBOX),
	///     LANGID::new(co::LANG::NEUTRAL, co::SUBLANG::NEUTRAL),
	///     hres_slice_lock,
	/// )?;
	///
	/// // EndUpdateResource() called automatically
	///
	/// // FreeLibrary() called automatically
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn LockResource(&self,
		res_info: &HRSRC, hres_loaded: &HRSRCMEM) -> SysResult<&[u8]>
	{
		let sz = self.SizeofResource(res_info)?;
		unsafe {
			ptr_to_sysresult(
				kernel::ffi::LockResource(hres_loaded.ptr()),
			).map(|ptr| std::slice::from_raw_parts(ptr.cast(), sz as _))
		}
	}

	/// [`SizeofResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-sizeofresource)
	/// function.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::prelude::kernel_Hinstance::LockResource).
	#[must_use]
	fn SizeofResource(&self, res_info: &HRSRC) -> SysResult<u32> {
		match unsafe {
			kernel::ffi::SizeofResource(self.ptr(), res_info.ptr())
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
	func(unsafe { LANGID::from_raw(language_id) }) as _
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
