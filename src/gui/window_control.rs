use std::cell::UnsafeCell;
use std::error::Error;
use std::sync::Arc;

use crate::co;
use crate::enums::{IdIdcStr, IdMenu};
use crate::gui::globals::{multiply_dpi, paint_control_borders};
use crate::gui::events::MsgEvents;
use crate::gui::window_base::WindowBase;
use crate::handles::{HBRUSH, HCURSOR, HICON, HINSTANCE, HWND};
use crate::structs::{POINT, SIZE, WNDCLASSEX};
use crate::WString;

/// Custom control.
#[derive(Clone)]
pub struct WindowControl {
	obj: Arc<UnsafeCell<Obj>>,
}

struct Obj { // actual fields of WindowControl
	base: WindowBase,
	opts: WindowControlOpts,
}

unsafe impl Send for WindowControl {}
unsafe impl Sync for WindowControl {}

cref_mref!(WindowControl);

impl WindowControl {
	pub fn new(opts: WindowControlOpts) -> WindowControl {
		let wnd = Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: WindowBase::new(),
					opts,
				},
			)),
		};
		wnd.default_message_handlers();
		wnd
	}

	/// Returns the underlying handle for this window.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the window is created.
	pub fn hwnd(&self) -> HWND {
		*self.cref().base.hwnd()
	}

	/// Exposes the window events.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before window
	/// creation.
	///
	/// # Examples
	///
	/// Prints some info right after the window is created:
	///
	/// ```rust,ignore
	/// use winsafe::gui::WindowControl;
	///
	/// let wnd: WindowControl; // initialize it somewhere...
	///
	/// wnd.on().wm_create({
	///   let wnd = wnd.clone(); // pass into the closure
	///   move |parms| {
	///     println!("HWND: {}, client area: {}x{}",
	///       wnd.hwnd(),
	///       parms.createstruct.cx, parms.createstruct.cy);
	///     0
	///   }
	/// });
	/// ```
	pub fn on(&self) -> &MsgEvents {
		self.cref().base.on()
	}

	/// Physically creates the control within the parent window by calling
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx). This method should be
	/// be called within parent window's `WM_CREATE` or `WM_INITDIALOG` events.
	///
	/// # Panics
	///
	/// Panics if the control is already created, or if the parent window was not
	/// created yet.
	pub fn create(&self,
		parent_hwnd: HWND, pos: POINT, size: SIZE) -> Result<(), Box<dyn Error>>
	{
		let hinst = parent_hwnd.Instance();

		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = WString::new();
		self.cref().opts.generate_wndclassex(hinst, &mut wcx, &mut class_name_buf)?;
		self.cref().base.register_class(&mut wcx)?;

		let mut pos = pos;
		let mut size = size;
		multiply_dpi(Some(&mut pos), Some(&mut size))?;

		self.cref().base.create_window( // may panic
			hinst,
			Some(parent_hwnd),
			&class_name_buf.to_string(),
			None,
			IdMenu::None,
			pos, size,
			self.cref().opts.ex_style,
			self.cref().opts.style,
		)?;

		Ok(())
	}

	/// Adds the default event processing.
	fn default_message_handlers(&self) {
		self.on().wm_nc_paint({
			let self2 = self.clone();
			move |p| { paint_control_borders(self2.hwnd(), p).ok(); }
		});
	}
}

//------------------------------------------------------------------------------

/// Options for [`WindowControl::new`](crate::gui::WindowControl::new).
pub struct WindowControlOpts {
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

impl Default for WindowControlOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: unsafe { HICON::null_handle() },
			class_cursor: unsafe { HCURSOR::null_handle() },
			class_bg_brush: unsafe { HBRUSH::null_handle() },
			style: co::WS::CHILD | co::WS::TABSTOP | co::WS::GROUP | co::WS::VISIBLE | co::WS::CLIPCHILDREN | co::WS::CLIPSIBLINGS,
			ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
		}
	}
}

impl WindowControlOpts {
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
