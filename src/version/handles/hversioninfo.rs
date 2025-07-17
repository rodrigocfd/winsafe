#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::version::ffi;

handle! { HVERSIONINFO;
	/// Handle to a
	/// [version info](https://learn.microsoft.com/en-us/windows/win32/api/winver/nf-winver-getfileversioninfow)
	/// block.
	///
	/// Originally just a pointer to a memory block.
}

impl HVERSIONINFO {
	/// [`GetFileVersionInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winver/nf-winver-getfileversioninfow)
	/// function.
	///
	/// The returned buffer will be automatically allocated with
	/// [`HVERSIONINFO::GetFileVersionInfoSize`](crate::HVERSIONINFO::GetFileVersionInfoSize).
	#[must_use]
	pub fn GetFileVersionInfo(file_name: &str) -> SysResult<VersionInfoGuard> {
		let block_sz = Self::GetFileVersionInfoSize(file_name)?;
		let mut hglobal =
			HGLOBAL::GlobalAlloc(co::GMEM::FIXED | co::GMEM::ZEROINIT, block_sz as _)?;
		let hglobal_ptr = hglobal.leak();

		BoolRet(unsafe {
			ffi::GetFileVersionInfoW(
				WString::from_str(file_name).as_ptr(),
				0,
				block_sz,
				hglobal_ptr.ptr(),
			)
		})
		.to_sysresult()
		.map(|_| unsafe {
			VersionInfoGuard::new(
				HVERSIONINFO::from_ptr(hglobal_ptr.ptr()), // simply use the HGLOBAL pointer
			)
		})
	}

	/// [`GetFileVersionInfoSize`](https://learn.microsoft.com/en-us/windows/win32/api/winver/nf-winver-getfileversioninfosizew)
	/// function.
	///
	/// You don't need to call this function directly, because
	/// [`HVERSIONINFO::GetFileVersionInfo`](crate::HVERSIONINFO::GetFileVersionInfo)
	/// already calls it.
	#[must_use]
	pub fn GetFileVersionInfoSize(file_name: &str) -> SysResult<u32> {
		let mut dw_handle = 0u32;
		match unsafe {
			ffi::GetFileVersionInfoSizeW(WString::from_str(file_name).as_ptr(), &mut dw_handle)
		} {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}

	/// Calls
	/// [`HVERSIONINFO::VerQueryValue`](crate::HVERSIONINFO::VerQueryValue) to
	/// retrieve a reference to a slice with all languages and code pages.
	///
	/// # Examples
	///
	/// Listing all pairs of language and code page:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let exe_name = w::HINSTANCE::NULL.GetModuleFileName()?;
	/// let hversion = w::HVERSIONINFO::GetFileVersionInfo(&exe_name)?;
	///
	/// for (lang, cp) in hversion.langs_and_cps()?.iter() {
	///     println!("{} {}", lang, cp);
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn langs_and_cps(&self) -> SysResult<&[(LANGID, co::CP)]> {
		unsafe {
			self.VerQueryValue::<(LANGID, co::CP)>("\\VarFileInfo\\Translation")
				.map(|(pblocks, sz)| {
					std::slice::from_raw_parts(
						pblocks,
						sz as usize / std::mem::size_of::<(LANGID, co::CP)>(),
					)
				})
		}
	}

	/// [`VerQueryValue`](https://learn.microsoft.com/en-us/windows/win32/api/winver/nf-winver-verqueryvaluew)
	/// function.
	///
	/// # Safety
	///
	/// The returned pointer and size vary according to `lpSubBlock`. If you set
	/// it wrong, you're likely to cause a buffer overrun.
	///
	/// This is a low-level method, prefer using:
	/// * [`langs_and_cps`](crate::HVERSIONINFO::langs_and_cps);
	/// * [`str_val`](crate::HVERSIONINFO::str_val);
	/// * [`version_info`](crate::HVERSIONINFO::version_info).
	///
	/// # Examples
	///
	/// Reading version information from resource:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let exe_name = w::HINSTANCE::NULL.GetModuleFileName()?;
	/// let hversion = w::HVERSIONINFO::GetFileVersionInfo(&exe_name)?;
	///
	/// let (pvsf, sz_data) = unsafe {
	///     hversion.VerQueryValue::<w::VS_FIXEDFILEINFO>("\\")?
	/// };
	///
	/// let ver = unsafe { &*pvsf }.dwFileVersion();
	/// println!("Version {}.{}.{}.{}",
	///     ver[0], ver[1], ver[2], ver[3]);
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub unsafe fn VerQueryValue<T>(&self, sub_block: &str) -> SysResult<(*const T, u32)> {
		let mut lp_lp_buffer = std::ptr::null();
		let mut pu_len = 0;

		BoolRet(unsafe {
			ffi::VerQueryValueW(
				self.ptr(),
				WString::from_str(sub_block).as_ptr(),
				pvoid(&mut lp_lp_buffer),
				&mut pu_len,
			)
		})
		.to_sysresult()
		.map(|_| (lp_lp_buffer as *const T, pu_len))
	}

	/// Calls
	/// [`HVERSIONINFO::VerQueryValue`](crate::HVERSIONINFO::VerQueryValue)
	/// to retrieve a string value.
	///
	/// Common value names are:
	/// * Comments
	/// * CompanyName
	/// * FileDescription
	/// * FileVersion
	/// * InternalName
	/// * LegalCopyright
	/// * LegalTrademarks
	/// * OriginalFilename
	/// * ProductName
	/// * ProductVersion
	/// * PrivateBuild
	/// * SpecialBuild
	///
	/// # Examples
	///
	/// Reading product name and legal copyright from resource:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let exe_name = w::HINSTANCE::NULL.GetModuleFileName()?;
	/// let hversion = w::HVERSIONINFO::GetFileVersionInfo(&exe_name)?;
	///
	/// let lang_cp = hversion.langs_and_cps()?[0]; // first language and code page
	///
	/// println!(
	///     "{}\n{}",
	///     hversion.str_val(lang_cp, "ProductName")?,
	///     hversion.str_val(lang_cp, "LegalCopyright")?,
	/// );
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn str_val(&self, lang_and_cp: (LANGID, co::CP), name: &str) -> SysResult<String> {
		unsafe {
			self.VerQueryValue::<u16>(&format!(
				"\\StringFileInfo\\{:04x}{:04x}\\{}",
				u16::from(lang_and_cp.0),
				u16::from(lang_and_cp.1),
				name
			))
			.map(|(pstr, len)| {
				WString::from_wchars_slice(std::slice::from_raw_parts(pstr, len as _)).to_string()
			})
		}
	}

	/// Calls
	/// [`HVERSIONINFO::VerQueryValue`](crate::HVERSIONINFO::VerQueryValue) to
	/// retrieve a reference to the fixed version block, if any.
	#[must_use]
	pub fn version_info(&self) -> SysResult<&VS_FIXEDFILEINFO> {
		unsafe {
			self.VerQueryValue::<VS_FIXEDFILEINFO>("\\")
				.map(|(p, _)| &*p)
		}
	}
}
