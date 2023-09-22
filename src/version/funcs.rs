#![allow(non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::version::ffi;

/// [`GetFileVersionInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winver/nf-winver-getfileversioninfow)
/// function.
///
/// The returned buffer will be automatically allocated with
/// [`GetFileVersionInfoSize`](crate::GetFileVersionInfoSize).
#[must_use]
pub fn GetFileVersionInfo(file_name: &str) -> SysResult<HeapBlock> {
	let block_sz = GetFileVersionInfoSize(file_name)?;
	let mut buf = HeapBlock::alloc(block_sz as _)?;

	bool_to_sysresult(
		unsafe {
			ffi::GetFileVersionInfoW(
				WString::from_str(file_name).as_ptr(),
				0,
				buf.len() as _,
				buf.as_mut_ptr() as _,
			)
		},
	).map(|_| buf)
}

/// [`GetFileVersionInfoSize`](https://learn.microsoft.com/en-us/windows/win32/api/winver/nf-winver-getfileversioninfosizew)
/// function.
#[must_use]
pub fn GetFileVersionInfoSize(file_name: &str) -> SysResult<u32> {
	let mut dw_handle = u32::default();
	match unsafe {
		ffi::GetFileVersionInfoSizeW(
			WString::from_str(file_name).as_ptr(),
			&mut dw_handle,
		)
	} {
		0 => Err(GetLastError()),
		sz => Ok(sz)
	}
}

/// [`VarQueryValue`](https://learn.microsoft.com/en-us/windows/win32/api/winver/nf-winver-verqueryvaluew)
/// function.
///
/// # Safety
///
/// The returned pointer and size vary according to `lpSubBlock`. If you set it
/// wrong, you're likely to cause a buffer overrun.
///
/// This function is rather tricky, consider using
/// [`ResourceInfo`](crate::ResourceInfo).
///
/// # Examples
///
/// Reading version information from resource:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let exe_name = w::HINSTANCE::NULL.GetModuleFileName()?;
/// let res_buf = w::GetFileVersionInfo(&exe_name)?;
///
/// let (pvsf, sz_data) = unsafe {
///     w::VarQueryValue::<w::VS_FIXEDFILEINFO>(
///         res_buf.as_slice(),
///         "\\",
///     )?
/// };
///
/// let ver = unsafe { &*pvsf }.dwFileVersion();
/// println!("Version {}.{}.{}.{}",
///     ver[0], ver[1], ver[2], ver[3]);
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub unsafe fn VarQueryValue<T>(
	block: &[u8],
	sub_block: &str,
) -> SysResult<(*const T, u32)>
{
	let mut lp_lp_buffer = std::ptr::null();
	let mut pu_len = 0;

	bool_to_sysresult(
		ffi::VerQueryValueW(
			vec_ptr(block) as _,
			WString::from_str(sub_block).as_ptr(),
			&mut lp_lp_buffer as *mut _ as _,
			&mut pu_len,
		),
	).map(|_| (lp_lp_buffer as *const T, pu_len))
}
