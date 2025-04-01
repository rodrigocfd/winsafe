use std::mem::ManuallyDrop;

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::prelude::*;

/// Returns a reference to the virtual table of the COM object.
///
/// # Safety
///
/// Make sure the inheritance is correct.
#[must_use]
pub(crate) unsafe fn vt<T>(obj: &impl ole_IUnknown) -> &T {
	let ppvt = obj.ptr() as *mut *mut T;
	&**ppvt
}

/// Given the pointer to the memory block, converts it to the `Box` of the
/// allocated VT struct.
#[must_use]
pub(crate) fn box_impl_of<T>(p: COMPTR) -> ManuallyDrop<Box<T>> {
	let pp = p as *mut *mut T;
	let box_impl = ManuallyDrop::new(unsafe { Box::from_raw(*pp) });
	box_impl
}

/// If value is `S_OK` yields `Ok()`, othersize `Err(hresult)`.
#[must_use]
pub(crate) const fn ok_to_hrresult(hr: HRES) -> HrResult<()> {
	match unsafe { co::HRESULT::from_raw(hr) } {
		co::HRESULT::S_OK => Ok(()),
		hr => Err(hr),
	}
}

/// If value is `S_OK` yields `Ok(true)`, if `S_FALSE` yields `Ok(false)`
/// othersize `Err(hresult)`.
#[must_use]
pub(crate) const fn okfalse_to_hrresult(hr: HRES) -> HrResult<bool> {
	match unsafe { co::HRESULT::from_raw(hr) } {
		co::HRESULT::S_OK => Ok(true),
		co::HRESULT::S_FALSE => Ok(false),
		hr => Err(hr),
	}
}

/// If value is `Ok` yields 0, otherwise the error code.
#[must_use]
pub(crate) fn hrresult_to_hres<T>(hrr: HrResult<T>) -> HRES {
	match hrr {
		Ok(_) => co::HRESULT::S_OK.raw(),
		Err(e) => e.raw(),
	}
}

/// If the error is `HrResult`, returns it, otherwise displays a message box
/// with the error and returns `ERROR::E_UNEXPECTED`.
#[must_use]
pub(crate) fn anyresult_to_hresult<T>(res: AnyResult<T>) -> HrResult<T> {
	res.map_err(|err| {
		if let Some(hr) = err.downcast_ref::<co::HRESULT>() {
			*hr
		} else {
			HWND::NULL
				.MessageBox(
					&format!("Unhandled error: {}", err.to_string()),
					"Unhandled error in COM impl",
					co::MB::ICONERROR,
				)
				.unwrap();
			co::HRESULT::E_UNEXPECTED
		}
	})
}
