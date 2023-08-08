use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::ole::privs::*;
use crate::prelude::*;

/// [`TVM_SETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setextendedstyle)
/// message parameters.
///
/// Return type: `HrResult<()>`.
pub struct SetExtendedStyle {
	pub style: co::TVS_EX,
	pub mask: co::TVS_EX,
}

unsafe impl MsgSend for SetExtendedStyle {
	type RetType = HrResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		ok_to_hrresult(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETEXTENDEDSTYLE.into(),
			wparam: self.style.raw() as _,
			lparam: self.mask.raw() as _,
		}
	}
}
