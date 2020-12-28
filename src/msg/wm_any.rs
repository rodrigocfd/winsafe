use crate::co::WM;
use crate::msg;

/// Possible
/// [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues),
/// and their input parameters.
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
	pub msg: WM,
	pub wparam: usize,
	pub lparam: isize,
}

impl WmAny {
	/// Generates the message result value.
	pub fn lresult(&self, val: isize) -> msg::LResult {
		msg::LResult(val)
	}

	/// Returns a [`Wm`](crate::msg::Wm) enum, which can be matched to identify
	/// the exact message type.
	pub fn message<'a, 'b, 'c>(self) -> Wm<'a, 'b, 'c> {
		match self.msg {
			WM::ACTIVATE => Wm::Activate(self.into()),
			WM::ACTIVATEAPP => Wm::ActivateApp(self.into()),
			WM::APPCOMMAND => Wm::AppCommand(self.into()),
			WM::CLOSE => Wm::Close(self.into()),
			WM::CREATE => Wm::Create(self.into()),
			WM::CTLCOLORBTN => Wm::CtlColorBtn(self.into()),
			WM::CTLCOLORDLG => Wm::CtlColorDlg(self.into()),
			WM::CTLCOLOREDIT => Wm::CtlColorEdit(self.into()),
			WM::CTLCOLORLISTBOX => Wm::CtlColorListBox(self.into()),
			WM::CTLCOLORSCROLLBAR => Wm::CtlColorListScrollBar(self.into()),
			WM::CTLCOLORSTATIC => Wm::CtlColorListStatic(self.into()),
			WM::DESTROY => Wm::Destroy(self.into()),
			WM::DROPFILES => Wm::DropFiles(self.into()),
			WM::ENDSESSION => Wm::EndSession(self.into()),
			WM::INITDIALOG => Wm::InitDialog(self.into()),
			WM::INITMENUPOPUP => Wm::InitMenuPopup(self.into()),
			WM::NCCREATE => Wm::NcCreate(self.into()),
			WM::NCDESTROY => Wm::NcDestroy(self.into()),
			WM::NCPAINT => Wm::NcPaint(self.into()),
			WM::NOTIFY => Wm::Notify(self.into()),
			WM::NULL => Wm::Null(self.into()),
			WM::PAINT => Wm::Paint(self.into()),
			WM::SETFOCUS => Wm::SetFocus(self.into()),
			WM::SIZE => Wm::Size(self.into()),
			WM::SIZING => Wm::Sizing(self.into()),
			WM::TIMER => Wm::Timer(self.into()),
			m => panic!("Unsupported message: {}.", m),
		}
	}
}