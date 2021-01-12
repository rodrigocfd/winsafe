use crate::co;
use crate::funcs::GetLastError;
use crate::msg::macros::{lparam_to_mut_ref, lparam_to_ref, ref_to_lparam};
use crate::msg::{Message, Wm};
use crate::structs::{BUTTON_IMAGELIST, SIZE};

/// [`BCM_GETIDEALSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getidealsize)
/// message parameters.
pub struct BcmGetIdealSize<'a> {
	pub size: &'a SIZE,
}

impl<'a> Message for BcmGetIdealSize<'a> {
	type RetType = Result<(), co::ERROR>;

	fn convert_ret(v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETIDEALSIZE,
			wparam: 0,
			lparam: ref_to_lparam(self.size),
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			size: lparam_to_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_GETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getimagelist)
/// message parameters.
pub struct BcmGetImageList<'a> {
	pub info: &'a mut BUTTON_IMAGELIST,
}

impl<'a> Message for BcmGetImageList<'a> {
	type RetType = Result<(), co::ERROR>;

	fn convert_ret(v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETIMAGELIST,
			wparam: 0,
			lparam: ref_to_lparam(self.info),
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			info: lparam_to_mut_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_CLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-click)
/// message parameters.

pub struct BmClick {}

impl Message for BmClick {
	type RetType = ();

	fn convert_ret(_: isize) -> Self::RetType {
		()
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::BM_CLICK,
			wparam: 0,
			lparam: 0,
		}
	}

	fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}

//------------------------------------------------------------------------------

/// [`BM_GETCHECK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-getcheck)
/// message parameters.
pub struct BmGetCheck {}

impl Message for BmGetCheck {
	type RetType = co::BST;

	fn convert_ret(v: isize) -> Self::RetType {
		co::BST::from(v as u32)
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::BM_GETCHECK,
			wparam: 0,
			lparam: 0,
		}
	}

	fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}

//------------------------------------------------------------------------------

/// [`BM_SETCHECK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setcheck)
/// message parameters.
pub struct BmSetCheck {
	pub state: co::BST,
}

impl Message for BmSetCheck {
	type RetType = ();

	fn convert_ret(_: isize) -> Self::RetType {
		()
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::BM_SETCHECK,
			wparam: u32::from(self.state) as usize,
			lparam: 0,
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			state: co::BST::from(p.wparam as u32),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_SETDONTCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setdontclick)
/// message parameters.
pub struct BmSetDontClick {
	pub dont_click: bool,
}

impl Message for BmSetDontClick {
	type RetType = ();

	fn convert_ret(_: isize) -> Self::RetType {
		()
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::BM_SETDONTCLICK,
			wparam: self.dont_click as usize,
			lparam: 0,
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			dont_click: p.wparam != 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_SETSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setstate)
/// message parameters.
pub struct BmSetState {
	pub highlight: bool,
}

impl Message for BmSetState {
	type RetType = ();

	fn convert_ret(_: isize) -> Self::RetType {
		()
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::BM_SETSTATE,
			wparam: self.highlight as usize,
			lparam: 0,
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			highlight: p.wparam != 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_SETSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setstyle)
/// message parameters.
pub struct BmSetStyle {
	pub style: co::BS,
	pub redraw: bool,
}

impl Message for BmSetStyle {
	type RetType = ();

	fn convert_ret(_: isize) -> Self::RetType {
		()
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::BM_SETSTYLE,
			wparam: u32::from(self.style) as usize,
			lparam: self.redraw as isize,
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			style: co::BS::from(p.wparam as u32),
			redraw: p.lparam != 0,
		}
	}
}
