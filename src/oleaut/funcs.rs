#![allow(non_snake_case)]

use crate::{co, oleaut};
use crate::kernel::decl::{SysResult, SYSTEMTIME, WString};
use crate::ole::decl::{CoTaskMemFree, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::PROPERTYKEY;

/// [`PSGetNameFromPropertyKey`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-psgetnamefrompropertykey)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
#[must_use]
pub fn PSGetNameFromPropertyKey(prop_key: &PROPERTYKEY) -> HrResult<String> {
	let mut pstr: *mut u16 = std::ptr::null_mut();
	ok_to_hrresult(
		unsafe {
			oleaut::ffi::PSGetNameFromPropertyKey(
				prop_key as *const _ as _,
				&mut pstr,
			)
		},
	).map(|_| {
		let name = WString::from_wchars_nullt(pstr);
		CoTaskMemFree(pstr as _);
		name.to_string()
	})
}

/// [`SystemTimeToVariantTime`](https://docs.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-systemtimetovarianttime)
/// function. The inverse operation is performed by
/// [`VariantTimeToSystemTime`](crate::VariantTimeToSystemTime).
///
/// Note that this function resolves the time to one second; milliseconds are
/// ignored.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
#[must_use]
pub fn SystemTimeToVariantTime(st: &SYSTEMTIME) -> SysResult<f64> {
	let mut double = f64::default();
	match unsafe {
		oleaut::ffi::SystemTimeToVariantTime(st as *const _ as _, &mut double)
	} {
		0 => Err(co::ERROR::INVALID_PARAMETER),
		_ => Ok(double),
	}
}

/// [`VariantTimeToSystemTime`](https://docs.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-varianttimetosystemtime)
/// function. The inverse operation is performed by
/// [`SystemTimeToVariantTime`](SystemTimeToVariantTime).
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
#[must_use]
pub fn VariantTimeToSystemTime(
	var_time: f64, st: &mut SYSTEMTIME) -> SysResult<()>
{
	match unsafe {
		oleaut::ffi::VariantTimeToSystemTime(var_time, st as *mut _ as _)
	} {
		0 => Err(co::ERROR::INVALID_PARAMETER),
		_ => Ok(()),
	}
}
