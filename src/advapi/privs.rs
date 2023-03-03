use crate::co::ERROR;
use crate::kernel::decl::SysResult;

pub(crate) const SECURITY_DESCRIPTOR_REVISION: u32 = 1;
pub(crate) const UNLEN: usize = 256;

/// If value is `ERROR::SUCCESS`, yields `Ok(())`, otherwise `Err(err)`.
pub(crate) const fn error_to_sysresult(lstatus: i32) -> SysResult<()> {
	match ERROR(lstatus as _) {
		ERROR::SUCCESS => Ok(()),
		err => Err(err),
	}
}
