use crate::co;
use crate::decl::*;
use crate::macros::*;
use crate::msg::*;
use crate::prelude::*;

pub_struct_msg_empty! { BmClick: co::BM::CLICK.into();
	/// [`BM_CLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-click)
}

/// [`BM_GETCHECK`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-getcheck)
/// message parameters.
///
/// Return type: `co::BST`.
pub struct BmGetCheck {}

impl MsgSend for BmGetCheck {
	type RetType = co::BST;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::BST::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BM::GETCHECK.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`BM_GETIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-getimage)
/// message parameters.
///
/// Return type: `SysResult<BmpIcon>`.
pub struct BmGetImage {
	pub img_type: co::IMAGE_TYPE,
}

impl MsgSend for BmGetImage {
	type RetType = SysResult<BmpIcon>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match self.img_type {
			co::IMAGE_TYPE::BITMAP => Ok(BmpIcon::Bmp(unsafe { HBITMAP::from_ptr(v as _) })),
			co::IMAGE_TYPE::ICON => Ok(BmpIcon::Icon(unsafe { HICON::from_ptr(v as _) })),
			_ => Err(co::ERROR::BAD_ARGUMENTS),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BM::GETIMAGE.into(),
			wparam: self.img_type.raw() as _,
			lparam: 0,
		}
	}
}

/// [`BM_GETSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-getstate)
/// message, which has no parameters.
///
/// Return type: `co::BST`.
pub struct BmGetState {}

impl MsgSend for BmGetState {
	type RetType = co::BST;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::BST::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BM::GETSTATE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`BM_SETCHECK`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-setcheck)
/// message parameters.
///
/// Return type: `()`.
pub struct BmSetCheck {
	pub state: co::BST,
}

impl MsgSend for BmSetCheck {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BM::SETCHECK.into(),
			wparam: self.state.raw() as _,
			lparam: 0,
		}
	}
}

/// [`BM_SETDONTCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-setdontclick)
/// message parameters.
///
/// Return type: `()`.
pub struct BmSetDontClick {
	pub dont_click: bool,
}

impl MsgSend for BmSetDontClick {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BM::SETDONTCLICK.into(),
			wparam: self.dont_click as _,
			lparam: 0,
		}
	}
}

/// [`BM_SETIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-setimage)
/// message parameters.
///
/// Return type: `SysResult<BmpIcon>`.
pub struct BmSetImage {
	pub image: BmpIcon,
}

impl MsgSend for BmSetImage {
	type RetType = SysResult<BmpIcon>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		if v == 0 {
			Err(co::ERROR::BAD_ARGUMENTS)
		} else {
			match self.image {
				BmpIcon::Bmp(_) => Ok(BmpIcon::Bmp(unsafe { HBITMAP::from_ptr(v as _) })),
				BmpIcon::Icon(_) => Ok(BmpIcon::Icon(unsafe { HICON::from_ptr(v as _) })),
			}
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BM::SETIMAGE.into(),
			wparam: match self.image {
				BmpIcon::Bmp(_) => co::IMAGE_TYPE::BITMAP,
				BmpIcon::Icon(_) => co::IMAGE_TYPE::ICON,
			}
			.raw() as _,
			lparam: self.image.as_isize(),
		}
	}
}

/// [`BM_SETSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-setstate)
/// message parameters.
///
/// Return type: `()`.
pub struct BmSetState {
	pub highlight: bool,
}

impl MsgSend for BmSetState {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BM::SETSTATE.into(),
			wparam: self.highlight as _,
			lparam: 0,
		}
	}
}

/// [`BM_SETSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-setstyle)
/// message parameters.
///
/// Return type: `()`.
pub struct BmSetStyle {
	pub style: co::BS,
	pub redraw: bool,
}

impl MsgSend for BmSetStyle {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BM::SETSTYLE.into(),
			wparam: self.style.raw() as _,
			lparam: self.redraw as _,
		}
	}
}
