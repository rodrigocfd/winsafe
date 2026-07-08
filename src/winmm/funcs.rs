#![allow(non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::winmm::ffi;

/// [`PlaySound`](https://learn.microsoft.com/en-us/previous-versions//dd743680(v=vs.85))
/// function.
///
/// # Examples
///
/// Playing directly from a file:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// w::PlaySound(w::Snd::FileAsync {
///     path: "C:\\Temp\\foo.wav",
///     default: true,
///     stop: true,
///     sentry: false,
///     loops: false,
/// })?;
///
/// # w::SysResult::Ok(())
/// ```
///
/// Playing a built-in system sound:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// w::PlaySound(w::Snd::AliasAsync {
///     alias: co::SND_ALIAS::ASTERISK,
///     stop: true,
///     sentry: false,
///     loops: false,
/// })?;
///
/// # w::SysResult::Ok(())
/// ```
pub fn PlaySound(snd: Snd) -> SysResult<()> {
	let mut str_buf = WString::new();
	let (ptr, hinst, flag) = snd.serialize(&mut str_buf);
	BoolRet(unsafe { ffi::PlaySoundW(ptr as _, hinst.ptr(), flag.raw()) }).to_sysresult()
}
