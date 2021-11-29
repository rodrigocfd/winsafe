use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::events::{EditEvents, EventsView, WindowEvents};
use crate::gui::native_controls::base_native_control::{
	BaseNativeControl,
	OptsId,
};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi_or_dtu, ui_font};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{
	AsAny,
	Child,
	FocusControl,
	NativeControl,
	NativeControlEvents,
	Parent,
	TextControl,
	Window,
};
use crate::handles::{Handle, HWND};
use crate::msg::{em, wm};
use crate::structs::{EDITBALLOONTIP, POINT, SIZE};
use crate::various::WString;

/// Native
/// [edit](https://docs.microsoft.com/en-us/windows/win32/controls/about-edit-controls)
/// control.
#[derive(Clone)]
pub struct Edit(Arc<Obj>);

struct Obj { // actual fields of Edit
	base: BaseNativeControl,
	opts_id: OptsId<EditOpts>,
	events: EditEvents,
}

unsafe impl Send for Edit {}

impl AsAny for Edit {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Window for Edit {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl Child for Edit {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl NativeControl for Edit {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl NativeControlEvents<EditEvents> for Edit {
	fn on(&self) -> &EditEvents {
		if !self.0.base.hwnd().is_null() {
			panic!("Cannot add events after the control creation.");
		} else if !self.0.base.parent_base().hwnd().is_null() {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl FocusControl for Edit {}
impl TextControl for Edit {}

impl Edit {
	/// Instantiates a new `Edit` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &impl Parent, opts: EditOpts) -> Edit {
		let opts = EditOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Wnd(opts),
					events: EditEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm(parent.as_base().wmcreate_or_wminitdialog(), {
			let self2 = new_self.clone();
			move |_| self2.create(horz, vert)
				.map_err(|e| e.into())
				.map(|_| 0)
		});
		new_self
	}

	/// Instantiates a new `Edit` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(
		parent: &impl Parent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert)) -> Edit
	{
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Dlg(ctrl_id),
					events: EditEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm_init_dialog({
			let self2 = new_self.clone();
			move |_| self2.create(resize_behavior.0, resize_behavior.1)
				.map_err(|e| e.into())
				.map(|_| true)
		});
		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				let mut sz = SIZE::new(opts.width as _, opts.height as _);
				multiply_dpi_or_dtu(
					self.0.base.parent_base(), Some(&mut pos), Some(&mut sz))?;

				let our_hwnd = self.0.base.create_window(
					"EDIT", Some(&opts.text), pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.edit_style.into(),
				)?;

				our_hwnd.SendMessage(wm::SetFont { hfont: ui_font(), redraw: true });
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?,
		}

		self.0.base.parent_base().add_to_resizer(self.hwnd(), horz, vert)
	}

	/// Hides any balloon tip by sending an
	/// [`em::HideBalloonTip`](crate::msg::em::HideBalloonTip) message.
	pub fn hide_balloon_tip(&self) {
		self.hwnd().SendMessage(em::HideBalloonTip {}).ok(); // ignore error
	}

	/// Returns an iterator over the lines in the Edit.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let my_edit: gui::Edit; // initialized somewhere
	///
	/// for line in my_edit.iter_lines()? {
	///     println!("{}", line);
	/// }
	/// ```
	pub fn iter_lines<'a>(&'a self) -> WinResult<impl Iterator<Item = String> + 'a> {
		LinesIter::new(self.hwnd())
	}

	/// Limits the number of characters that can be type by sending an
	/// /// [`em::SetLimitText`](crate::msg::em::SetLimitText) message.
	pub fn limit_text(&self, max_chars: Option<u32>) {
		self.hwnd().SendMessage(em::SetLimitText { max_chars });
	}

	/// Sets the selection range of the text by sending an
	/// [`em::SetSel`](crate::msg::em::SetSel) message.
	///
	/// # Examples
	///
	/// Selecting all text in the control:
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let my_edit: gui::Edit; // initialized somewhere
	///
	/// my_edit.set_selection(Some(0), None);
	/// ```
	///
	/// Clearing the selection:
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_edit: gui::Edit; // initialized somewhere
	///
	/// my_edit.set_selection(None, None);
	/// ```
	pub fn set_selection(&self, start: Option<u32>, end: Option<u32>) {
		self.hwnd().SendMessage(em::SetSel { start, end });
	}

	/// Displays a balloon tip by sending an
	/// [`em::ShowBalloonTip`](crate::msg::em::ShowBalloonTip) message.
	pub fn show_ballon_tip(&self,
		title: &str, text: &str, icon: co::TTI) -> WinResult<()>
	{
		let mut title16 = WString::from_str(title);
		let mut text16 = WString::from_str(text);

		let mut info = EDITBALLOONTIP::default();
		info.set_pszTitle(Some(&mut title16));
		info.set_pszText(Some(&mut text16));
		info.ttiIcon = icon;

		self.hwnd().SendMessage(em::ShowBalloonTip { info: &info })
	}
}

//------------------------------------------------------------------------------

struct LinesIter<'a> {
	hwnd: HWND,
	buf: WString,
	total: usize,
	current: usize,
	owner_: PhantomData<&'a ()>,
}

impl<'a> Iterator for LinesIter<'a> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.total {
			return None;
		}

		let line = match self.hwnd.SendMessage(em::GetLine {
			index: self.current as _,
			buffer: &mut self.buf,
		}) {
			Some(_) => self.buf.to_string(),
			None => "".to_owned(),
		};

		self.current += 1;
		Some(line)
	}
}

impl<'a> LinesIter<'a> {
	fn new(hwnd: HWND) -> WinResult<Self> {
		Ok(Self {
			hwnd,
			buf: WString::new_alloc_buffer(hwnd.GetWindowTextLength()? as usize + 1), // so we alloc just once
			total: hwnd.SendMessage(em::GetLineCount {}) as _,
			current: 0,
			owner_: PhantomData,
		})
	}
}

//------------------------------------------------------------------------------

/// Options to create an [`Edit`](crate::gui::Edit) programmatically with
/// [`Edit::new`](crate::gui::Edit::new).
pub struct EditOpts {
	/// Text of the control to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Control position within parent client area, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control width, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the value is in Dialog Template Units;
	/// otherwise in pixels, which will be multiplied to match current system
	/// DPI.
	///
	/// Defaults to 100.
	pub width: u32,
	/// Control height, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the value is in Dialog Template Units;
	/// otherwise in pixels, which will be multiplied to match current system
	/// DPI.
	///
	/// Defaults to 21.
	///
	/// **Note:** You should change the default height only in a multi-line edit.
	pub height: u32,
	/// Edit styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `ES::AUTOHSCROLL | ES::NOHIDESEL`.
	///
	/// Suggestions:
	/// * add `ES::PASSWORD` for a password input;
	/// * add `ES::NUMBER` to accept only numbers;
	/// * replace with `ES::MULTILINE | ES::WANTRETURN | ES::AUTOVSCROLL | ES::NOHIDESEL` for a multi-line edit.
	pub edit_style: co::ES,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::CLIENTEDGE`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	///
	/// **Note:** You should use `Vert::Resize` only in a multi-line edit.
	pub vert_resize: Vert,
}

impl Default for EditOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: POINT::new(0, 0),
			width: 100,
			height: 21,
			edit_style: co::ES::AUTOHSCROLL | co::ES::NOHIDESEL,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
		}
	}
}

impl EditOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
