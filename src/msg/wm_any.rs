use core::panic;

use crate::co;
use crate::msg;

/// Possible
/// [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
pub enum Wm<'a> {
	Close(msg::WmClose),
	Create(msg::WmCreate<'a>),
	InitDialog(msg::WmInitDialog),
	Notify(msg::WmNotify<'a>),
}

/// Generic
/// [window message](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues)
/// parameters.
pub struct WmAny {
	pub msg: co::WM,
	pub wparam: usize,
	pub lparam: isize,
}

impl WmAny {
	pub fn message<'a>(self) -> Wm<'a> {
		match self.msg {
			co::WM::CLOSE => Wm::Close(self.into()),
			co::WM::CREATE => Wm::Create(self.into()),
			co::WM::INITDIALOG => Wm::InitDialog(self.into()),
			co::WM::NOTIFY => Wm::Notify(self.into()),
			m => panic!("Unsupported message: {}.", m),
		}
	}
}