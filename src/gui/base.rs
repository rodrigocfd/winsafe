use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::gui::events::{MsgEvents, ProcessResult};
use crate::gui::immut::Immut;
use crate::gui::parent::Parent;
use crate::msg::Wm;
use crate::handles::{HINSTANCE, HWND};

/// Base to `WindowBase` and `DialogBase`.
pub struct Base {
	hwnd: HWND,
	events: MsgEvents,
	ptr_parent_hwnd: Option<NonNull<HWND>>, // used only in control creation
	children_creates: Immut<Vec<Box<dyn Fn() -> WinResult<()> + 'static>>>,
}

impl Parent for Base {
	fn hwnd_ref(&self) -> &HWND {
		&self.hwnd
	}

	fn events_ref(&self) -> &MsgEvents {
		if !self.hwnd.is_null() {
			panic!("Cannot add event after window is created.");
		}
		&self.events
	}

	fn add_child_to_be_created(&self,
		func: Box<dyn Fn() -> WinResult<()> + 'static>)
	{
		self.children_creates.as_mut().push(func);
	}
}

impl Base {
	pub fn new(parent: Option<&dyn Parent>) -> Base {
		Self {
			hwnd: unsafe { HWND::null_handle() },
			events: MsgEvents::new(),
			ptr_parent_hwnd: parent.map(|parent| NonNull::from(parent.hwnd_ref())), // ref implicitly converted to pointer
 			children_creates: Immut::new(Vec::with_capacity(16)), // arbitrary, prealloc for speed
		}
	}

	pub fn set_hwnd(&mut self, hwnd: HWND) {
		self.hwnd = hwnd;
	}

	pub fn parent_hwnd(&self) -> Option<HWND> {
		self.ptr_parent_hwnd.map(|ptr| unsafe { *ptr.as_ref() })
	}

	pub fn parent_hinstance(&self) -> WinResult<HINSTANCE> {
		Ok(match self.parent_hwnd() {
			Some(hparent) => hparent.hinstance(),
			None => HINSTANCE::GetModuleHandle(None)?,
		})
	}

	pub fn process_message(&mut self, wm_any: Wm) -> ProcessResult {
		self.events.process_message(wm_any)
	}

	pub fn create_children(&self) -> WinResult<()> {
		for creat in self.children_creates.iter() {
			creat()?;
		}
		Ok(())
	}
}
