use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::prelude::*;

struct RawModalObj {
	raw_base: RawBase,
	opts: WindowModalOptsObj,
	hchild_prev_focus_parent: UnsafeCell<HWND>,
	_pin: PhantomPinned,
}

/// An ordinary modal window.
///
/// Hierarchy: `BaseWnd` -> `RawBase` -> `RawModal`.
#[derive(Clone)]
pub(in crate::gui) struct RawModal(Pin<Arc<RawModalObj>>);

impl RawModal {
	#[must_use]
	pub(in crate::gui) fn new(opts: WindowModalOpts) -> Self {
		let new_self = Self(Arc::pin(RawModalObj {
			raw_base: RawBase::new(),
			opts: opts.into(),
			hchild_prev_focus_parent: UnsafeCell::new(HWND::NULL),
			_pin: PhantomPinned,
		}));
		new_self.default_message_handlers();
		new_self
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.0.raw_base.base().before_on().wm_set_focus(move |_| {
			self2.0.raw_base.delegate_focus_to_first_child();
			Ok(())
		});

		let self2 = self.clone();
		self.0.raw_base.base().on().wm_close(move || {
			if let Ok(hparent) = self2.0.raw_base.base().hwnd().GetWindow(co::GW::OWNER) {
				hparent.EnableWindow(true); // re-enable parent
				self2
					.0
					.raw_base
					.base()
					.hwnd()
					.DestroyWindow()
					.expect(DONTFAIL); // then destroy modal

				let hchild_prev_focus_parent =
					unsafe { &mut *self2.0.hchild_prev_focus_parent.get() };

				if *hchild_prev_focus_parent != HWND::NULL {
					hchild_prev_focus_parent.SetFocus(); // this focus could be set on WM_DESTROY as well
				}
			}
			Ok(())
		});
	}

	#[must_use]
	pub(in crate::gui) fn raw_base(&self) -> &RawBase {
		&self.0.raw_base
	}

	pub(in crate::gui) fn show_modal(&self, parent: &impl GuiParent) -> AnyResult<()> {
		let hinst = parent.hwnd().hinstance();
		let opts = &self.0.opts;
		let atom = self.0.raw_base.register_class(
			&hinst,
			&opts.class_name,
			opts.class_style,
			&opts.class_icon,
			&opts.class_bg_brush,
			&opts.class_cursor,
		);

		*unsafe { &mut *self.0.hchild_prev_focus_parent.get() } =
			HWND::GetFocus().unwrap_or(HWND::NULL);
		parent.hwnd().EnableWindow(false); // https://devblogs.microsoft.com/oldnewthing/20040227-00/?p=40463

		let mut rc_wnd = RECT {
			left: 0, // client area, will be adjusted to size with title bar and borders
			top: 0,
			right: opts.size.0,
			bottom: opts.size.1,
		};
		rc_wnd = AdjustWindowRectEx(rc_wnd, opts.style, false, opts.ex_style).expect(DONTFAIL);

		let rc_parent = parent.hwnd().GetWindowRect().expect(DONTFAIL); // relative to screen
		let wnd_pos = POINT::with(
			rc_parent.left + (rc_parent.right - rc_parent.left) / 2
				- (rc_wnd.right - rc_wnd.left) / 2, // center on parent
			rc_parent.top + (rc_parent.bottom - rc_parent.top) / 2
				- (rc_wnd.bottom - rc_wnd.top) / 2,
		);

		self.0.raw_base.create_window(
			opts.ex_style,
			atom,
			Some(&opts.title),
			opts.style,
			POINT::with(wnd_pos.x, wnd_pos.y),
			SIZE::with(rc_wnd.right - rc_wnd.left, rc_wnd.bottom - rc_wnd.top),
			Some(parent.hwnd()),
			IdMenu::None,
			&hinst,
		);

		self.0
			.raw_base
			.base()
			.run_modal_loop(opts.process_dlg_msgs) // blocks until window is closed
			.map(|_| ())
	}
}

/// Options to create a [`WindowModal`](crate::gui::WindowModal)
/// programmatically with [`WindowModal::new`](crate::gui::WindowModal::new).
pub struct WindowModalOpts<'a> {
	/// Window class name to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to an auto-generated string.
	pub class_name: &'a str,
	/// Window class styles to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::CS::DBLCLKS`.
	pub class_style: co::CS,
	/// Window main icon to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Icon::None`.
	pub class_icon: Icon,
	/// Window cursor to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Cursor::Idc(co::IDC::ARROW)`.
	pub class_cursor: Cursor,
	/// Window background brush to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Brush::Color(co::COLOR::BTNFACE)`.
	pub class_bg_brush: Brush,

	/// Window title to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub title: &'a str,
	/// Width and height of window client area, in pixels, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	/// Does not include title bar or borders.
	///
	/// Defaults to `gui::dpi(500, 400)`.
	pub size: (i32, i32),
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CAPTION | WS::SYSMENU | WS::CLIPCHILDREN | WS::BORDER | WS::VISIBLE`.
	///
	/// Suggestions:
	/// * `WS::SIZEBOX` to make the window resizable;
	/// * `WS::MAXIMIZEBOX` to have a maximize button.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::DLGMODALFRAME`.
	pub ex_style: co::WS_EX,
	/// In most applications, the window loop calls
	/// [`IsDialogMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isdialogmessagew)
	/// so child control messages will properly work. However, this has the side
	/// effect of inhibiting
	/// [`WM_CHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-char)
	/// messages from being sent to the window procedure. So, applications which
	/// do not have child controls and deal directly with character processing –
	/// like text editors – will never be able to receive `WM_CHAR`.
	///
	/// This flag, when `true`, will enable the normal `IsDialogMessage` call in
	/// the window loop. When `false`, the call will be suppressed.
	///
	/// Defaults to `true`.
	pub process_dlg_msgs: bool,
}

impl<'a> Default for WindowModalOpts<'a> {
	fn default() -> Self {
		Self {
			class_name: "",
			class_style: co::CS::DBLCLKS,
			class_icon: Icon::None,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::BTNFACE),
			title: "",
			size: dpi(500, 400),
			style: co::WS::CAPTION
				| co::WS::SYSMENU
				| co::WS::CLIPCHILDREN
				| co::WS::BORDER
				| co::WS::VISIBLE,
			ex_style: co::WS_EX::LEFT | co::WS_EX::DLGMODALFRAME,
			process_dlg_msgs: true,
		}
	}
}

impl<'a> Into<WindowModalOptsObj> for WindowModalOpts<'a> {
	fn into(self) -> WindowModalOptsObj {
		WindowModalOptsObj {
			class_name: self.class_name.to_owned(),
			class_style: self.class_style,
			class_icon: self.class_icon,
			class_cursor: self.class_cursor,
			class_bg_brush: self.class_bg_brush,
			title: self.title.to_owned(),
			size: self.size,
			style: self.style,
			ex_style: self.ex_style,
			process_dlg_msgs: self.process_dlg_msgs,
		}
	}
}

/// To be stored inside the object.
struct WindowModalOptsObj {
	class_name: String,
	class_style: co::CS,
	class_icon: Icon,
	class_cursor: Cursor,
	class_bg_brush: Brush,
	title: String,
	size: (i32, i32),
	style: co::WS,
	ex_style: co::WS_EX,
	process_dlg_msgs: bool,
}
