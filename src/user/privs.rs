#![allow(dead_code)]

use crate::co;
use crate::decl::*;

const_values_num_privs! {
	ASFW_ANY u32 = -1i32 as _
	CB_ERR i32 = -1
	CB_ERRSPACE i32 = -2
	CCHDEVICENAME usize = 32
	CCHFORMNAME usize = 32
	CCHILDREN_TITLEBAR usize = 5
	DM_SPECVERSION u16 = 0x0401
	FAPPCOMMAND_MASK u16 = 0xf000
	HWND_MESSAGE isize = -3
	LB_ERR i32 = -1
	LB_ERRSPACE i32 = -2
	WC_DIALOG u16 = 0x8002
}

/// Takes an `isize` and returns `Err` if `-1`.
#[must_use]
pub(crate) const fn minus1_as_badargs(v: isize) -> SysResult<isize> {
	match v {
		-1 => Err(co::ERROR::BAD_ARGUMENTS), // all message errors will return this code
		v => Ok(v),
	}
}

/// Takes an `isize` and returns `None` if `-1`.
#[must_use]
pub(crate) const fn minus1_as_none(v: isize) -> Option<isize> {
	match v {
		-1 => None,
		v => Some(v),
	}
}

/// Takes an `isize` and returns `Err` if zero.
#[must_use]
pub(crate) const fn zero_as_badargs(v: isize) -> SysResult<isize> {
	match v {
		0 => Err(co::ERROR::BAD_ARGUMENTS), // all message errors will return this code
		v => Ok(v),
	}
}

/// Takes an `isize` and returns `None` if zero.
#[must_use]
pub(crate) const fn zero_as_none(v: isize) -> Option<isize> {
	match v {
		0 => None,
		v => Some(v),
	}
}
