use crate::co;
use crate::msg;
use crate::structs as s;

/// Possible
/// [control notifications](https://docs.microsoft.com/en-us/windows/win32/controls/control-messages).
pub enum Nm<'a, 'b> {
	Char(&'a s::NMCHAR),

	LvnBeginDrag(&'a s::NMLISTVIEW),
	LvnBeginLabelEdit(&'a s::NMLVDISPINFO<'b>),
	LvnBeginRDrag(&'a s::NMLISTVIEW),
	LvnBeginScroll(&'a s::NMLVSCROLL),
	LvnColumnClick(&'a s::NMLISTVIEW),
	LvnColumnDropDown(&'a s::NMLISTVIEW),
	LvnColumnOverflowClick(&'a s::NMLISTVIEW),
	LvnDeleteAllItems(&'a s::NMLISTVIEW),
	LvnDeleteItem(&'a s::NMLISTVIEW),
	LvnEndLabelEdit(&'a s::NMLVDISPINFO<'b>),
	LvnEndScroll(&'a s::NMLVSCROLL),
	LvnGetDispInfo(&'a s::NMLVDISPINFO<'b>),
	LvnGetEmptyMarkup(&'a s::NMLVEMPTYMARKUP),
	LvnGetInfoTip(&'a s::NMLVGETINFOTIP<'b>),
	LvnHotTrack(&'a s::NMLISTVIEW),
	LvnIncrementalSearch(&'a s::NMLVFINDITEM<'b>),
	LvnInsertItem(&'a s::NMLISTVIEW),
	LvnItemActivate(&'a s::NMITEMACTIVATE),
	LvnItemChanged(&'a s::NMLISTVIEW),
	LvnItemChanging(&'a s::NMLISTVIEW),
}

//------------------------------------------------------------------------------

/// [`WM_NOTIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify)
/// message parameters.
#[derive(Copy, Clone)]
pub struct WmNotify<'a> {
	pub nmhdr: &'a s::NMHDR,
}

impl<'a> From<WmNotify<'a>> for msg::WmAny {
	fn from(p: WmNotify) -> Self {
		Self {
			msg: co::WM::NOTIFY,
			wparam: unsafe { p.nmhdr.hwndFrom.as_ptr() } as usize,
			lparam: p.nmhdr as *const s::NMHDR as isize,
		}
	}
}

impl<'a> From<msg::WmAny> for WmNotify<'a> {
	fn from(p: msg::WmAny) -> Self {
		Self {
			nmhdr: unsafe { (p.lparam as *const s::NMHDR).as_ref() }.unwrap(),
		}
	}
}

impl<'a> WmNotify<'a> {
	/// Casts the `NMHDR` reference into a derived struct.
	///
	/// You should always prefer the specific notification handlers, which
	/// perform this conversion for you.
	pub unsafe fn cast_nmhdr<T>(&self) -> &T {
		(self.nmhdr as *const s::NMHDR as *const T).as_ref().unwrap()
	}
}

/// Converts self.nmhdr to another reference.
macro_rules! ref_hdr {
	($me:expr, $ty:ty) => {
		unsafe { ($me.nmhdr as *const s::NMHDR as *const $ty).as_ref() }.unwrap()
	};
}

impl<'a> WmNotify<'a> {
	/// Returns a [`Nm`](crate::msg::Nm) enum, which can be matched to identify
	/// the exact notification type.
	pub fn notification<'b, 'c>(self) -> Nm<'b, 'c> {
		match self.nmhdr.code {
			co::NM::CHAR => Nm::Char(ref_hdr!(self, s::NMCHAR)),

			co::NM::LVN_BEGINDRAG => Nm::LvnBeginDrag(ref_hdr!(self, s::NMLISTVIEW)),
			co::NM::LVN_BEGINLABELEDIT => Nm::LvnBeginLabelEdit(ref_hdr!(self, s::NMLVDISPINFO)),
			co::NM::LVN_BEGINRDRAG => Nm::LvnBeginRDrag(ref_hdr!(self, s::NMLISTVIEW)),
			co::NM::LVN_BEGINSCROLL => Nm::LvnBeginScroll(ref_hdr!(self, s::NMLVSCROLL)),
			co::NM::LVN_COLUMNCLICK => Nm::LvnColumnClick(ref_hdr!(self, s::NMLISTVIEW)),
			co::NM::LVN_COLUMNDROPDOWN => Nm::LvnColumnDropDown(ref_hdr!(self, s::NMLISTVIEW)),
			co::NM::LVN_COLUMNOVERFLOWCLICK => Nm::LvnColumnOverflowClick(ref_hdr!(self, s::NMLISTVIEW)),
			co::NM::LVN_DELETEALLITEMS => Nm::LvnDeleteAllItems(ref_hdr!(self, s::NMLISTVIEW)),
			co::NM::LVN_DELETEITEM => Nm::LvnDeleteItem(ref_hdr!(self, s::NMLISTVIEW)),
			co::NM::LVN_ENDLABELEDIT => Nm::LvnEndLabelEdit(ref_hdr!(self, s::NMLVDISPINFO)),
			co::NM::LVN_ENDSCROLL => Nm::LvnEndScroll(ref_hdr!(self, s::NMLVSCROLL)),
			co::NM::LVN_GETDISPINFO => Nm::LvnBeginLabelEdit(ref_hdr!(self, s::NMLVDISPINFO)),
			co::NM::LVN_GETEMPTYMARKUP => Nm::LvnGetEmptyMarkup(ref_hdr!(self, s::NMLVEMPTYMARKUP)),
			co::NM::LVN_GETINFOTIP => Nm::LvnGetInfoTip(ref_hdr!(self, s::NMLVGETINFOTIP)),
			co::NM::LVN_HOTTRACK => Nm::LvnHotTrack(ref_hdr!(self, s::NMLISTVIEW)),
			co::NM::LVN_INCREMENTALSEARCH => Nm::LvnIncrementalSearch(ref_hdr!(self, s::NMLVFINDITEM)),
			co::NM::LVN_INSERTITEM => Nm::LvnInsertItem(ref_hdr!(self, s::NMLISTVIEW)),
			co::NM::LVN_ITEMACTIVATE => Nm::LvnItemActivate(ref_hdr!(self, s::NMITEMACTIVATE)),
			co::NM::LVN_ITEMCHANGED => Nm::LvnItemChanged(ref_hdr!(self, s::NMLISTVIEW)),
			co::NM::LVN_ITEMCHANGING => Nm::LvnItemChanging(ref_hdr!(self, s::NMLISTVIEW)),
			_ => panic!("Unsupported notification: {}.", self.nmhdr.code),
		}
	}
}