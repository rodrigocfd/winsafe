use std::collections::HashMap;

use crate::co;
use crate::handles::HDC;
use crate::msg;

struct MsgMaps {
	msgs: HashMap< // ordinary WM messages
		co::WM,
		Box<dyn FnMut(msg::Wm) -> isize + Send + Sync + 'static>,
	>,
	cmds: HashMap< // WM_COMMAND notifications
		(co::CMD, u16), // code, ctrl_id
		Box<dyn FnMut() + Send + Sync + 'static>,
	>,
	nfys: HashMap< // WM_NOTIFY notifications
		(u16, co::NM), // idFrom, code
		Box<dyn FnMut(msg::WmNotify) -> isize + Send + Sync + 'static>,
	>,
}

/// Allows adding closures to handle window
/// [messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
pub struct Events {
	original: bool,

	// Performs manual memory management by keeping a raw pointer to a
	// heap-allocated memory block. All cloned objects will have a pointer to the
	// memory block of the original object, which must outlive them all. This
	// could be safely achieved with Arc and RwLock, but it would incur in an
	// unnecessary cost, since Events is shared only between a parent window and
	// its child controls, and the controls only use it to add events at the
	// beginning of the program. Adding events later is not allowed.
	msg_maps: *mut MsgMaps,
}

unsafe impl Send for Events {}
unsafe impl Sync for Events {}

impl Clone for Events {
	fn clone(&self) -> Self {
		Events {
			original: false, // clones won't release the memory
			msg_maps: self.msg_maps, // simply copy away the pointer
		}
	}
}

impl Drop for Events {
	fn drop(&mut self) {
		if self.original {
			unsafe { Box::from_raw(self.msg_maps); } // release the memory
		}
	}
}

/// Panics when a message is incorrectly handled. Should never happen.
macro_rules! panic_msg {
	() => { panic!("Internal event incorrectly handled. This is a bug."); };
}

/// A message which has no parameters and returns zero.
macro_rules! empty_wm {
	(
		$name:ident, $wmconst:expr, $wmenum:path,
		$(#[$attr:meta])*
	) => {
		$(#[$attr])*
		pub fn $name<F>(&self, func: F)
			where F: FnMut() + Send + Sync + 'static,
		{
			self.wm($wmconst, {
				let mut func = func;
				move |p| {
					match p {
						$wmenum(_) => { func(); 0 },
						_ => panic_msg!(),
					}
				}
			});
		}
	};
}

impl Events {
	pub(super) fn new() -> Events {
		let heap_msg_maps = Box::new( // alloc memory on the heap
			MsgMaps {
				msgs: HashMap::new(),
				cmds: HashMap::new(),
				nfys: HashMap::new(),
			}
		);

		Self {
			original: true, // this is the object that will actually release the memory
			msg_maps: Box::into_raw(heap_msg_maps), // leak and keep the pointer
		}
	}

	/// Adds a handler to any [window message](crate::co::WM).
	///
	/// You should always prefer the specific message handlers, which will give
	/// you the correct message parameters.
	pub fn wm<F>(&self, ident: co::WM, func: F)
		where F: FnMut(msg::Wm) -> isize + Send + Sync + 'static,
	{
		unsafe { self.msg_maps.as_mut() }.unwrap()
			.msgs.insert(ident, Box::new(func));
	}

	/// Adds a handler to [`WM_COMMAND`](crate::msg::WmCommand) message.
	///
	/// A command notification must be narrowed by the
	/// [command code](crate::co::CMD) and the control ID, so the closure will
	/// be fired for that specific control at that specific event.
	///
	/// You should always prefer the specific command notification handlers,
	/// which will give you the correct message parameters.
	pub fn wm_command<F>(&self, code: co::CMD, ctrl_id: u16, func: F)
		where F: FnMut() + Send + Sync + 'static,
	{
		unsafe { self.msg_maps.as_mut() }.unwrap()
			.cmds.insert((code, ctrl_id), Box::new(func));
	}

	/// Adds a handler to [`WM_NOTIFY`](crate::msg::WmNotify) message.
	///
	/// A notification must be narrowed by the [notification code](crate::co::NM)
	/// and the control ID, so the closure will be fired for that specific
	/// control at the specific event.
	///
	/// You should always prefer the specific notification handlers, which
	/// will give you the correct notification struct.
	pub fn wm_notify<F>(&self, id_from: u16, code: co::NM, func: F)
		where F: FnMut(msg::WmNotify) -> isize + Send + Sync + 'static,
	{
		unsafe { self.msg_maps.as_mut() }.unwrap()
			.nfys.insert((id_from, code), Box::new(func));
	}

	/// Adds a handler to [`WM_ACTIVATE`](crate::msg::WmActivate) message.
	pub fn wm_activate<F>(&self, func: F)
		where F: FnMut(msg::WmActivate) + Send + Sync + 'static,
	{
		self.wm(co::WM::ACTIVATE, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::Activate(p) => { func(p); 0 },
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_ACTIVATEAPP`](crate::msg::WmActivateApp) message.
	pub fn wm_activate_app<F>(&self, func: F)
		where F: FnMut(msg::WmActivateApp) + Send + Sync + 'static,
	{
		self.wm(co::WM::ACTIVATEAPP, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::ActivateApp(p) => { func(p); 0 },
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_APPCOMMAND`](crate::msg::WmAppCommand) message.
	pub fn wm_app_command<F>(&self, func: F)
		where F: FnMut(msg::WmAppCommand) + Send + Sync + 'static,
	{
		self.wm(co::WM::APPCOMMAND, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::AppCommand(p) => { func(p); 1 },
					_ => panic_msg!(),
				}
			}
		});
	}

	empty_wm! { wm_close, co::WM::CLOSE, msg::Wm::Close,
		/// Adds a handler to [`WM_CLOSE`](crate::msg::WmClose) message.
	}

	/// Adds a handler to [`WM_CREATE`](crate::msg::WmCreate) message.
	pub fn wm_create<F>(&self, func: F)
		where F: FnMut(msg::WmCreate) -> i32 + Send + Sync + 'static,
	{
		self.wm(co::WM::CREATE, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::Create(p) => { func(p) as isize },
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_CTLCOLORBTN`](crate::msg::WmCtlColorBtn) message.
	pub fn wm_ctl_color_btn<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorBtn) -> HDC + Send + Sync + 'static,
	{
		self.wm(co::WM::CTLCOLORBTN, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::CtlColorBtn(p) => (unsafe { func(p).as_ptr() }) as isize,
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_CTLCOLORDLG`](crate::msg::WmCtlColorDlg) message.
	pub fn wm_ctl_color_dlg<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorDlg) -> HDC + Send + Sync + 'static,
	{
		self.wm(co::WM::CTLCOLORDLG, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::CtlColorDlg(p) => (unsafe { func(p).as_ptr() }) as isize,
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_CTLCOLOREDIT`](crate::msg::WmCtlColorEdit) message.
	pub fn wm_ctl_color_edit<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorEdit) -> HDC + Send + Sync + 'static,
	{
		self.wm(co::WM::CTLCOLOREDIT, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::CtlColorEdit(p) => (unsafe { func(p).as_ptr() }) as isize,
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_CTLCOLORLISTBOX`](crate::msg::WmCtlColorListBox) message.
	pub fn wm_ctl_color_list_box<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorListBox) -> HDC + Send + Sync + 'static,
	{
		self.wm(co::WM::CTLCOLORLISTBOX, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::CtlColorListBox(p) => (unsafe { func(p).as_ptr() }) as isize,
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_CTLCOLORSCROLLBAR`](crate::msg::WmCtlColorScrollBar) message.
	pub fn wm_ctl_color_scroll_bar<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorScrollBar) -> HDC + Send + Sync + 'static,
	{
		self.wm(co::WM::CTLCOLORSCROLLBAR, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::CtlColorListScrollBar(p) => (unsafe { func(p).as_ptr() }) as isize,
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_CTLCOLORSTATIC`](crate::msg::WmCtlColorStatic) message.
	pub fn wm_ctl_color_static<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorStatic) -> HDC + Send + Sync + 'static,
	{
		self.wm(co::WM::CTLCOLORSTATIC, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::CtlColorListStatic(p) => (unsafe { func(p).as_ptr() }) as isize,
					_ => panic_msg!(),
				}
			}
		});
	}

	empty_wm! { wm_destroy, co::WM::DESTROY, msg::Wm::Destroy,
		/// Adds a handler to [`WM_DESTROY`](crate::msg::WmDestroy) message.
	}

	/// Adds a handler to [`WM_DROPFILES`](crate::msg::WmDropFiles) message.
	pub fn wm_drop_files<F>(&self, func: F)
		where F: FnMut(msg::WmDropFiles) + Send + Sync + 'static,
	{
		self.wm(co::WM::DROPFILES, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::DropFiles(p) => { func(p); 0 },
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_ENDSESSION`](crate::msg::WmEndSession) message.
	pub fn wm_end_session<F>(&self, func: F)
		where F: FnMut(msg::WmEndSession) + Send + Sync + 'static,
	{
		self.wm(co::WM::ENDSESSION, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::EndSession(p) => { func(p); 0 },
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_INITDIALOG`](crate::msg::WmInitDialog) message.
	pub fn wm_init_dialog<F>(&self, func: F)
		where F: FnMut(msg::WmInitDialog) -> bool + Send + Sync + 'static,
	{
		self.wm(co::WM::INITDIALOG, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::InitDialog(p) => { func(p) as isize },
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_INITMENUPOPUP`](crate::msg::WmInitMenuPopup) message.
	pub fn wm_init_menu_popup<F>(&self, func: F)
		where F: FnMut(msg::WmInitMenuPopup) -> bool + Send + Sync + 'static,
	{
		self.wm(co::WM::INITMENUPOPUP, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::InitMenuPopup(p) => { func(p); 0 },
					_ => panic_msg!(),
				}
			}
		});
	}

	empty_wm! { wm_nc_destroy, co::WM::NCDESTROY, msg::Wm::NcDestroy,
		/// Adds a handler to [`WM_NCDESTROY`](crate::msg::WmNcDestroy) message.
	}

	empty_wm! { wm_nc_paint, co::WM::NCPAINT, msg::Wm::NcPaint,
		/// Adds a handler to [`WM_NCPAINT`](crate::msg::WmNcPaint) message.
	}

	empty_wm! { wm_null, co::WM::NULL, msg::Wm::Null,
		/// Adds a handler to [`WM_NULL`](crate::msg::WmNull) message.
		///
		/// Usually this message is not handled.
	}

	empty_wm! { wm_paint, co::WM::PAINT, msg::Wm::Paint,
		/// Adds a handler to [`WM_PAINT`](crate::msg::WmPaint) message.
	}

	/// Adds a handler to [`WM_SETFOCUS`](crate::msg::WmSetFocus) message.
	pub fn wm_set_focus<F>(&self, func: F)
		where F: FnMut(msg::WmSetFocus) + Send + Sync + 'static,
	{
		self.wm(co::WM::SETFOCUS, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::SetFocus(p) => { func(p); 0 },
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_SIZE`](crate::msg::WmSize) message.
	pub fn wm_size<F>(&self, func: F)
		where F: FnMut(msg::WmSize) + Send + Sync + 'static,
	{
		self.wm(co::WM::SIZE, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::Size(p) => { func(p); 0 },
					_ => panic_msg!(),
				}
			}
		});
	}

	/// Adds a handler to [`WM_SIZING`](crate::msg::WmSizing) message.
	pub fn wm_sizing<F>(&self, func: F)
		where F: FnMut(msg::WmSizing) + Send + Sync + 'static,
	{
		self.wm(co::WM::SIZING, {
			let mut func = func;
			move |p| {
				match p {
					msg::Wm::Sizing(p) => { func(p); 1 },
					_ => panic_msg!(),
				}
			}
		});
	}
}