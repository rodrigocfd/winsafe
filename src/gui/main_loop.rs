use crate::co;
use crate::funcs::{DispatchMessage, GetMessage, TranslateMessage};
use crate::handles::{HACCEL, HWND};
use crate::structs::MSG;

pub fn run_loop(hwnd: HWND, haccel: Option<HACCEL>) -> Result<i32, co::ERROR> {
	loop {
		let mut msg = MSG::default();
		if !GetMessage(&mut msg, None, 0, 0)? {
			// WM_QUIT was sent, gracefully terminate the program.
			// wParam has the program exit code.
			// https://docs.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues
			return Ok(msg.wParam as i32);
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
