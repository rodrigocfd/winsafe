use crate::co;
use crate::decl::*;

/// Variable parameter for:
///
/// * [`HWND::HtmlHelp`](crate::HWND::HtmlHelp)
pub enum HhCmd<'a> {
	/// [`HH_CLOSE_ALL`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/htmlhelp/hh-close-all-command)
	/// command.
	CloseAll,
	/// [`HH_DISPLAY_INDEX`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/htmlhelp/hh-display-index-command)
	/// command.
	///
	/// Receives the keyword to select in the index file.
	DisplayIndex(&'a str),
	/// [`HH_DISPLAY_TOC`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/htmlhelp/hh-display-toc-command)
	/// command.
	DisplayToc,
	/// [`HH_TP_HELP_CONTEXTMENU`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/htmlhelp/hh-tp-help-contextmenu-command)
	/// command.
	///
	/// Receives pairs composed of:
	/// - dialog box control ID;
	/// - help topic ID.
	TpHelpContextMenu(&'a [(u16, u16)]),
	/// [`HH_TP_HELP_WM_HELP`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/htmlhelp/hh-tp-help-wm-help-command)
	/// command.
	///
	/// Receives pairs composed of:
	/// - dialog box control ID;
	/// - help topic ID.
	TpHelpWmHelp(&'a [(u16, u16)]),
}

impl<'a> HhCmd<'a> {
	/// Returns the [`co::HH`](crate::co::HH) constant and serialized data. So
	/// far, only a limited subset has been implemented. The ones involving
	/// pointers are still up to debate.
	#[must_use]
	pub(in crate::htmlhelp) fn as_data(&self) -> (co::HH, HhCmdData) {
		use HhCmd::*;
		match self {
			CloseAll => (co::HH::CLOSE_ALL, HhCmdData::None),
			DisplayIndex(s) => (co::HH::DISPLAY_INDEX, Self::gen_str(s)),
			DisplayToc => (co::HH::DISPLAY_TOC, HhCmdData::None),
			TpHelpContextMenu(arr) => (co::HH::TP_HELP_CONTEXTMENU, Self::gen_vec(arr)),
			TpHelpWmHelp(arr) => (co::HH::TP_HELP_WM_HELP, Self::gen_vec(arr)),
		}
	}

	#[must_use]
	fn gen_str(s: &str) -> HhCmdData {
		HhCmdData::Str(WString::from_str(s))
	}

	#[must_use]
	fn gen_vec(array: &[(u16, u16)]) -> HhCmdData {
		let mut buf = Vec::<u32>::with_capacity(array.len() * 2 + 1);
		for (ctrl_id, topic_id) in array.iter() {
			buf.push(*ctrl_id as _);
			buf.push(*topic_id as _);
		}
		buf.push(0);
		HhCmdData::Ids(buf)
	}
}

/// Serialized data of [`HhCmd`].
pub(in crate::htmlhelp) enum HhCmdData {
	None,
	Str(WString),
	Ids(Vec<u32>),
}

impl HhCmdData {
	#[must_use]
	pub(in crate::htmlhelp) fn serialize(&self) -> usize {
		match self {
			HhCmdData::None => 0,
			HhCmdData::Str(s) => s.as_ptr() as _,
			HhCmdData::Ids(arr) => arr.as_ptr() as _,
		}
	}
}
