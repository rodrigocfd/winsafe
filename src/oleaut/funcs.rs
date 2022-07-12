#![allow(non_snake_case)]

use crate::kernel::decl::WString;
use crate::ole::decl::{CoTaskMemFree, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut;
use crate::oleaut::decl::PROPERTYKEY;

/// [`PSGetNameFromPropertyKey`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-psgetnamefrompropertykey)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
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
		CoTaskMemFree(pstr);
		name.to_string()
	})
}
