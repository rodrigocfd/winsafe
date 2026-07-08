#![allow(non_camel_case_types)]

use crate::macros::*;

const_bitflag! { SND: u32;
	/// [`PlaySound`](crate::PlaySound) `fdwSound`.
	=>
	SYNC 0x0000
	ASYNC 0x0001
	NODEFAULT 0x0002
	MEMORY 0x0004
	LOOP 0x0008
	NOSTOP 0x0010
	NOWAIT 0x0000_2000
	ALIAS 0x0001_0000
	ALIAS_ID 0x0011_0000
	FILENAME 0x0002_0000
	RESOURCE 0x0004_0004
	PURGE 0x0040
	APPLICATION 0x0080
	SENTRY 0x0008_0000
	RING 0x0010_0000
	SYSTEM 0x0020_0000
}

const_str! { SND_ALIAS;
	/// [`PlaySound`](crate::PlaySound) system sound alias.
	=>
	ASTERISK "SystemAsterisk"
	DEFAULT "SystemDefault"
	EXCLAMATION "SystemExclamation"
	EXIT "SystemExit"
	HAND "SystemHand"
	QUESTION "SystemQuestion"
	START "SystemStart"
	WELCOME "SystemWelcome"
}
