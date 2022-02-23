use crate::co;
use crate::msg::WndMsg;
use crate::ole::decl::HrResult;
use crate::prelude::MsgSend;

/// [`TVM_SETEXTENDEDSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-setextendedstyle)
/// message parameters.
///
/// Return type: `HrResult<()>`.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
pub struct SetExtendedStyle {
	pub style: co::TVS_EX,
	pub mask: co::TVS_EX,
}

unsafe impl MsgSend for SetExtendedStyle {
	type RetType = HrResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match co::HRESULT(v as _) {
			co::HRESULT::S_OK => Ok(()),
			hr => Err(hr),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETEXTENDEDSTYLE.into(),
			wparam: self.style.0 as _,
			lparam: self.mask.0 as _,
		}
	}
}
