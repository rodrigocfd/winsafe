#![allow(unused)]

use crate::co::*;

const_values_num_privs! {
	CLR_DEFAULT u32 = 0xff00_0000
	CLR_NONE u32 = 0xffff_ffff
	GDT_ERROR i32 = -1
	HINST_COMMCTRL isize = -1
	I_IMAGECALLBACK isize = -1
	I_IMAGENONE isize = -2
	L_MAX_URL_LENGTH usize = 2048 + 32 + 4
	MAX_LINKID_TEXT usize = 48
}

const_values_num_privs! {
	BCM_FIRST u32 = 0x1600
	BCN_FIRST i32 = -1250
	CB_FIRST u32 = 0x1700
	CCM_FIRST u32 = 0x2000
	DTM_FIRST u32 = 0x1000
	DTN_FIRST i32 = -740
	DTN_FIRST2 i32 = -753
	EM_FIRST u32 = 0x1500
	HDM_FIRST u32 = 0x1200
	HDN_FIRST i32 = -300
	IPN_FIRST i32 = -860
	LVM_FIRST u32 = 0x1000
	LVN_FIRST i32 = -100
	MCM_FIRST u32 = 0x1000
	MCN_FIRST i32 = -746
	NM_FIRST i32 = 0
	PSN_FIRST i32 = 200
	RBN_FIRST i32 = -831
	SBN_FIRST i32 = -880
	TBN_FIRST i32 = -700
	TCM_FIRST u32 = 0x1300
	TCN_FIRST i32 = -550
	TRBN_FIRST i32 = -1501
	TVM_FIRST u32 = 0x1100
	TVN_FIRST i32 = -400
	UDN_FIRST i32 = -721
}

const_values_num_privs! {
	TDF_USE_HICON_FOOTER: TDF = 0x0004
	TDF_USE_HICON_MAIN: TDF = 0x0002
}
