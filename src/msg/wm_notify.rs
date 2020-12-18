use crate::co;
use crate::msg;
use crate::structs::{NMCHAR, NMHDR, NMITEMACTIVATE, NMLISTVIEW, NMLVDISPINFO,
	NMLVEMPTYMARKUP, NMLVFINDITEM, NMLVGETINFOTIP, NMLVSCROLL};

/// Possible
/// [control notifications](https://docs.microsoft.com/en-us/windows/win32/controls/control-messages).
pub enum Nm<'a> {
	Char(&'a NMCHAR),

	LvnBeginDrag(&'a NMLISTVIEW),
	LvnBeginLabelEdit(&'a NMLVDISPINFO),
	LvnBeginRDrag(&'a NMLISTVIEW),
	LvnBeginScroll(&'a NMLVSCROLL),
	LvnColumnClick(&'a NMLISTVIEW),
	LvnColumnDropDown(&'a NMLISTVIEW),
	LvnColumnOverflowClick(&'a NMLISTVIEW),
	LvnDeleteAllItems(&'a NMLISTVIEW),
	LvnDeleteItem(&'a NMLISTVIEW),
	LvnEndLabelEdit(&'a NMLVDISPINFO),
	LvnEndScroll(&'a NMLVSCROLL),
	LvnGetDispInfo(&'a NMLVDISPINFO),
	LvnGetEmptyMarkup(&'a NMLVEMPTYMARKUP),
	LvnGetInfoTip(&'a NMLVGETINFOTIP),
	LvnHotTrack(&'a NMLISTVIEW),
	LvnIncrementalSearch(&'a NMLVFINDITEM),
	LvnInsertItem(&'a NMLISTVIEW),
	LvnItemActivate(&'a NMITEMACTIVATE),
	LvnItemChanged(&'a NMLISTVIEW),
	LvnItemChanging(&'a NMLISTVIEW),
}

/// [`WM_NOTIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify)
/// message parameters.
#[derive(Copy, Clone)]
pub struct WmNotify<'a> {
	pub nmhdr: &'a NMHDR,
}

impl<'a> From<WmNotify<'a>> for msg::WmAny {
	fn from(p: WmNotify) -> Self {
		Self {
			msg: co::WM::NOTIFY,
			wparam: unsafe { p.nmhdr.hwndFrom.as_ptr() } as usize,
			lparam: p.nmhdr as *const NMHDR as isize,
		}
	}
}

impl<'a> From<msg::WmAny> for WmNotify<'a> {
	fn from(p: msg::WmAny) -> Self {
		Self {
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
	/// Returns a [`Nm`](crate::msg::Nm) enum, which can be matched to identify
	/// the exact notification type.
	pub fn notification<'b>(self) -> Nm<'b> {
		match self.nmhdr.code {
			co::NM::CHAR => Nm::Char(ref_hdr!(self, NMCHAR)),

			co::NM::LVN_BEGINDRAG => Nm::LvnBeginDrag(ref_hdr!(self, NMLISTVIEW)),
			co::NM::LVN_BEGINLABELEDIT => Nm::LvnBeginLabelEdit(ref_hdr!(self, NMLVDISPINFO)),
			co::NM::LVN_BEGINRDRAG => Nm::LvnBeginRDrag(ref_hdr!(self, NMLISTVIEW)),
			co::NM::LVN_BEGINSCROLL => Nm::LvnBeginScroll(ref_hdr!(self, NMLVSCROLL)),
			co::NM::LVN_COLUMNCLICK => Nm::LvnColumnClick(ref_hdr!(self, NMLISTVIEW)),
			co::NM::LVN_COLUMNDROPDOWN => Nm::LvnColumnDropDown(ref_hdr!(self, NMLISTVIEW)),
			co::NM::LVN_COLUMNOVERFLOWCLICK => Nm::LvnColumnOverflowClick(ref_hdr!(self, NMLISTVIEW)),
			co::NM::LVN_DELETEALLITEMS => Nm::LvnDeleteAllItems(ref_hdr!(self, NMLISTVIEW)),
			co::NM::LVN_DELETEITEM => Nm::LvnDeleteItem(ref_hdr!(self, NMLISTVIEW)),
			co::NM::LVN_ENDLABELEDIT => Nm::LvnEndLabelEdit(ref_hdr!(self, NMLVDISPINFO)),
			co::NM::LVN_ENDSCROLL => Nm::LvnEndScroll(ref_hdr!(self, NMLVSCROLL)),
			co::NM::LVN_GETDISPINFO => Nm::LvnBeginLabelEdit(ref_hdr!(self, NMLVDISPINFO)),
			co::NM::LVN_GETEMPTYMARKUP => Nm::LvnGetEmptyMarkup(ref_hdr!(self, NMLVEMPTYMARKUP)),
			co::NM::LVN_GETINFOTIP => Nm::LvnGetInfoTip(ref_hdr!(self, NMLVGETINFOTIP)),
			co::NM::LVN_HOTTRACK => Nm::LvnHotTrack(ref_hdr!(self, NMLISTVIEW)),
			co::NM::LVN_INCREMENTALSEARCH => Nm::LvnIncrementalSearch(ref_hdr!(self, NMLVFINDITEM)),
			co::NM::LVN_INSERTITEM => Nm::LvnInsertItem(ref_hdr!(self, NMLISTVIEW)),
			co::NM::LVN_ITEMACTIVATE => Nm::LvnItemActivate(ref_hdr!(self, NMITEMACTIVATE)),
			co::NM::LVN_ITEMCHANGED => Nm::LvnItemChanged(ref_hdr!(self, NMLISTVIEW)),
			co::NM::LVN_ITEMCHANGING => Nm::LvnItemChanging(ref_hdr!(self, NMLISTVIEW)),
			_ => panic!("Unsupported notification: {}.", self.nmhdr.code),
		}
	}
}