use std::collections::HashMap;

use crate::co;
use crate::msg;

/// Allows you to add closures to handle window
/// [messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
pub struct Events {
	original: bool,

	// Performs manual memory management by keeping a raw pointer to a
	// heap-allocated memory block. All cloned objects will have a pointer to the
	// memory block of the original object, which must outlive them all. This
	// could be safely achieved with `Arc` and `RwLock`, but it would incur in an
	// unnecessary cost, since `Events` is shared only between a parent window
	// and its child controls, and the controls only use it to add events at the
	// beginning of the program. Adding events later is not allowed.
	msgs: *mut HashMap<
		co::WM,
		Box<dyn FnMut(msg::Wm) -> isize + Send + Sync + 'static>,
	>,
}

unsafe impl Send for Events {}
unsafe impl Sync for Events {}

impl Clone for Events {
	fn clone(&self) -> Self {
		Events {
			original: false, // clones won't release the memory
			msgs: self.msgs, // simply copy away the pointer
		}
	}
}

impl Drop for Events {
	fn drop(&mut self) {
		if self.original {
			unsafe { Box::from_raw(self.msgs); } // release the memory
		}
	}
}

/// Implements a method that receives a specific message which returns a
/// value of a specific type, and wraps it in a generic message.
macro_rules! wm_ret_t {
	($name:ident, $arg:ty, $ret:ty, $wmconst:expr, $wmpat:path) => {
		pub fn $name<F>(&self, func: F)
			where F: FnMut($arg) -> $ret + Send + Sync + 'static,
		{
			self.wm($wmconst, {
				let mut func = func;
				move |p| {
					if let $wmpat(p) = p {
						func(p) as isize // convert user returned value
					} else {
						panic!("Event incorrectly handled internally. This is a bug.");
					}
				}
			});
		}
	};
}

/// Implements a method that receives a specific message which returns the given
/// value, and wraps it in a generic message.
macro_rules! wm_ret_v {
	($name:ident, $arg:ty, $wmconst:expr, $wmpat:path, $retval:expr) => {
		pub fn $name<F>(&self, func: F)
			where F: FnMut($arg) + Send + Sync + 'static
		{
			self.wm($wmconst, {
				let mut func = func;
				move |p| {
					if let $wmpat(p) = p {
						func(p);
						$retval // ignore user returned value, return specific value
					} else {
						panic!("Event incorrectly handled internally. This is a bug.");
					}
				}
			});
		}
	};
}

impl Events {
	pub(super) fn new() -> Events {
		let msgs_heap = Box::new(HashMap::new()); // alloc memory on the heap

		Self {
			original: true, // this is the object that will actually release the memory
			msgs: Box::into_raw(msgs_heap), // leak and keep the pointer
		}
	}

	/// Adds a handler to any [window message](crate::co::WM).
	///
	/// You should always prefer the specific message handlers, which will give
	/// you the correct message parameters.
	pub fn wm<F>(&self, ident: co::WM, func: F)
		where F: FnMut(msg::Wm) -> isize + Send + Sync + 'static,
	{
		unsafe { self.msgs.as_mut() }
			.unwrap().insert(ident, Box::new(func));
	}

	wm_ret_v!(wm_activate, msg::WmActivate, co::WM::ACTIVATE, msg::Wm::Activate, 0);
	wm_ret_v!(wm_activate_app, msg::WmActivateApp, co::WM::ACTIVATEAPP, msg::Wm::ActivateApp, 0);
	wm_ret_v!(wm_close, msg::WmClose, co::WM::CLOSE, msg::Wm::Close, 0);
	wm_ret_v!(wm_command, msg::WmCommand, co::WM::COMMAND, msg::Wm::Command, 0);
	wm_ret_t!(wm_create, msg::WmCreate, i32, co::WM::CREATE, msg::Wm::Create);
	wm_ret_v!(wm_destroy, msg::WmDestroy, co::WM::DESTROY, msg::Wm::Destroy, 0);
	wm_ret_v!(wm_drop_files, msg::WmDropFiles, co::WM::DROPFILES, msg::Wm::DropFiles, 0);
	wm_ret_t!(wm_init_dialog, msg::WmInitDialog, bool, co::WM::INITDIALOG, msg::Wm::InitDialog);
	wm_ret_v!(wm_init_menu_popup, msg::WmInitMenuPopup, co::WM::INITMENUPOPUP, msg::Wm::InitMenuPopup, 0);
	wm_ret_t!(wm_notify, msg::WmNotify, isize, co::WM::NOTIFY, msg::Wm::Notify);
	wm_ret_v!(wm_null, msg::WmNull, co::WM::NULL, msg::Wm::Null, 0);
	wm_ret_v!(wm_size, msg::WmSize, co::WM::SIZE, msg::Wm::Size, 0);
	wm_ret_v!(wm_sizing, msg::WmSizing, co::WM::SIZING, msg::Wm::Sizing, 1);
}