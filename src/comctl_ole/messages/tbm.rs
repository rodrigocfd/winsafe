use crate::co;
use crate::msg::WndMsg;
use crate::ole::decl::{HrResult, IDropTarget};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{MsgSend, ole_IDropTarget, ole_IUnknown};

/// [`TB_GETOBJECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getobject)
/// message parameters.
///
/// Return type: `HrResult<()>`.
pub struct GetObject<'a, T>
	where T: ole_IDropTarget,
{
	pub obj: &'a mut T,
}

unsafe impl<'a, T> MsgSend for GetObject<'a, T>
	where T: ole_IDropTarget,
{
	type RetType = HrResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		ok_to_hrresult(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETOBJECT.into(),
			wparam: &IDropTarget::IID as *const _ as _,
			lparam: &self.obj.ptr() as *const _ as _,
		}
	}
}
