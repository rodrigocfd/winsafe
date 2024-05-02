use std::any::Any;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::prelude::*;
use crate::user::privs::*;

/// A
/// [message-only](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-features#message-only-windows)
/// window, which can handle events.
#[derive(Clone)]
pub struct WindowMessageOnly(Pin<Arc<RawBase>>);

unsafe impl Send for WindowMessageOnly {}

impl AsRef<Base> for WindowMessageOnly {
	fn as_ref(&self) -> &Base {
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
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	#[must_use]
	pub fn new(parent: Option<&WindowMessageOnly>) -> Self {
		let new_self = Self(
			Arc::pin(RawBase::new(parent)),
		);
		new_self.create();
		new_self
	}

	fn create(&self) {
		let hinst = HINSTANCE::GetModuleHandle(None).unwrap();
		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = WString::new();
		RawBase::fill_wndclassex(
			&hinst,
			co::CS::default(), &Icon::None, &Icon::None,
			&Brush::None, &Cursor::None, &mut wcx,
			&mut class_name_buf).unwrap();
		let atom = self.0.register_class(&mut wcx).unwrap();

		let hparent_msg = unsafe { HWND::from_ptr(HWND_MESSAGE as _) };

		self.0.create_window(
			Some(match self.0.base().parent() {
				Some(parent) => parent.hwnd(),
				None => &hparent_msg, // special case: message-only window with no parent
			}),
			atom, None, IdMenu::None,
			POINT::default(), SIZE::default(),
			co::WS_EX::NoValue, co::WS::NoValue,
		).unwrap();
	}
}
