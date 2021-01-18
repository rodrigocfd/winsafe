use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::{IdIdcStr, IdMenu};
use crate::gui::events::MsgEvents;
use crate::gui::globals::{multiply_dpi, paint_control_borders};
use crate::gui::immut::Immut;
use crate::gui::parent::Parent;
use crate::gui::window_base::WindowBase;
use crate::handles::{HBRUSH, HCURSOR, HICON, HINSTANCE, HWND};
use crate::structs::{POINT, SIZE, WNDCLASSEX};
use crate::WString;

#[derive(Clone)]
pub struct WindowControl(Arc<Immut<Obj>>);

struct Obj { // actual fields of WindowControl
	base: WindowBase,
	opts: CustomControlOpts,
}

impl Parent for WindowControl {
	fn hwnd_ref(&self) -> &HWND {
		self.0.base.hwnd_ref()
	}

	fn events_ref(&self) -> &MsgEvents {
		self.0.base.events_ref()
	}

	fn add_child_to_be_created(&self,
		func: Box<dyn Fn() -> WinResult<()> + 'static>)
	{
		self.0.base.add_child_to_be_created(func);
	}
}

impl WindowControl {
	pub fn new(parent: &dyn Parent, opts: CustomControlOpts) -> WindowControl {
		let wnd = Self(
			Arc::new(Immut::new(
				Obj {
					base: WindowBase::new(Some(parent)),
					opts,
				},
			)),
		);
		wnd.default_message_handlers();
		wnd.add_creation_to_parent(parent);
		wnd
	}

	fn add_creation_to_parent(&self, parent: &dyn Parent) {
		let me = self.clone();
		parent.add_child_to_be_created(
			Box::new(move || {
				let opts = &mut me.0.as_mut().opts;

				let mut wcx = WNDCLASSEX::default();
				let mut class_name_buf = WString::new();
				opts.generate_wndclassex(
					me.0.base.parent_hinstance()?, &mut wcx, &mut class_name_buf)?;
				me.0.base.register_class(&mut wcx)?;

				multiply_dpi(Some(&mut opts.position), Some(&mut opts.size))?;

				me.0.base.create_window( // may panic
					&class_name_buf.to_string(),
					None,
					IdMenu::None,
					opts.position, opts.size,
					opts.ex_style,
					opts.style,
				)
			})
		);
	}

	fn default_message_handlers(&self) {
		self.events_ref().wm_nc_paint({
			let self2 = self.clone();
			move |p| { paint_control_borders(*self2.hwnd_ref(), p).ok(); }
		});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`CustomControl`](crate::gui::CustomControl)
/// programatically with [`CustomControl::new`](crate::gui::CustomControl::new).
pub struct CustomControlOpts {
	/// Window class name to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to an auto-generated string.
	pub class_name: String,
	/// Window class styles to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::CS::DBLCLKS`.
	pub class_style: co::CS,
	/// Window main icon to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to none.
	pub class_icon: HICON,
	/// Window cursor to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::IDC::ARROW`.
	pub class_cursor: HCURSOR,
	/// Window background brush to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::COLOR::WINDOW`.
	pub class_bg_brush: HBRUSH,

	/// Position of control within parent's client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Size of window, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub size: SIZE,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::TABSTOP | WS::GROUP | WS::VISIBLE | WS::CLIPCHILDREN | WS::CLIPSIBLINGS`.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	///
	/// Suggestion:
	/// * `WS_EX::CLIENTEDGE` to have a border.
	pub ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
}

impl Default for CustomControlOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: unsafe { HICON::null_handle() },
			class_cursor: unsafe { HCURSOR::null_handle() },
			class_bg_brush: HBRUSH::from_sys_color(co::COLOR::WINDOW),
			position: POINT { x: 0, y: 0 },
			size: SIZE { cx: 0, cy: 0 },
			style: co::WS::CHILD | co::WS::TABSTOP | co::WS::GROUP | co::WS::VISIBLE | co::WS::CLIPCHILDREN | co::WS::CLIPSIBLINGS,
			ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
		}
	}
}

impl CustomControlOpts {
	fn generate_wndclassex<'a, 'b>( // https://stackoverflow.com/q/65481548/6923555
		&self,
		hinst: HINSTANCE,
		wcx: &mut WNDCLASSEX<'_, 'a>,
		class_name_buf: &'a mut WString) -> WinResult<()>
	{
		wcx.hInstance = hinst;
		wcx.style = self.class_style;
		wcx.hIcon = self.class_icon;
		wcx.hIconSm = self.class_icon;
		wcx.hbrBackground = self.class_bg_brush;

		wcx.hCursor = match self.class_cursor.as_opt() {
			Some(h) => h,
			None => HINSTANCE::oem().LoadCursor(IdIdcStr::Idc(co::IDC::ARROW))?,
		};

		if wcx.lpszClassName().is_empty() {
			*class_name_buf = WindowBase::generate_wcx_class_name_hash(&wcx);
			wcx.set_lpszClassName(class_name_buf);
		}

		Ok(())
	}
}
