use crate::co;
use crate::msg;
use crate::structs::{NMCHAR, NMHDR, NMLISTVIEW};

/// Possible
/// [control notifications](https://docs.microsoft.com/en-us/windows/win32/controls/control-messages).
pub enum Nm<'a> {
	Char(&'a NMCHAR),
	LvnItemChanged(&'a NMLISTVIEW),
	LvnItemChanging(&'a NMLISTVIEW),
}

/// [`WM_NOTIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify)
/// message parameters.
pub struct WmNotify<'a> {
	pub nmhdr: &'a NMHDR,
}

impl<'a> From<WmNotify<'a>> for msg::WmAny {
	fn from(p: WmNotify) -> msg::WmAny {
		msg::WmAny {
			msg: co::WM::NOTIFY,
			wparam: unsafe { p.nmhdr.hwndFrom.as_ptr() } as usize,
			lparam: p.nmhdr as *const NMHDR as isize,
		}
	}
}

impl<'a> From<msg::WmAny> for WmNotify<'a> {
	fn from(p: msg::WmAny) -> WmNotify<'a> {
		WmNotify {
			nmhdr: unsafe { (p.lparam as *const NMHDR).as_ref() }.unwrap(),
		}
	}
}

/// Converts self.nmhdr to another reference.
macro_rules! ref_hdr {
	($me:expr, $ty:ty) => {
		unsafe { ($me.nmhdr as *const NMHDR as *const $ty).as_ref() }.unwrap()
	};
}

impl<'a> WmNotify<'a> {
	pub fn notification<'b>(self) -> Nm<'b> {
		match self.nmhdr.code {
			co::NM::CHAR => Nm::Char(ref_hdr!(self, NMCHAR)),
			co::NM::LVN_ITEMCHANGED => Nm::LvnItemChanged(ref_hdr!(self, NMLISTVIEW)),
			co::NM::LVN_ITEMCHANGING => Nm::LvnItemChanging(ref_hdr!(self, NMLISTVIEW)),
			_ => panic!("Unsupported notification: {}.", self.nmhdr.code),
		}
	}
}