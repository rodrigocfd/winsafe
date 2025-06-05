#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*, proc};
use crate::prelude::*;

handle! { HINSTANCE;
	/// Handle to an
	/// [instance](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance),
	/// same as `HMODULE`.
}

impl HINSTANCE {
	/// [`EnumResourceLanguages`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-enumresourcelanguagesw)
	/// function.
	pub fn EnumResourceLanguages<F>(
		&self,
		resource_type: RtStr,
		resource_id: IdStr,
		func: F,
	) -> SysResult<()>
	where
		F: FnMut(LANGID) -> bool,
	{
		bool_to_sysresult(unsafe {
			ffi::EnumResourceLanguagesW(
				self.ptr(),
				resource_type.as_ptr(),
				resource_id.as_ptr(),
				proc::hinstance_enum_resource_languages::<F> as _,
				pcvoid(&func),
			)
		})
	}

	/// [`EnumResourceNames`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-enumresourcenamesw)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hexe = w::HINSTANCE::LoadLibrary("hand.exe")?;
	///
	/// hexe.EnumResourceTypes(
	///     |res_type: w::RtStr| -> bool {
	///         let res_type2 = res_type.clone();
	///         hexe.EnumResourceNames(
	///             res_type,
	///             |name: w::IdStr| -> bool {
	///                 println!("Type: {}, name: {}", res_type2, name);
	///                 true
	///             },
	///         ).unwrap();
	///         true
	///     },
	/// )?;
	///
	/// // FreeLibrary() called automatically
	/// # w::SysResult::Ok(())
	/// ```
	pub fn EnumResourceNames<F>(&self, resource_type: RtStr, func: F) -> SysResult<()>
	where
		F: FnMut(IdStr) -> bool,
	{
		bool_to_sysresult(unsafe {
			ffi::EnumResourceNamesW(
				self.ptr(),
				resource_type.as_ptr(),
				proc::hinstance_enum_resource_names::<F> as _,
				pcvoid(&func),
			)
		})
	}

	/// [`EnumResourceTypes`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-enumresourcetypesw)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hexe = w::HINSTANCE::LoadLibrary("hand.exe")?;
	///
	/// hexe.EnumResourceTypes(
	///     |res_type: w::RtStr| -> bool {
	///         println!("Type {}", res_type);
	///         true
	///     },
	/// )?;
	///
	/// // FreeLibrary() called automatically
	/// # w::SysResult::Ok(())
	/// ```
	pub fn EnumResourceTypes<F>(&self, func: F) -> SysResult<()>
	where
		F: FnMut(RtStr) -> bool,
	{
		bool_to_sysresult(unsafe {
			ffi::EnumResourceTypesW(
				self.ptr(),
				proc::hinstance_enum_resource_types::<F> as _,
				pcvoid(&func),
			)
		})
	}

	/// [`FindResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-findresourcew)
	/// function.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::HINSTANCE::LockResource).
	#[must_use]
	pub fn FindResource(&self, resource_id: IdStr, resource_type: RtStr) -> SysResult<HRSRC> {
		ptr_to_sysresult_handle(unsafe {
			ffi::FindResourceW(self.ptr(), resource_id.as_ptr(), resource_type.as_ptr())
		})
	}

	/// [`FindResourceEx`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-findresourceexw)
	/// function.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::HINSTANCE::LockResource).
	#[must_use]
	pub fn FindResourceEx(
		&self,
		resource_id: IdStr,
		resource_type: RtStr,
		language: Option<LANGID>,
	) -> SysResult<HRSRC> {
		ptr_to_sysresult_handle(unsafe {
			ffi::FindResourceExW(
				self.ptr(),
				resource_id.as_ptr(),
				resource_type.as_ptr(),
				language.unwrap_or(LANGID::NEUTRAL).into(),
			)
		})
	}

	/// [`GetModuleFileName`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulefilenamew)
	/// function.
	///
	/// # Examples
	///
	/// Retrieving the full path of currently running .exe file:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let exe_name = w::HINSTANCE::NULL.GetModuleFileName()?;
	///
	/// println!("EXE: {}", exe_name);
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn GetModuleFileName(&self) -> SysResult<String> {
		let mut buf_sz = WString::SSO_LEN; // start with no string heap allocation
		loop {
			let mut buf = WString::new_alloc_buf(buf_sz);
			let copied = match unsafe {
				ffi::GetModuleFileNameW(self.ptr(), buf.as_mut_ptr(), buf.buf_len() as _)
			} {
				0 => return Err(GetLastError()),
				len => len,
			} + 1; // plus terminating null count

			if (copied as usize) < buf_sz {
				return Ok(buf.to_string()); // to break, must have at least 1 char gap
			}

			buf_sz += MAX_PATH; // increase buffer size to try again
		}
	}

	/// [`GetModuleHandle`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
	/// function.
	///
	/// # Examples
	///
	/// Retrieving current module instance:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hinstance = w::HINSTANCE::GetModuleHandle(None)?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn GetModuleHandle(module_name: Option<&str>) -> SysResult<HINSTANCE> {
		ptr_to_sysresult_handle(unsafe {
			ffi::GetModuleHandleW(WString::from_opt_str(module_name).as_ptr())
		})
	}

	/// [`GetModuleHandleEx`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandleexw)
	/// function.
	///
	/// The `GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS` is automatically managed by
	/// the function.
	///
	/// The `GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT` flag is never used,
	/// for safety reasons.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hinstance = w::HINSTANCE::GetModuleHandleEx(
	///     w::AddrStr::from_str("foo.dll"),
	///     co::GET_MODULE_HANDLE_EX_FLAG::NoValue,
	/// )?;
	///
	/// // FreeLibrary() called automatically
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn GetModuleHandleEx(
		addr_or_name: AddrStr,
		flags: co::GET_MODULE_HANDLE_EX_FLAG,
	) -> SysResult<FreeLibraryGuard> {
		let mut addr_or_name = addr_or_name;
		let (module_name, mut f) = match &mut addr_or_name {
			AddrStr::None => (std::ptr::null_mut(), 0),
			AddrStr::Addr(a) => (*a as _, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS),
			AddrStr::Str(ws) => (unsafe { ws.as_mut_ptr() }, 0),
		};
		f |= flags.raw();

		let mut h = unsafe { HINSTANCE::from_ptr(std::ptr::null_mut()) };
		unsafe {
			bool_to_sysresult(ffi::GetModuleHandleExW(f, module_name, h.as_mut()))
				.map(|_| FreeLibraryGuard::new(h))
		}
	}

	/// [`GetProcAddress`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)
	/// function.
	#[must_use]
	pub fn GetProcAddress(&self, proc_name: &str) -> SysResult<*const std::ffi::c_void> {
		ptr_to_sysresult(unsafe {
			ffi::GetProcAddress(self.ptr(), vec_ptr(&str_to_iso88591(proc_name))) as _
		})
		.map(|ptr| ptr as _)
	}

	/// [`LoadLibrary`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)
	/// function.
	#[must_use]
	pub fn LoadLibrary(lib_file_name: &str) -> SysResult<FreeLibraryGuard> {
		unsafe {
			ptr_to_sysresult_handle(ffi::LoadLibraryW(WString::from_str(lib_file_name).as_ptr()))
				.map(|h| FreeLibraryGuard::new(h))
		}
	}

	/// [`LoadResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadresource)
	/// function.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::HINSTANCE::LockResource).
	#[must_use]
	pub fn LoadResource(&self, res_info: &HRSRC) -> SysResult<HRSRCMEM> {
		ptr_to_sysresult_handle(unsafe { ffi::LoadResource(self.ptr(), res_info.ptr()) })
	}

	/// [`LockResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-lockresource)
	/// function.
	///
	/// This method should belong to [`HRSRCMEM`](crate::HRSRCMEM), but in order
	/// to make it safe, we automatically call
	/// [`HINSTANCE::SizeofResource`](crate::HINSTANCE::SizeofResource), so it's
	/// implemented here.
	///
	/// # Examples
	///
	/// The
	/// [Updating Resources](https://learn.microsoft.com/en-us/windows/win32/menurc/using-resources#updating-resources)
	/// example:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// const IDD_HAND_ABOUTBOX: u16 = 103;
	/// const IDD_FOOT_ABOUTBOX: u16 = 110;
	///
	/// let hexe = w::HINSTANCE::LoadLibrary("hand.exe")?;
	///
	/// let hres = hexe.FindResource(
	///     w::IdStr::Id(IDD_HAND_ABOUTBOX),
	///     w::RtStr::Rt(co::RT::DIALOG),
	/// )?;
	///
	/// let hres_load = hexe.LoadResource(&hres)?;
	/// let hres_slice_lock = hexe.LockResource(&hres, &hres_load)?;
	/// let hres_update = w::HUPDATERSRC::BeginUpdateResource("foot.exe", false)?;
	///
	/// hres_update.UpdateResource(
	///     w::RtStr::Rt(co::RT::DIALOG),
	///     w::IdStr::Id(IDD_FOOT_ABOUTBOX),
	///     w::LANGID::NEUTRAL,
	///     hres_slice_lock,
	/// )?;
	///
	/// // EndUpdateResource() called automatically
	///
	/// // FreeLibrary() called automatically
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn LockResource(&self, res_info: &HRSRC, hres_loaded: &HRSRCMEM) -> SysResult<&[u8]> {
		let sz = self.SizeofResource(res_info)?;
		unsafe {
			ptr_to_sysresult(ffi::LockResource(hres_loaded.ptr()))
				.map(|ptr| std::slice::from_raw_parts(ptr as _, sz as _))
		}
	}

	/// [`SizeofResource`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-sizeofresource)
	/// function.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::HINSTANCE::LockResource).
	#[must_use]
	pub fn SizeofResource(&self, res_info: &HRSRC) -> SysResult<u32> {
		match unsafe { ffi::SizeofResource(self.ptr(), res_info.ptr()) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}
}
