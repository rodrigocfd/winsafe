use crate::co::*;

pub(crate) const BCM_FIRST: u32 = 0x1600;
pub(crate) const BCN_FIRST: i32 = -1250;
pub(crate) const CB_FIRST: u32 = 0x1700;
pub(crate) const CCM_FIRST: u32 = 0x2000;
pub(crate) const CLR_DEFAULT: u32 = 0xff00_0000;
pub(crate) const CLR_NONE: u32 = 0xffff_ffff;
pub(crate) const DTM_FIRST: u32 = 0x1000;
pub(crate) const DTN_FIRST: i32 = -740;
pub(crate) const DTN_FIRST2: i32 = -753;
pub(crate) const EM_FIRST: u32 = 0x1500;
pub(crate) const GDT_ERROR: i32 = -1;
pub(crate) const HDM_FIRST: u32 = 0x1200;
pub(crate) const HDN_FIRST: i32 = -300;
pub(crate) const HINST_COMMCTRL: isize = -1;
pub(crate) const I_IMAGECALLBACK: isize = -1;
pub(crate) const I_IMAGENONE: isize = -2;
pub(crate) const IPN_FIRST: i32 = -860;
pub(crate) const L_MAX_URL_LENGTH: usize = 2048 + 32 + 4;
pub(crate) const LVM_FIRST: u32 = 0x1000;
pub(crate) const LVN_FIRST: i32 = -100;
pub(crate) const MAX_LINKID_TEXT: usize = 48;
pub(crate) const MCM_FIRST: u32 = 0x1000;
pub(crate) const MCN_FIRST: i32 = -746;
pub(crate) const NM_FIRST: i32 = 0;
pub(crate) const RBN_FIRST: i32 = -831;
pub(crate) const SBN_FIRST: i32 = -880;
pub(crate) const TBN_FIRST: i32 = -700;
pub(crate) const TCM_FIRST: u32 = 0x1300;
pub(crate) const TCN_FIRST: i32 = -550;
pub(crate) const TRBN_FIRST: i32 = -1501;
pub(crate) const TVM_FIRST: u32 = 0x1100;
pub(crate) const TVN_FIRST: i32 = -400;
pub(crate) const UDN_FIRST: i32 = -721;

const_values_pubcrate! { TDF;
	USE_HICON_FOOTER 0x0004
	USE_HICON_MAIN 0x0002
}
