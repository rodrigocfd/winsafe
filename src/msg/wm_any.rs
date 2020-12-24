use core::panic;

use crate::co;
use crate::msg;

/// Possible
/// [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
pub enum Wm<'a, 'b, 'c> {
	Activate(msg::WmActivate),
	ActivateApp(msg::WmActivateApp),
	AppCommand(msg::WmAppCommand),
	Close(msg::WmClose),
	Command(msg::WmCommand),
	Create(msg::WmCreate<'a, 'b, 'c>),
	CtlColorBtn(msg::WmCtlColorBtn),
	CtlColorDlg(msg::WmCtlColorDlg),
	CtlColorEdit(msg::WmCtlColorEdit),
	CtlColorListBox(msg::WmCtlColorListBox),
	CtlColorListScrollBar(msg::WmCtlColorScrollBar),
	CtlColorListStatic(msg::WmCtlColorStatic),
	Destroy(msg::WmDestroy),
	DropFiles(msg::WmDropFiles),
	EndSession(msg::WmEndSession),
	InitDialog(msg::WmInitDialog),
	InitMenuPopup(msg::WmInitMenuPopup),
	NcCreate(msg::WmNcCreate<'a, 'b, 'c>),
	NcDestroy(msg::WmNcDestroy),
	NcPaint(msg::WmNcPaint),
	Notify(msg::WmNotify<'a>),
	Null(msg::WmNull),
	Paint(msg::WmPaint),
	SetFocus(msg::WmSetFocus),
	Size(msg::WmSize),
	Sizing(msg::WmSizing<'a>),
	Timer(msg::WmTimer),
}

//------------------------------------------------------------------------------

/// Generic
/// [window message](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues)
/// parameters.
#[derive(Copy, Clone)]
pub struct WmAny {
	pub msg: co::WM,
	pub wparam: usize,
	pub lparam: isize,
}

impl WmAny {
	/// Returns a [`Wm`](crate::msg::Wm) enum, which can be matched to identify
	/// the exact message type.
	pub fn message<'a, 'b, 'c>(self) -> Wm<'a, 'b, 'c> {
		match self.msg {
			co::WM::ACTIVATE => Wm::Activate(self.into()),
			co::WM::ACTIVATEAPP => Wm::ActivateApp(self.into()),
			co::WM::APPCOMMAND => Wm::AppCommand(self.into()),
			co::WM::CLOSE => Wm::Close(self.into()),
			co::WM::CREATE => Wm::Create(self.into()),
			co::WM::CTLCOLORBTN => Wm::CtlColorBtn(self.into()),
			co::WM::CTLCOLORDLG => Wm::CtlColorDlg(self.into()),
			co::WM::CTLCOLOREDIT => Wm::CtlColorEdit(self.into()),
			co::WM::CTLCOLORLISTBOX => Wm::CtlColorListBox(self.into()),
			co::WM::CTLCOLORSCROLLBAR => Wm::CtlColorListScrollBar(self.into()),
			co::WM::CTLCOLORSTATIC => Wm::CtlColorListStatic(self.into()),
			co::WM::DESTROY => Wm::Destroy(self.into()),
			co::WM::DROPFILES => Wm::DropFiles(self.into()),
			co::WM::ENDSESSION => Wm::EndSession(self.into()),
			co::WM::INITDIALOG => Wm::InitDialog(self.into()),
			co::WM::INITMENUPOPUP => Wm::InitMenuPopup(self.into()),
			co::WM::NCCREATE => Wm::NcCreate(self.into()),
			co::WM::NCDESTROY => Wm::NcDestroy(self.into()),
			co::WM::NCPAINT => Wm::NcPaint(self.into()),
			co::WM::NOTIFY => Wm::Notify(self.into()),
			co::WM::NULL => Wm::Null(self.into()),
			co::WM::PAINT => Wm::Paint(self.into()),
			co::WM::SETFOCUS => Wm::SetFocus(self.into()),
			co::WM::SIZE => Wm::Size(self.into()),
			co::WM::SIZING => Wm::Sizing(self.into()),
			co::WM::TIMER => Wm::Timer(self.into()),
			m => panic!("Unsupported message: {}.", m),
		}
	}
}