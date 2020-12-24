use std::ffi::c_void;

use crate::co::WM;
use crate::handles::HBRUSH;

/// Possible return values of
/// [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
///
/// Same variants of [`Wm`](crate::msg::Wm).
pub enum RetWm {
	Activate(()),
	ActivateApp(()),
	AppCommand(()),
	Close(()),
	Command(()),
	Create(i32),
	CtlColorBtn(HBRUSH),
	CtlColorDlg(HBRUSH),
	CtlColorEdit(HBRUSH),
	CtlColorListBox(HBRUSH),
	CtlColorListScrollBar(HBRUSH),
	CtlColorListStatic(HBRUSH),
	WmDestroy(()),
	WmDropFiles(()),
	EndSession(()),
	WmInitDialog(bool),
	WmInitMenuPopup(()),
	WmNcCreate(bool),
	WmNcDestroy(()),
	WmNcPaint(()),
	WmNotify(isize),
	WmNull(()),
	WmPaint(()),
	WmSetFocus(()),
	WmSize(()),
	WmSizing(()),
	WmTimer(()),
}

impl From<RetWm> for isize {
	fn from(r: RetWm) -> isize {
		match r {
			RetWm::AppCommand(_) => 1,
			RetWm::Create(r) => r as isize,
			RetWm::CtlColorBtn(b) => (unsafe { b.as_ptr() }) as isize,
			RetWm::CtlColorDlg(b) => (unsafe { b.as_ptr() }) as isize,
			RetWm::CtlColorEdit(b) => (unsafe { b.as_ptr() }) as isize,
			RetWm::CtlColorListBox(b) => (unsafe { b.as_ptr() }) as isize,
			RetWm::CtlColorListScrollBar(b) => (unsafe { b.as_ptr() }) as isize,
			RetWm::CtlColorListStatic(b) => (unsafe { b.as_ptr() }) as isize,
			RetWm::WmInitDialog(r) => r as isize,
			RetWm::WmNcCreate(r) => r as isize,
			RetWm::WmSizing(_) => 1,
			RetWm::WmNotify(r) => r,
			_ => 0
		}
	}
}

impl RetWm {
	/// Converts the `isize` result from [`SendMessage`](crate::HWND::SendMessage)
	/// to the expected value type, according to the message identifier.
	pub fn from_msg_ret(m: WM, ret: isize) -> RetWm {
		match m {
			WM::ACTIVATE => RetWm::Activate(()),
			WM::ACTIVATEAPP => RetWm::ActivateApp(()),
			WM::APPCOMMAND => RetWm::AppCommand(()),
			WM::CLOSE => RetWm::Close(()),
			WM::COMMAND => RetWm::Command(()),
			WM::CREATE => RetWm::Create(ret as i32),
			WM::CTLCOLORBTN => RetWm::CtlColorBtn(unsafe { HBRUSH::from_ptr(ret as *mut c_void) }),
			WM::CTLCOLORDLG => RetWm::CtlColorDlg(unsafe { HBRUSH::from_ptr(ret as *mut c_void) }),
			WM::CTLCOLOREDIT => RetWm::CtlColorEdit(unsafe { HBRUSH::from_ptr(ret as *mut c_void) }),
			WM::CTLCOLORLISTBOX => RetWm::CtlColorListBox(unsafe { HBRUSH::from_ptr(ret as *mut c_void) }),
			WM::CTLCOLORSCROLLBAR => RetWm::CtlColorListScrollBar(unsafe { HBRUSH::from_ptr(ret as *mut c_void) }),
			WM::CTLCOLORSTATIC => RetWm::CtlColorListStatic(unsafe { HBRUSH::from_ptr(ret as *mut c_void) }),
			WM::DESTROY => RetWm::WmDestroy(()),
			WM::DROPFILES => RetWm::WmDropFiles(()),
			WM::ENDSESSION => RetWm::EndSession(()),
			WM::INITDIALOG => RetWm::WmInitDialog(ret != 0),
			WM::INITMENUPOPUP => RetWm::WmInitMenuPopup(()),
			WM::NCCREATE => RetWm::WmNcCreate(ret != 0),
			WM::NCDESTROY => RetWm::WmNcDestroy(()),
			WM::NCPAINT => RetWm::WmNcPaint(()),
			WM::NOTIFY => RetWm::WmNotify(ret),
			WM::NULL => RetWm::WmNull(()),
			WM::PAINT => RetWm::WmPaint(()),
			WM::SETFOCUS => RetWm::WmSetFocus(()),
			WM::SIZE => RetWm::WmSize(()),
			WM::SIZING => RetWm::WmSizing(()),
			WM::TIMER => RetWm::WmTimer(()),
			_ => panic!("Unsupported message: {}.", m),
		}
	}
}