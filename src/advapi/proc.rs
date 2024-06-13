use crate::decl::*;
use crate::kernel::ffi_types::*;

pub(in crate::advapi) extern "system" fn hservicestatus_register_service_ctrl_handler_ex<F>(
	control: u32,
	event_type: u32,
	event_data: PVOID,
	context: PVOID,
) -> u32
	where F: FnMut(SvcCtl) -> u32,
{
	let func = unsafe { &mut *(context as *mut F) };
	func(unsafe { SvcCtl::from_raw(control, event_type, event_data) })
}
