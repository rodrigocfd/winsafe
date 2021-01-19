use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{DispatchMessage, GetMessage, TranslateMessage};
use crate::handles::{HACCEL, HWND};
use crate::privs::WM_WINSAFE_ERROR;
use crate::structs::MSG;

pub fn run_loop(hwnd: &HWND, haccel: Option<HACCEL>) -> WinResult<i32> {
	loop {
		let mut msg = MSG::default();
		if !GetMessage(&mut msg, None, 0, 0)? {
			// WM_QUIT was sent, gracefully terminate the program.
			// wParam has the program exit code.
			// https://docs.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues
			return Ok(msg.wParam as i32);
		}

		if msg.message == WM_WINSAFE_ERROR && msg.wParam == 0xc0de_f00d {
			// A WinResult bubbled-up to here.
			// Terminate the program returning the error code passed in lParam.
			return Err(co::ERROR::from(msg.lParam as u32));
		}

		// Does this message belong to a modeless child window (if any)?
		// http://www.winprog.org/tutorial/modeless_dialogs.html





		// If a child window, will retrieve its top-level parent.
		// If a top-level, use itself.
		let hwnd_top_level = msg.hwnd.GetAncestor(co::GA::ROOT)
			.unwrap_or(msg.hwnd);

		// If we have an accelerator table, try to translate the message.
		if let Some(haccel) = haccel {
			if hwnd_top_level.TranslateAccelerator(haccel, &mut msg).is_ok() {
				continue; // message translated
			}
		}

		// Try to process keyboard actions for child controls.
		if hwnd_top_level.IsDialogMessage(&mut msg) {
			continue;
		}

		TranslateMessage(&msg);
		DispatchMessage(&msg);
	}
}
