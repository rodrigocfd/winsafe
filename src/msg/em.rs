//! Edit control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-messages),
//! whose constants have [`EM`](crate::co::EM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{HIWORD, LOWORD, MAKEDWORD};
use crate::handles::HLOCAL;
use crate::msg::{MsgSend, WndMsg};
use crate::msg::macros::zero_as_err;
use crate::structs::{POINT, RECT};
use crate::various::WString;

/// [`EN_CANUNDO`](https://docs.microsoft.com/en-us/windows/win32/controls/em-canundo)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct CanUndo {}

impl MsgSend for CanUndo {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::CANUNDO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_CHARFROMPOS`](https://docs.microsoft.com/en-us/windows/win32/controls/em-charfrompos)
/// message parameters.
///
/// Return type: `(u16, u16)`.
///
/// This message is implemented for ordinary edit controls, not for rich edit.
pub struct CharFromPos {
	pub coords: POINT,
}

impl MsgSend for CharFromPos {
	type RetType = (u16, u16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), HIWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::CHARFROMPOS.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.coords.x as _, self.coords.y as _) as _,
		}
	}
}

pub_struct_msg_empty! { EmptyUndoBuffer, co::EM::EMPTYUNDOBUFFER.into(),
	/// [`EM_EMPTYUNDOBUFFER`](https://docs.microsoft.com/en-us/windows/win32/controls/em-emptyundobuffer)
}

/// [`EM_FMTLINES`](https://docs.microsoft.com/en-us/windows/win32/controls/em-fmtlines)
/// message parameters.
///
/// Return type: `bool`.
pub struct FmtLines {
	pub insert_soft_line_breaks: bool,
}

impl MsgSend for FmtLines {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::FMTLINES.into(),
			wparam: self.insert_soft_line_breaks as _,
			lparam: 0,
		}
	}
}

/// [`EM_GETCUEBANNER`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getcuebanner)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetCueBanner<'a> {
	pub buffer: &'a mut WString,
}

impl<'a> MsgSend for GetCueBanner<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 | 1 => Ok(()),
			_ => Err(co::ERROR::BAD_ARGUMENTS),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETCUEBANNER.into(),
			wparam: unsafe { self.buffer.as_mut_ptr() } as _,
			lparam: self.buffer.buffer_size() as _,
		}
	}
}

/// [`EM_GETFIRSTVISIBLELINE`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getfirstvisibleline)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetFirstVisibleLine {}

impl MsgSend for GetFirstVisibleLine {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETFIRSTVISIBLELINE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETHANDLE`](https://docs.microsoft.com/en-us/windows/win32/controls/em-gethandle)
/// message, which has no parameters.
///
/// Return type: `HLOCAL`.
pub struct GetHandle {}

impl MsgSend for GetHandle {
	type RetType = HLOCAL;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		HLOCAL(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETHANDLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETIMESTATUS`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getimestatus)
/// message, which has no parameters.
///
/// Return type: `co::EIMES`.
pub struct GetImeStatus {}

impl MsgSend for GetImeStatus {
	type RetType = co::EIMES;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::EIMES(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETIMESTATUS.into(),
			wparam: 0x0001, // EMSIS_COMPOSITIONSTRING
			lparam: 0,
		}
	}
}

/// [`EM_GETLIMITTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getlimittext)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetLimitText {}

impl MsgSend for GetLimitText {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETLIMITTEXT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETLINE`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getline)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetLine<'a> {
	pub index: u16,
	pub text: &'a mut WString,
}

impl<'a> MsgSend for GetLine<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|count| count as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		self.text.fill_with_zero();
		let buf_len = self.text.len() - 1; // leave room for terminating null
		self.text.as_mut_slice()
			.iter_mut()
			.next()
			.map(|wchar| *wchar = buf_len as _); // leave room for terminating null

		WndMsg {
			msg_id: co::EM::GETLINE.into(),
			wparam: self.index as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`EM_GETLINECOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getlinecount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetLineCount {}

impl MsgSend for GetLineCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETLINECOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getrect)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for GetRect<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`EM_GETSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getsel)
/// message parameters.
///
/// Return type: `()`.
pub struct GetSel<'a, 'b> {
	pub first_index: Option<&'a mut u32>,
	pub past_last_index: Option<&'b mut u32>,
}

impl<'a, 'b> MsgSend for GetSel<'a, 'b> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETSEL.into(),
			wparam: self.first_index.as_mut().map_or(0, |r| r as *mut _ as _),
			lparam: self.past_last_index.as_mut().map_or(0, |r| r as *mut _ as _),
		}
	}
}

/// [`EM_GETTHUMB`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getthumb)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetThumb {}

impl MsgSend for GetThumb {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETTHUMB.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_REPLACESEL`](https://docs.microsoft.com/en-us/windows/win32/controls/em-replacesel)
/// message parameters.
///
/// Return type: `()`.
pub struct ReplaceSel {
	pub can_be_undone: bool,
	pub replacement_text: WString,
}

impl MsgSend for ReplaceSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::REPLACESEL.into(),
			wparam: self.can_be_undone as _,
			lparam: unsafe { self.replacement_text.as_ptr() } as _,
		}
	}
}

/// [`EM_SETLIMITTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setlimittext)
/// message parameters.
///
/// Return type: `()`.
pub struct SetLimitText {
	pub max_chars: u32,
}

impl MsgSend for SetLimitText {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETLIMITTEXT.into(),
			wparam: self.max_chars as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setsel)
/// message parameters.
///
/// Return type: `()`.
pub struct SetSel {
	pub start: Option<u32>,
	pub end: Option<u32>,
}

impl MsgSend for SetSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETSEL.into(),
			wparam: self.start.map_or(-1, |n| n as i32) as _,
			lparam: self.end.map_or(-1, |n| n as i32) as _,
		}
	}
}

/// [`EM_UNDO`](https://docs.microsoft.com/en-us/windows/win32/controls/em-undo)
/// message, which has no parameters.
///
/// Return type: `WinResult<()>`.
pub struct Undo {}

impl MsgSend for Undo {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::UNDO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}
