use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::ole::privs::*;
use crate::prelude::*;

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
