use std::cell::UnsafeCell;
use std::sync::Arc;

use crate::co;
use crate::enums::{IdIdcStr, IdMenu};
use crate::gui::globals::{multiply_dpi, paint_control_borders};
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;
use crate::gui::window_base::WindowBase;
use crate::handles::{HBRUSH, HCURSOR, HICON, HINSTANCE, HWND};
use crate::structs::{POINT, SIZE, WNDCLASSEX};
use crate::WString;

#[derive(Clone)]
pub struct WindowControl {
	obj: Arc<UnsafeCell<Obj>>,
}

struct Obj { // actual fields of WindowControl
	base: WindowBase,
	opts: CustomControlOpts,
}

cref_mref!(WindowControl);

impl WindowControl {
	pub fn new(parent: &dyn Parent, opts: CustomControlOpts) -> WindowControl {
		let wnd = Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: WindowBase::new(Some(parent)),
					opts,
				},
			)),
		};
		wnd.default_message_handlers();
		wnd
	}

	pub fn create(&self) -> Result<(), co::ERROR> {
		let opts = &mut self.mref().opts;
		let hinst = self.cref().base.hwnd().hinstance();

		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = WString::new();
		opts.generate_wndclassex(hinst, &mut wcx, &mut class_name_buf)?;
		self.cref().base.register_class(&mut wcx)?;

		multiply_dpi(Some(&mut opts.position), Some(&mut opts.size))?;

		self.cref().base.create_window( // may panic
			hinst,
			&class_name_buf.to_string(),
			None,
			IdMenu::None,
			opts.position, opts.size,
			opts.ex_style,
			opts.style,
		)?;

		Ok(())
	}

	pub fn hwnd(&self) -> HWND {
		*self.cref().base.hwnd()
	}

	pub fn on(&self) -> &MsgEvents {
		self.cref().base.on()
	}

	fn default_message_handlers(&self) {
		self.on().wm_nc_paint({
			let self2 = self.clone();
			move |p| { paint_control_borders(self2.hwnd(), p).ok(); }
		});
	}
}

//------------------------------------------------------------------------------

/// Options for [`CustomControl::new`](crate::gui::CustomControl::new).
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

	/// Position of window within parent's client area, in pixels, to be
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
	/// Defaults to `co::WS::CHILD | co::WS::TABSTOP | co::WS::GROUP | co::WS::VISIBLE | co::WS::CLIPCHILDREN | co::WS::CLIPSIBLINGS`.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `co::WS_EX::LEFT`.
	///
	/// Suggestion:
	/// * `co::WS_EX::CLIENTEDGE` to have a border.
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
			class_bg_brush: unsafe { HBRUSH::null_handle() },
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
		class_name_buf: &'a mut WString) -> Result<(), co::ERROR>
	{
		wcx.hInstance = hinst;
		wcx.style = self.class_style;
		wcx.hIcon = self.class_icon;
		wcx.hIconSm = self.class_icon;

		wcx.hbrBackground = self.class_bg_brush.as_opt()
			.unwrap_or_else(|| HBRUSH::from_sys_color(co::COLOR::WINDOW));

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
