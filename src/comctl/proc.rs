use crate::co;
use crate::decl::*;

pub(in crate::comctl) extern "system" fn func_task_dialog_callback(
	hwnd: HWND,
	msg: co::TDN,
	wparam: usize,
	lparam: isize,
	lp_ref_data: isize,
) -> co::HRESULT
{
	let tdc = unsafe { &*(lp_ref_data as *const TASKDIALOGCONFIG) };
	tdc.callback.as_ref().map_or(
		co::HRESULT::S_OK,
		|tdc| tdc(&hwnd, unsafe { Tdn::from_msg(msg, wparam, lparam) }),
	)
}
