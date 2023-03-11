use crate::advapi::decl::SID;

pub(crate) const SECURITY_DESCRIPTOR_REVISION: u32 = 1;
pub(crate) const SECURITY_MAX_SID_SIZE: u32 = (
	std::mem::size_of::<SID>() -
	std::mem::size_of::<u32>() +
	(SID_MAX_SUB_AUTHORITIES as usize * std::mem::size_of::<u32>())
) as u32;
pub(crate) const SID_MAX_SUB_AUTHORITIES: u32 = 15;
pub(crate) const UNLEN: usize = 256;
