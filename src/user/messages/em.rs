use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::macros::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`EN_CANUNDO`](https://learn.microsoft.com/en-us/windows/win32/controls/em-canundo)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct EmCanUndo {}

impl MsgSend for EmCanUndo {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::CANUNDO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_CHARFROMPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-charfrompos)
/// message parameters.
///
/// Return type: `(u16, u16)`.
///
/// This message is implemented for ordinary edit controls, not for rich edit.
pub struct EmCharFromPos {
	pub coords: POINT,
}

impl MsgSend for EmCharFromPos {
	type RetType = (u16, u16);

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), HIWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::CHARFROMPOS.into(),
			wparam: 0,
			lparam: u32::from(self.coords) as _,
		}
	}
}

pub_struct_msg_empty! { EmEmptyUndoBuffer: co::EM::EMPTYUNDOBUFFER.into();
	/// [`EM_EMPTYUNDOBUFFER`](https://learn.microsoft.com/en-us/windows/win32/controls/em-emptyundobuffer)
}

/// [`EM_FMTLINES`](https://learn.microsoft.com/en-us/windows/win32/controls/em-fmtlines)
/// message parameters.
///
/// Return type: `bool`.
pub struct EmFmtLines {
	pub insert_soft_line_breaks: bool,
}

impl MsgSend for EmFmtLines {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::FMTLINES.into(),
			wparam: self.insert_soft_line_breaks as _,
			lparam: 0,
		}
	}
}

/// [`EM_GETFIRSTVISIBLELINE`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getfirstvisibleline)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct EmGetFirstVisibleLine {}

impl MsgSend for EmGetFirstVisibleLine {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETFIRSTVISIBLELINE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETHANDLE`](https://learn.microsoft.com/en-us/windows/win32/controls/em-gethandle)
/// message, which has no parameters.
///
/// Return type: `SysResult<HLOCAL>`.
pub struct EmGetHandle {}

impl MsgSend for EmGetHandle {
	type RetType = SysResult<HLOCAL>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|v| unsafe { HLOCAL::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETHANDLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETIMESTATUS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getimestatus)
/// message, which has no parameters.
///
/// Return type: `co::EIMES`.
pub struct EmGetImeStatus {}

impl MsgSend for EmGetImeStatus {
	type RetType = co::EIMES;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::EIMES::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETIMESTATUS.into(),
			wparam: 0x0001, // EMSIS_COMPOSITIONSTRING
			lparam: 0,
		}
	}
}

/// [`EM_GETLIMITTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getlimittext)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct EmGetLimitText {}

impl MsgSend for EmGetLimitText {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETLIMITTEXT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETLINE`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getline)
/// message parameters.
///
/// The message will retrieve at most `buffer.len() - 1` characters for the
/// line, because there must be room for a terminating null.
///
/// Returns the number of chars copied to `buffer`, not counting the terminating
/// null, or `None` if no chars were copied. There is no documented way to
/// differentiate between an error and an empty line.
///
/// Return type: `Option<u32>`.
pub struct EmGetLine<'a> {
	pub index: u16,
	pub buffer: &'a mut WString,
}

impl<'a> MsgSend for EmGetLine<'a> {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|count| count as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		self.buffer.fill_with_zero();
		let buf_len = self.buffer.buf_len() - 1; // leave room for terminating null
		self.buffer
			.as_mut_slice()
			.iter_mut()
			.next()
			.map(|wchar| *wchar = buf_len as _); // leave room for terminating null

		Wm {
			msg_id: co::EM::GETLINE.into(),
			wparam: self.index as _,
			lparam: unsafe { self.buffer.as_mut_ptr() } as _,
		}
	}
}

/// [`EM_GETLINECOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getlinecount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct EmGetLineCount {}

impl MsgSend for EmGetLineCount {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETLINECOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETMARGINS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getmargins)
/// message, which has no parameters.
///
/// Return type: `SIZE`.
pub struct EmGetMargins {}

impl MsgSend for EmGetMargins {
	type RetType = SIZE;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		SIZE::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETMARGINS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETMODIFY`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getmodify)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct EmGetModify {}

impl MsgSend for EmGetModify {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETMODIFY.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETPASSWORDCHAR`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getpasswordchar)
/// message, which has no parameters.
///
/// Return type: `Option<char>`.
pub struct EmGetPasswordChar {}

impl MsgSend for EmGetPasswordChar {
	type RetType = Option<char>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|c| unsafe { std::char::from_u32_unchecked(c as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETPASSWORDCHAR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct EmGetRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for EmGetRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`EM_GETSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getsel)
/// message parameters.
///
/// Return type: `()`.
pub struct EmGetSel<'a, 'b> {
	pub first_index: Option<&'a mut u32>,
	pub past_last_index: Option<&'b mut u32>,
}

impl<'a, 'b> MsgSend for EmGetSel<'a, 'b> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETSEL.into(),
			wparam: self.first_index.as_mut().map_or(0, |r| r as *mut _ as _),
			lparam: self
				.past_last_index
				.as_mut()
				.map_or(0, |r| r as *mut _ as _),
		}
	}
}

/// [`EM_GETTHUMB`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getthumb)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct EmGetThumb {}

impl MsgSend for EmGetThumb {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETTHUMB.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETWORDBREAKPROC`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getwordbreakproc)
/// message, which has no parameters.
///
/// Return type: `Option<EDITWORDBREAKPROC>`.
pub struct EmGetWordBreakProc {}

impl MsgSend for EmGetWordBreakProc {
	type RetType = Option<EDITWORDBREAKPROC>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { std::mem::transmute(p) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::GETTHUMB.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_LIMITTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-limittext)
/// message parameters.
///
/// Return type: `()`.
pub struct EmLimitText {
	pub max: Option<u32>,
}

impl MsgSend for EmLimitText {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::LIMITTEXT.into(),
			wparam: self.max.unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINEFROMCHAR`](https://learn.microsoft.com/en-us/windows/win32/controls/em-linefromchar)
/// message parameters.
///
/// Return type: `u32`.
pub struct EmLineFromChar {
	pub char_index: Option<u32>,
}

impl MsgSend for EmLineFromChar {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::LINEFROMCHAR.into(),
			wparam: self.char_index.unwrap_or(-1i32 as _) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINEINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/em-lineindex)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct EmLineIndex {
	pub line_index: Option<u32>,
}

impl MsgSend for EmLineIndex {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::LINEINDEX.into(),
			wparam: self.line_index.unwrap_or(-1i32 as _) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINELENGTH`](https://learn.microsoft.com/en-us/windows/win32/controls/em-linelength)
/// message parameters.
///
/// Return type: `u32`.
pub struct EmLineLength {
	pub char_index: Option<u32>,
}

impl MsgSend for EmLineLength {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::LINELENGTH.into(),
			wparam: self.char_index.unwrap_or(-1i32 as _) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINESCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/em-linescroll)
/// message parameters.
///
/// Return type: `bool`.
pub struct EmLineScroll {
	pub num_chars: u32,
	pub num_lines: u32,
}

impl MsgSend for EmLineScroll {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::LINESCROLL.into(),
			wparam: self.num_chars as _,
			lparam: self.num_lines as _,
		}
	}
}

/// [`EM_POSFROMCHAR`](https://learn.microsoft.com/en-us/windows/win32/controls/em-posfromchar)
/// message parameters.
///
/// Return type: `POINT`.
///
/// This message is implemented for ordinary edit controls, not for rich edit.
pub struct EmPosFromChar {
	pub char_index: u32,
}

impl MsgSend for EmPosFromChar {
	type RetType = POINT;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		POINT::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::POSFROMCHAR.into(),
			wparam: self.char_index as _,
			lparam: 0,
		}
	}
}

/// [`EM_REPLACESEL`](https://learn.microsoft.com/en-us/windows/win32/controls/em-replacesel)
/// message parameters.
///
/// Return type: `()`.
pub struct EmReplaceSel {
	pub can_be_undone: bool,
	pub replacement_text: WString,
}

impl MsgSend for EmReplaceSel {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::REPLACESEL.into(),
			wparam: self.can_be_undone as _,
			lparam: self.replacement_text.as_ptr() as _,
		}
	}
}

/// [`EM_SCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/em-scroll)
/// message parameters.
///
/// Return type: `SysResult<u16>`.
pub struct EmScroll {
	pub action: co::SB_EM,
}

impl MsgSend for EmScroll {
	type RetType = SysResult<u16>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|num_lines| num_lines as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SCROLL.into(),
			wparam: self.action.raw() as _,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { EmScrollCaret: co::EM::SCROLLCARET.into();
	/// [`EM_SCROLLCARET`](https://learn.microsoft.com/en-us/windows/win32/controls/em-scrollcaret)
}

/// [`EM_SETHANDLE`](https://learn.microsoft.com/en-us/windows/win32/controls/em-sethandle)
/// message parameters.
///
/// Return type: `()`.
pub struct EmSetHandle<'a> {
	pub handle: &'a HLOCAL,
}

impl<'a> MsgSend for EmSetHandle<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETHANDLE.into(),
			wparam: self.handle.ptr() as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETIMESTATUS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setimestatus)
/// message parameters.
///
/// Return type: `co::EIMES`.
pub struct EmSetImeStatus {
	pub status: co::EIMES,
}

impl MsgSend for EmSetImeStatus {
	type RetType = co::EIMES;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::EIMES::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETIMESTATUS.into(),
			wparam: 0x0001, // EMSIS_COMPOSITIONSTRING
			lparam: self.status.raw() as _,
		}
	}
}

/// [`EM_SETLIMITTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setlimittext)
/// message parameters.
///
/// Return type: `()`.
pub struct EmSetLimitText {
	pub max_chars: Option<u32>,
}

impl MsgSend for EmSetLimitText {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETLIMITTEXT.into(),
			wparam: self.max_chars.unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETMARGINS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setmargins)
/// message parameters.
///
/// Return type: `()`.
pub struct EmSetMargins {
	pub margins: co::EC,
	pub size: SIZE,
}

impl MsgSend for EmSetMargins {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETMARGINS.into(),
			wparam: self.margins.raw() as _,
			lparam: u32::from(self.size) as _,
		}
	}
}

/// [`EM_SETMODIFY`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setmodify)
/// message parameters.
///
/// Return type: `()`.
pub struct EmSetModify {
	pub flag: bool,
}

impl MsgSend for EmSetModify {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETMODIFY.into(),
			wparam: self.flag as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETPASSWORDCHAR`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setpasswordchar)
/// message parameters.
///
/// Return type: `()`.
pub struct EmSetPasswordChar {
	pub character: Option<char>,
}

impl MsgSend for EmSetPasswordChar {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETPASSWORDCHAR.into(),
			wparam: self.character.map(|ch| ch as u32).unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETREADONLY`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setreadonly)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct EmSetReadOnly {
	pub read_only: bool,
}

impl MsgSend for EmSetReadOnly {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETREADONLY.into(),
			wparam: self.read_only as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setrect)
/// message parameters.
///
/// Return type: `()`.
pub struct EmSetRect<'a> {
	pub is_absolute_coords: bool,
	pub rect: Option<&'a RECT>,
}

impl<'a> MsgSend for EmSetRect<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETRECT.into(),
			wparam: self.is_absolute_coords as _,
			lparam: self.rect.map_or(0, |rect| rect as *const _ as _),
		}
	}
}

/// [`EM_SETRECTNP`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setrectnp)
/// message parameters.
///
/// Return type: `()`.
pub struct EmSetRectNp<'a> {
	pub is_absolute_coords: bool,
	pub rect: Option<&'a RECT>,
}

impl<'a> MsgSend for EmSetRectNp<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETRECTNP.into(),
			wparam: self.is_absolute_coords as _,
			lparam: self.rect.map_or(0, |rect| rect as *const _ as _),
		}
	}
}

/// [`EM_SETSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setsel)
/// message parameters.
///
/// Return type: `()`.
pub struct EmSetSel {
	pub start: i32,
	pub end: i32,
}

impl MsgSend for EmSetSel {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETSEL.into(),
			wparam: self.start as _,
			lparam: self.end as _,
		}
	}
}

/// [`EM_SETTABSTOPS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-settabstops)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct EmSetTabStops<'a> {
	pub tab_stops: Option<&'a [i32]>,
}

impl<'a> MsgSend for EmSetTabStops<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETTABSTOPS.into(),
			wparam: self.tab_stops.map_or(0, |ts| ts.len() as _),
			lparam: self.tab_stops.map_or(0, |ts| vec_ptr(ts) as _),
		}
	}
}

/// [`EM_SETWORDBREAKPROC`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setwordbreakproc)
/// message parameters.
///
/// Return type: `()`.
pub struct EmSetWordBreakProc {
	pub proc: Option<EDITWORDBREAKPROC>,
}

impl MsgSend for EmSetWordBreakProc {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::SETWORDBREAKPROC.into(),
			wparam: 0,
			lparam: self.proc.map_or(0, |proc| proc as _),
		}
	}
}

/// [`EM_UNDO`](https://learn.microsoft.com/en-us/windows/win32/controls/em-undo)
/// message, which has no parameters.
///
/// Return type: `SysResult<()>`.
pub struct EmUndo {}

impl MsgSend for EmUndo {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::EM::UNDO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}
