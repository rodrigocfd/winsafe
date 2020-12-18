use std::collections::HashMap;

use crate::co;
use crate::msg;

/// Exposes events of window messages.
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

impl Events {
	pub(super) fn new() -> Events {
		let msgs_heap = Box::new(HashMap::new()); // alloc memory on the heap

		Self {
			original: true, // this is the object that will actually release the memory
			msgs: Box::into_raw(msgs_heap), // leak and keep the pointer
		}
	}

	/// Adds a handler to any [window message](crate::co::WM).
	pub fn wm<F>(&self, ident: co::WM, func: F)
		where F: FnMut(msg::Wm) -> isize + Send + Sync + 'static,
	{
		unsafe { self.msgs.as_mut() }
			.unwrap().insert(ident, Box::new(func));
	}

	/// Adds a handler to [`WM_CREATE`](crate::msg::WmCreate) message.
	pub fn wm_create<F>(&self, func: F)
		where F: FnMut(msg::WmCreate) -> i32 + Send + Sync + 'static,
	{
		self.wm(co::WM::CREATE, {
			let mut func = func;
			move |p| {
				if let msg::Wm::Create(p) = p {
					func(p) as isize
				} else {
					panic!("Event incorrectly handled internally. This is a bug.");
				}
			}
		});
	}

	pub fn wm_init_dialog<F>(&self, func: F)
		where F: FnMut(msg::WmInitDialog) -> bool + Send + Sync + 'static,
	{
		self.wm(co::WM::INITDIALOG, {
			let mut func = func;
			move |p| {
				if let msg::Wm::InitDialog(p) = p {
					func(p) as isize
				} else {
					panic!("Event incorrectly handled internally. This is a bug.");
				}
			}
		});
	}
}