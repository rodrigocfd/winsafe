#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::user::{callbacks, ffi};

handle! { HDC;
	/// Handle to a
	/// [device context](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc).
}

impl HDC {
	/// [`DrawFocusRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawfocusrect)
	/// function.
	pub fn DrawFocusRect(&self, rect: RECT) -> SysResult<()> {
		BoolRet(unsafe { ffi::DrawFocusRect(self.ptr(), pcvoid(&rect)) }).to_sysresult()
	}

	/// [`DrawText`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawtextw)
	/// function.
	pub fn DrawText(&self, text: &str, bounds: RECT, format: co::DT) -> SysResult<i32> {
		let mut bounds = bounds;
		let wtext = WString::from_str(text);

		match unsafe {
			ffi::DrawTextW(
				self.ptr(),
				wtext.as_ptr(),
				wtext.str_len() as _,
				pvoid(&mut bounds),
				format.raw(),
			)
		} {
			0 => Err(GetLastError()),
			i => Ok(i),
		}
	}

	/// [`DrawTextExW`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawtextexw)
	/// function.
	pub fn DrawTextEx(
		&self,
		text: &str,
		bounds: RECT,
		format: co::DT,
		dtp: Option<&DRAWTEXTPARAMS>,
	) -> SysResult<i32> {
		let mut bounds = bounds;
		let wtext = WString::from_str(text);

		match unsafe {
			ffi::DrawTextExW(
				self.ptr(),
				wtext.as_ptr(),
				wtext.str_len() as _,
				pvoid(&mut bounds),
				format.raw(),
				pcvoid_or_null(dtp),
			)
		} {
			0 => Err(GetLastError()),
			i => Ok(i),
		}
	}

	/// [`EnumDisplayMonitors`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaymonitors)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hdc: w::HDC; // initialized somewhere
	/// # let hdc = w::HDC::NULL;
	///
	/// hdc.EnumDisplayMonitors(
	///     None,
	///     |hmon: w::HMONITOR, hdc: w::HDC, rc: &w::RECT| -> bool {
	///         println!("HMONITOR: {}, ", hmon);
	///         true
	///     },
	/// )?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn EnumDisplayMonitors<F>(&self, rc_clip: Option<RECT>, func: F) -> SysResult<()>
	where
		F: FnMut(HMONITOR, HDC, &RECT) -> bool,
	{
		BoolRet(unsafe {
			ffi::EnumDisplayMonitors(
				self.ptr(),
				pcvoid_or_null(rc_clip.as_ref()),
				callbacks::hdc_enum_display_monitors::<F> as _,
				pcvoid(&func),
			)
		})
		.to_sysresult()
	}

	/// [`ExcludeUpdateRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-excludeupdatergn)
	/// function.
	pub fn ExcludeUpdateRgn(&self, hwnd: &HWND) -> SysResult<co::REGION> {
		match unsafe { ffi::ExcludeUpdateRgn(self.ptr(), hwnd.ptr()) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			v => Ok(unsafe { co::REGION::from_raw(v) }),
		}
	}

	/// [`FrameRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-framerect)
	/// function.
	pub fn FrameRect(&self, rc: RECT, hbr: &HBRUSH) -> SysResult<()> {
		BoolRet(unsafe { ffi::FrameRect(self.ptr(), pcvoid(&rc), hbr.ptr()) }).to_sysresult()
	}

	/// [`InvertRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invertrect)
	/// function.
	pub fn InvertRect(&self, rc: RECT) -> SysResult<()> {
		BoolRet(unsafe { ffi::InvertRect(self.ptr(), pcvoid(&rc)) }).to_sysresult()
	}

	/// [`PaintDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-paintdesktop)
	/// function.
	pub fn PaintDesktop(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::PaintDesktop(self.ptr()) }).to_sysresult()
	}

	/// [`WindowFromDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfromdc)
	/// function.
	#[must_use]
	pub fn WindowFromDC(&self) -> Option<HWND> {
		PtrRet(unsafe { ffi::WindowFromDC(self.ptr()) }).to_opt_handle()
	}
}
