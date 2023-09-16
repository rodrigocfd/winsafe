use std::any::Any;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::prelude::*;

/// A
/// [message-only](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-features#message-only-windows)
/// window.
#[derive(Clone)]
pub struct WindowMessageOnly(Pin<Arc<RawBase>>);

unsafe impl Send for WindowMessageOnly {}

impl GuiWindow for WindowMessageOnly {
	fn hwnd(&self) -> &HWND {
		self.0.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiParent for WindowMessageOnly {
	fn on(&self) -> &WindowEventsAll {
		self.0.on()
	}

	unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		self.0.as_base()
	}
}

impl WindowMessageOnly {
	/// Instantiates a new `WindowMessageOnly` object, to be created internally
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	#[must_use]
	pub fn new() -> Self {
		let new_self = Self(
			Arc::pin(RawBase::new(None)),
		);
		new_self.create();
		new_self
	}

	fn create(&self) {
		let hinst = HINSTANCE::GetModuleHandle(None).unwrap();
		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = WString::default();
		RawBase::fill_wndclassex(
			&hinst,
			co::CS::default(), &Icon::None, &Icon::None,
			&Brush::None, &Cursor::None, &mut wcx,
			&mut class_name_buf).unwrap();
		let atom = self.0.register_class(&mut wcx).unwrap();

		self.0.create_window(
			atom, None, IdMenu::None,
			POINT::default(), SIZE::default(),
			co::WS_EX::NoValue, co::WS::NoValue,
		).unwrap();
	}
}
