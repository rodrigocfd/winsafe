use core::panic;

use crate::co;
use crate::msg;

/// Possible
/// [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
pub enum Wm<'a> {
	Close(msg::WmClose),
	Command(msg::WmCommand),
	Create(msg::WmCreate<'a>),
	DropFiles(msg::WmDropFiles),
	InitDialog(msg::WmInitDialog),
	Notify(msg::WmNotify<'a>),
}

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
	pub fn message<'a>(self) -> Wm<'a> {
		match self.msg {
			co::WM::CLOSE => Wm::Close(self.into()),
			co::WM::CREATE => Wm::Create(self.into()),
			co::WM::DROPFILES => Wm::DropFiles(self.into()),
			co::WM::INITDIALOG => Wm::InitDialog(self.into()),
			co::WM::NOTIFY => Wm::Notify(self.into()),
			m => panic!("Unsupported message: {}.", m),
		}
	}
}