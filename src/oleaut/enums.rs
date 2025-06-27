use std::mem::ManuallyDrop;

use crate::co;
use crate::decl::*;
use crate::prelude::*;

/// High-level representation of the [`PROPVARIANT`](crate::PROPVARIANT) struct,
/// which is automatically converted into its low-level representation when
/// needed.
///
/// Not to be confused with the similar [`Variant`].
///
/// The fields are named according to the
/// [`VARENUM`](https://learn.microsoft.com/en-us/windows/win32/api/wtypes/ne-wtypes-varenum)
/// enumeration.
#[derive(Clone)]
pub enum PropVariant {
	/// Nothing.
	Empty,
	/// 2 byte signed int (`i16`).
	I2(i16),
	/// 4 byte signed int (`i32`).
	I4(i32),
	/// 4 byte real (`f32`).
	R4(f32),
	/// 8 byte real (`f64`).
	R8(f64),
	/// Date.
	Date(SYSTEMTIME),
	/// OLE Automation string.
	Bstr(String),
	/// [`IDispatch`](crate::IDispatch) pointer.
	Dispatch(IDispatch),
	/// Boolean value (`bool`).
	Bool(bool),
	/// [`IUnknown`](crate::IUnknown) pointer.
	Unknown(IUnknown),
	/// Signed char (`i8`).
	I1(i8),
	/// Unsigned char (`u8`).
	UI1(u8),
	/// Unsigned short (`u16`).
	UI2(u16),
	/// Unsigned long (`u32`).
	UI4(u32),
	/// Signed 64-bit int (`i64`).
	I8(i64),
	/// Unsigned 64-bit int (`u64`).
	UI8(u64),
}

impl Default for PropVariant {
	fn default() -> Self {
		Self::Empty
	}
}

impl PropVariant {
	/// Constructs a [`PropVariant::Bstr`](crate::PropVariant::Bstr) from a
	/// string.
	#[must_use]
	pub fn from_str(s: impl AsRef<str>) -> Self {
		Self::Bstr(s.as_ref().to_owned())
	}

	/// Constructs a [`PropVariant::Bstr`](crate::PropVariant::Bstr) if a string
	/// is present, otherwise creates a
	/// [`PropVariant::Empty`](crate::PropVariant::Empty).
	#[must_use]
	pub fn from_opt_str(s: Option<impl AsRef<str>>) -> Self {
		match s {
			Some(s) => Self::Bstr(s.as_ref().to_owned()),
			None => Self::Empty,
		}
	}

	#[must_use]
	pub(crate) fn from_raw(v: &PROPVARIANT) -> HrResult<Self> {
		Ok(match v.vt {
			co::VT::EMPTY => Self::Empty,
			co::VT::I2 => Self::I2(unsafe { v.data.iVal }),
			co::VT::I4 => Self::I4(unsafe { v.data.lVal }),
			co::VT::R4 => Self::R4(unsafe { v.data.fltVal }),
			co::VT::R8 => Self::R8(unsafe { v.data.dblVal }),
			co::VT::DATE => {
				let st = VariantTimeToSystemTime(unsafe { v.data.dblVal })
					.map_err(|err| err.to_hresult())?;
				Self::Date(st)
			},
			co::VT::BSTR => {
				let bstr = ManuallyDrop::new(unsafe { BSTR::from_ptr(v.data.ptr as _) }); // won't release the stored pointer
				Self::Bstr(bstr.to_string())
			},
			co::VT::DISPATCH => {
				let obj = ManuallyDrop::new(unsafe { IDispatch::from_ptr(v.data.ptr as _) }); // won't release the stored pointer
				let cloned = IDispatch::clone(&obj); // call AddRef
				Self::Dispatch(cloned)
			},
			co::VT::BOOL => Self::Bool(unsafe { v.data.iVal != 0 }),
			co::VT::UNKNOWN => {
				let obj = ManuallyDrop::new(unsafe { IUnknown::from_ptr(v.data.ptr as _) }); // won't release the stored pointer
				let cloned = IUnknown::clone(&obj); // call AddRef
				Self::Unknown(cloned)
			},
			co::VT::I1 => Self::I1(unsafe { v.data.cVal }),
			co::VT::UI1 => Self::UI1(unsafe { v.data.bVal }),
			co::VT::UI2 => Self::UI2(unsafe { v.data.uiVal }),
			co::VT::UI4 => Self::UI4(unsafe { v.data.ulVal }),
			co::VT::I8 => Self::I8(unsafe { v.data.hVal }),
			co::VT::UI8 => Self::UI8(unsafe { v.data.uhVal }),
			_ => panic!("PROPVARIANT type not implemented."),
		})
	}

	#[must_use]
	pub(crate) fn to_raw(&self) -> HrResult<PROPVARIANT> {
		let mut v = PROPVARIANT::default();
		match self {
			Self::Empty => {},
			Self::I2(n) => {
				v.vt = co::VT::I2;
				v.data.iVal = *n;
			},
			Self::I4(n) => {
				v.vt = co::VT::I4;
				v.data.lVal = *n;
			},
			Self::R4(n) => {
				v.vt = co::VT::R4;
				v.data.fltVal = *n;
			},
			Self::R8(n) => {
				v.vt = co::VT::R8;
				v.data.dblVal = *n;
			},
			Self::Date(st) => {
				v.vt = co::VT::DATE;
				v.data.dblVal = SystemTimeToVariantTime(st).map_err(|err| err.to_hresult())?;
			},
			Self::Bstr(s) => {
				v.vt = co::VT::BSTR;
				let mut bstr = BSTR::SysAllocString(s)?;
				v.data.ptr = bstr.leak() as _; // the VARIANT will own the pointer
			},
			Self::Dispatch(p) => {
				v.vt = co::VT::DISPATCH;
				let mut cloned = p.clone(); // call AddRef
				v.data.ptr = cloned.leak(); // the VARIANT will own the pointer
			},
			Self::Bool(b) => {
				v.vt = co::VT::BOOL;
				v.data.iVal = if *b { -1 } else { 0 };
			},
			Self::Unknown(p) => {
				v.vt = co::VT::UNKNOWN;
				let mut cloned = p.clone(); // call AddRef
				v.data.ptr = cloned.leak(); // the VARIANT will own the pointer
			},
			Self::I1(n) => {
				v.vt = co::VT::I1;
				v.data.cVal = *n;
			},
			Self::UI1(n) => {
				v.vt = co::VT::UI1;
				v.data.bVal = *n;
			},
			Self::UI2(n) => {
				v.vt = co::VT::UI2;
				v.data.uiVal = *n;
			},
			Self::UI4(n) => {
				v.vt = co::VT::UI4;
				v.data.ulVal = *n;
			},
			Self::I8(n) => {
				v.vt = co::VT::I8;
				v.data.hVal = *n;
			},
			Self::UI8(n) => {
				v.vt = co::VT::UI8;
				v.data.uhVal = *n;
			},
		}
		Ok(v)
	}
}

/// High-level representation of the [`VARIANT`](crate::VARIANT) struct, which
/// is automatically converted into its low-level representation when needed.
///
/// Not to be confused with the similar [`PropVariant`].
///
/// The fields are named according to the
/// [`VARENUM`](https://learn.microsoft.com/en-us/windows/win32/api/wtypes/ne-wtypes-varenum)
/// enumeration.
#[derive(Clone)]
pub enum Variant {
	/// Nothing.
	Empty,
	/// 2 byte signed int (`i16`).
	I2(i16),
	/// 4 byte signed int (`i32`).
	I4(i32),
	/// 4 byte real (`f32`).
	R4(f32),
	/// 8 byte real (`f64`).
	R8(f64),
	/// Date.
	Date(SYSTEMTIME),
	/// OLE Automation string.
	Bstr(String),
	/// [`IDispatch`](crate::IDispatch) pointer.
	Dispatch(IDispatch),
	/// Boolean value (`bool`).
	Bool(bool),
	/// [`IUnknown`](crate::IUnknown) pointer.
	Unknown(IUnknown),
	/// Signed char (`i8`).
	I1(i8),
	/// Unsigned char (`u8`).
	UI1(u8),
	/// Unsigned short (`u16`).
	UI2(u16),
	/// Unsigned long (`u32`).
	UI4(u32),
	/// Signed 64-bit int (`i64`).
	I8(i64),
	/// Unsigned 64-bit int (`u64`).
	UI8(u64),
}

impl Default for Variant {
	fn default() -> Self {
		Self::Empty
	}
}

impl Variant {
	/// Creates a [`Variant::Bstr`](crate::Variant::Bstr) from a string.
	#[must_use]
	pub fn from_str(s: impl AsRef<str>) -> Self {
		Self::Bstr(s.as_ref().to_owned())
	}

	/// Creates a [`Variant::Bstr`](crate::Variant::Bstr) if a string is
	/// present, otherwise creates a [`Variant::Empty`](crate::Variant::Empty).
	#[must_use]
	pub fn from_opt_str(s: Option<impl AsRef<str>>) -> Self {
		match s {
			Some(s) => Self::Bstr(s.as_ref().to_owned()),
			None => Self::Empty,
		}
	}

	#[allow(unused)]
	#[must_use]
	pub(crate) fn from_raw(v: &VARIANT) -> HrResult<Self> {
		Ok(match v.vt {
			co::VT::EMPTY => Self::Empty,
			co::VT::I2 => Self::I2(unsafe { v.data.iVal }),
			co::VT::I4 => Self::I4(unsafe { v.data.lVal }),
			co::VT::R4 => Self::R4(unsafe { v.data.fltVal }),
			co::VT::R8 => Self::R8(unsafe { v.data.dblVal }),
			co::VT::DATE => {
				let st = VariantTimeToSystemTime(unsafe { v.data.dblVal })
					.map_err(|err| err.to_hresult())?;
				Self::Date(st)
			},
			co::VT::BSTR => {
				let bstr = ManuallyDrop::new(unsafe { BSTR::from_ptr(v.data.ptr as _) }); // won't release the stored pointer
				Self::Bstr(bstr.to_string())
			},
			co::VT::DISPATCH => {
				let obj = ManuallyDrop::new(unsafe { IDispatch::from_ptr(v.data.ptr as _) }); // won't release the stored pointer
				let cloned = IDispatch::clone(&obj); // call AddRef
				Self::Dispatch(cloned)
			},
			co::VT::BOOL => Self::Bool(unsafe { v.data.iVal != 0 }),
			co::VT::UNKNOWN => {
				let obj = ManuallyDrop::new(unsafe { IUnknown::from_ptr(v.data.ptr as _) }); // won't release the stored pointer
				let cloned = IUnknown::clone(&obj); // call AddRef
				Self::Unknown(cloned)
			},
			co::VT::I1 => Self::I1(unsafe { v.data.cVal }),
			co::VT::UI1 => Self::UI1(unsafe { v.data.bVal }),
			co::VT::UI2 => Self::UI2(unsafe { v.data.uiVal }),
			co::VT::UI4 => Self::UI4(unsafe { v.data.ulVal }),
			co::VT::I8 => Self::I8(unsafe { v.data.llVal }),
			co::VT::UI8 => Self::UI8(unsafe { v.data.ullVal }),
			_ => panic!("VARIANT type not implemented."),
		})
	}

	#[allow(unused)]
	#[must_use]
	pub(crate) fn to_raw(&self) -> HrResult<VARIANT> {
		let mut v = VARIANT::default();
		match self {
			Self::Empty => {},
			Self::I2(n) => {
				v.vt = co::VT::I2;
				v.data.iVal = *n;
			},
			Self::I4(n) => {
				v.vt = co::VT::I4;
				v.data.lVal = *n;
			},
			Self::R4(n) => {
				v.vt = co::VT::R4;
				v.data.fltVal = *n;
			},
			Self::R8(n) => {
				v.vt = co::VT::R8;
				v.data.dblVal = *n;
			},
			Self::Date(st) => {
				v.vt = co::VT::DATE;
				v.data.dblVal = SystemTimeToVariantTime(st).map_err(|err| err.to_hresult())?;
			},
			Self::Bstr(s) => {
				v.vt = co::VT::BSTR;
				let mut bstr = BSTR::SysAllocString(s)?;
				v.data.ptr = bstr.leak() as _; // the VARIANT will own the pointer
			},
			Self::Dispatch(p) => {
				v.vt = co::VT::DISPATCH;
				let mut cloned = p.clone(); // call AddRef
				v.data.ptr = cloned.leak(); // the VARIANT will own the pointer
			},
			Self::Bool(b) => {
				v.vt = co::VT::BOOL;
				v.data.iVal = if *b { -1 } else { 0 };
			},
			Self::Unknown(p) => {
				v.vt = co::VT::UNKNOWN;
				let mut cloned = p.clone(); // call AddRef
				v.data.ptr = cloned.leak(); // the VARIANT will own the pointer
			},
			Self::I1(n) => {
				v.vt = co::VT::I1;
				v.data.cVal = *n;
			},
			Self::UI1(n) => {
				v.vt = co::VT::UI1;
				v.data.bVal = *n;
			},
			Self::UI2(n) => {
				v.vt = co::VT::UI2;
				v.data.uiVal = *n;
			},
			Self::UI4(n) => {
				v.vt = co::VT::UI4;
				v.data.ulVal = *n;
			},
			Self::I8(n) => {
				v.vt = co::VT::I8;
				v.data.llVal = *n;
			},
			Self::UI8(n) => {
				v.vt = co::VT::UI8;
				v.data.ullVal = *n;
			},
		}
		Ok(v)
	}

	/// If the value is [`Variant::Bool`](crate::Variant::Bool), returns it;
	/// otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// `Variant` content, its general use is discouraged.
	#[must_use]
	pub const fn unwrap_bool(&self) -> bool {
		match self {
			Self::Bool(b) => *b,
			_ => panic!("Variant does not contain Bool."),
		}
	}

	/// If the value is [`Variant::Bstr`](crate::Variant::Bstr), returns a clone
	/// of it; otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// `Variant` content, its general use is discouraged.
	#[must_use]
	pub fn unwrap_bstr(&self) -> String {
		match self {
			Self::Bstr(s) => s.clone(),
			_ => panic!("Variant does not contain Bstr."),
		}
	}

	/// If the value is [`Variant::Dispatch`](crate::Variant::Dispatch), returns
	/// a clone of it; otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// `Variant` content, its general use is discouraged.
	#[must_use]
	pub fn unwrap_dispatch(&self) -> IDispatch {
		match self {
			Self::Dispatch(disp) => disp.clone(),
			_ => panic!("Variant does not contain Dispatch."),
		}
	}
}
