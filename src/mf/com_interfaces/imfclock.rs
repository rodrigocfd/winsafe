#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFClock: "2eb1e945-18b8-4139-9b1a-d5d584818530";
	/// [`IMFClock`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfclock)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFClock for IMFClock {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFClock`](crate::IMFClock).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFClock: ole_IUnknown {
	/// [`IMFClock::GetClockCharacteristics`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfclock-getclockcharacteristics)
	/// method.
	#[must_use]
	fn GetClockCharacteristics(&self) -> HrResult<co::MFCLOCK_CHARACTERISTICS_FLAG> {
		let mut characteristics = co::MFCLOCK_CHARACTERISTICS_FLAG::default();
		HrRet(unsafe {
			(vt::<IMFClockVT>(self).GetClockCharacteristics)(self.ptr(), characteristics.as_mut())
		})
		.to_hrresult()
		.map(|_| characteristics)
	}

	/// [`IMFClock::GetContinuityKey`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfclock-getcontinuitykey)
	/// method.
	#[must_use]
	fn GetContinuityKey(&self) -> HrResult<u32> {
		let mut ck = 0u32;
		HrRet(unsafe { (vt::<IMFClockVT>(self).GetContinuityKey)(self.ptr(), &mut ck) })
			.to_hrresult()
			.map(|_| ck)
	}

	/// [`IMFClock::GetCorrelatedTime`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfclock-getcorrelatedtime)
	/// method.
	///
	/// Returns the last known clock time (in units of the clock's frequency)
	/// and the system time that corresponds to the last known clock time (in
	/// 100-nanosecond units).
	#[must_use]
	fn GetCorrelatedTime(&self) -> HrResult<(i64, i64)> {
		let (mut clock, mut system) = (0i64, 0i64);
		HrRet(unsafe {
			(vt::<IMFClockVT>(self).GetCorrelatedTime)(self.ptr(), 0, &mut clock, &mut system)
		})
		.to_hrresult()
		.map(|_| (clock, system))
	}

	/// [`IMFClock::GetProperties`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfclock-getproperties)
	/// method.
	#[must_use]
	fn GetProperties(&self) -> HrResult<MFCLOCK_PROPERTIES> {
		let mut cp = MFCLOCK_PROPERTIES::default();
		HrRet(unsafe { (vt::<IMFClockVT>(self).GetProperties)(self.ptr(), pvoid(&mut cp)) })
			.to_hrresult()
			.map(|_| cp)
	}

	/// [`IMFClock::GetState`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfclock-getstate)
	/// method.
	#[must_use]
	fn GetState(&self) -> HrResult<co::MFCLOCK_STATE> {
		let mut state = co::MFCLOCK_STATE::default();
		HrRet(unsafe { (vt::<IMFClockVT>(self).GetState)(self.ptr(), 0, state.as_mut()) })
			.to_hrresult()
			.map(|_| state)
	}
}
