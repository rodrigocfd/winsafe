use std::error::Error;

use crate::co;
use crate::funcs as f;
use crate::gui::events::Events;
use crate::gui::Parent;
use crate::gui::window_base::WindowBase;
use crate::handles::{HINSTANCE, HWND};

/// Main application window.
#[derive(Clone)]
pub struct WindowMain {
	base: WindowBase,
}

impl WindowMain {
	/// Creates a new `WindowMain` object.
	pub fn new() -> WindowMain {
		Self {
			base: WindowBase::new(),
		}
	}

	/// Returns the underlying handle for this window.
	pub fn hwnd(&self) -> HWND {
		self.base.hwnd()
	}

	/// Creates the window and runs the main application loop. This function will
	/// block until the window is closed.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn run_as_main(
		&self, cmd_show: Option<co::SW>) -> Result<i32, Box<dyn Error>>
	{
		if f::IsWindowsVistaOrGreater()
			.map_err(|e| Box::new(e))?
		{
			f::SetProcessDPIAware()
				.map_err(|_| Into::<Box<dyn Error>>::into(
					String::from("SetProcessDPIAware failed.")
				))?;
		}

		f::InitCommonControls();


		let hinst = HINSTANCE::GetModuleHandle(None)
			.map_err(|e| Box::new(e))?;

		Ok(0)
	}
}

impl Parent for WindowMain {
	fn on(&self) -> Events {
		self.base.on()
	}
}