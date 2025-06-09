#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::oleaut::ffi;
use crate::prelude::*;

/// [`OleLoadPicture`](https://learn.microsoft.com/en-us/windows/win32/api/olectl/nf-olectl-oleloadpicture)
/// function.
///
/// # Examples
///
/// Parsing an image from raw data:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let stream: w::IStream; // initialized somewhere
/// # let stream = unsafe { w::IStream::null() };
///
/// let picture = w::OleLoadPicture(&stream, None, true)?;
/// # w::HrResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`OleLoadPicturePath`](crate::OleLoadPicturePath)
#[must_use]
pub fn OleLoadPicture(
	stream: &impl ole_IStream,
	size: Option<u32>,
	keep_original_format: bool,
) -> HrResult<IPicture> {
	let mut queried = unsafe { IPicture::null() };
	ok_to_hrresult(unsafe {
		ffi::OleLoadPicture(
			stream.ptr() as _,
			size.unwrap_or_default() as _,
			!keep_original_format as _, // note: reversed
			pcvoid(&IPicture::IID),
			queried.as_mut(),
		)
	})
	.map(|_| queried)
}

/// [`OleLoadPicturePath`](https://learn.microsoft.com/en-us/windows/win32/api/olectl/nf-olectl-oleloadpicturepath)
/// function.
///
/// The picture must be in BMP (bitmap), JPEG, WMF (metafile), ICO (icon), or
/// GIF format.
///
/// # Related functions
///
/// * [`OleLoadPicture`](crate::OleLoadPicture)
#[must_use]
pub fn OleLoadPicturePath(path: &str, transparent_color: Option<COLORREF>) -> HrResult<IPicture> {
	let mut queried = unsafe { IPicture::null() };
	ok_to_hrresult(unsafe {
		ffi::OleLoadPicturePath(
			WString::from_str(path).as_ptr(),
			std::ptr::null_mut(),
			0,
			transparent_color.map_or(0, |c| c.into()),
			pcvoid(&IPicture::IID),
			queried.as_mut(),
		)
	})
	.map(|_| queried)
}

/// [`PSGetNameFromPropertyKey`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-psgetnamefrompropertykey)
/// function.
#[must_use]
pub fn PSGetNameFromPropertyKey(prop_key: &co::PKEY) -> HrResult<String> {
	let mut pstr = std::ptr::null_mut::<u16>();
	ok_to_hrresult(unsafe { ffi::PSGetNameFromPropertyKey(pcvoid(prop_key), &mut pstr) }).map(
		|_| {
			let name = unsafe { WString::from_wchars_nullt(pstr) };
			let _ = unsafe { CoTaskMemFreeGuard::new(pstr as _, 0) };
			name.to_string()
		},
	)
}

/// [`SystemTimeToVariantTime`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-systemtimetovarianttime)
/// function.
///
/// Note that this function resolves the time to one second; milliseconds are
/// ignored.
///
/// # Related functions
///
/// * [`VariantTimeToSystemTime`](crate::VariantTimeToSystemTime)
#[must_use]
pub fn SystemTimeToVariantTime(st: &SYSTEMTIME) -> SysResult<f64> {
	let mut double = f64::default();
	bool_to_invalidparm(unsafe { ffi::SystemTimeToVariantTime(pcvoid(st), &mut double) })
		.map(|_| double)
}

/// [`VariantTimeToSystemTime`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-varianttimetosystemtime)
/// function.
///
/// # Related functions
///
/// * [`SystemTimeToVariantTime`](crate::SystemTimeToVariantTime)
#[must_use]
pub fn VariantTimeToSystemTime(var_time: f64) -> SysResult<SYSTEMTIME> {
	let mut st = SYSTEMTIME::default();
	bool_to_invalidparm(unsafe { ffi::VariantTimeToSystemTime(var_time, pvoid(&mut st)) })
		.map(|_| st)
}
