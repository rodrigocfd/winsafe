use std::any::Any;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*, *};
use crate::prelude::*;
use crate::user::privs::*;

/// A
/// [message-only](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-features#message-only-windows)
/// window, which can handle events.
#[derive(Clone)]
pub struct WindowMessageOnly(Pin<Arc<RawBase>>);

unsafe impl Send for WindowMessageOnly {}

impl AsRef<BaseWnd> for WindowMessageOnly {
	fn as_ref(&self) -> &BaseWnd {
		self.0.base()
	}
}

impl GuiWindow for WindowMessageOnly {
	fn hwnd(&self) -> &HWND {
		self.0.base().hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiParent for WindowMessageOnly {}

impl WindowMessageOnly {
	/// Instantiates a new `WindowMessageOnly` object, to be created internally
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the creation process fails.
	#[must_use]
	pub fn new(parent: Option<&WindowMessageOnly>) -> Self {
		let new_self = Self(Arc::pin(RawBase::new()));
		new_self.create(parent);
		new_self
	}

	fn create(&self, parent: Option<&WindowMessageOnly>) {
		let hinst = HINSTANCE::GetModuleHandle(None).expect(DONTFAIL);
		let atom = self.0.register_class(
			&hinst,
			"",
			co::CS::default(),
			&Icon::None,
			&Brush::None,
			&Cursor::None,
		);

		let hparent_msg = unsafe { HWND::from_ptr(HWND_MESSAGE as _) };

		self.0.create_window(
			co::WS_EX::NoValue,
			atom,
			None,
			co::WS::NoValue,
			POINT::default(),
			SIZE::default(),
			Some(match parent {
				Some(parent) => parent.hwnd(),
				None => &hparent_msg, // special case: message-only window with no parent
			}),
			IdMenu::None,
			&hinst,
		);
	}

	/// Exposes methods to handle the basic window messages, plus timer and
	/// native control notifications.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before
	/// window creation.
	#[must_use]
	pub fn on(&self) -> &WindowEventsAll {
		self.as_ref().on()
	}
}
