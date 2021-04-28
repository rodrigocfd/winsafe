//! Progress bar control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-progress-bar-control-reference-messages),
//! whose constants have [`PBM`](crate::co::PBM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{HIWORD, LOWORD, MAKEDWORD};
use crate::msg::{MsgSend, WndMsg};
use crate::structs::PBRANGE;

/// [`PBM_DELTAPOS`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-deltapos)
/// message parameters.
///
/// Return type: `u32`.
pub struct DeltaPos {
	pub advance_amount: u32,
}

impl MsgSend for DeltaPos {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::DELTAPOS.into(),
			wparam: self.advance_amount as _,
			lparam: 0,
		}
	}
}

/// [`PBM_GETPOS`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-getpos)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetPos {}

impl MsgSend for GetPos {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETPOS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_GETRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-getrange)
/// message parameters.
///
/// Return type: `i32`.
pub struct GetRange<'a> {
	pub return_low: bool,
	pub ranges: Option<&'a mut PBRANGE>,
}

impl<'a> MsgSend for GetRange<'a> {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETRANGE.into(),
			wparam: self.return_low as _,
			lparam: self.ranges.as_ref().map_or(0, |r| r as *const _ as _),
		}
	}
}

/// [`PBM_GETSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-getstate)
/// message, which has no parameters.
///
/// Return type: `PBST`.
pub struct GetState {}

impl MsgSend for GetState {
	type RetType = co::PBST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::PBST(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETSTATE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_SETMARQUEE`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setmarquee)
/// message parameters.
///
/// Return type: `()`.
pub struct SetMarquee {
	pub turn_on: bool,
	pub time_ms: Option<u32>,
}

impl MsgSend for SetMarquee {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETMARQUEE.into(),
			wparam: self.turn_on as _,
			lparam: self.time_ms.unwrap_or_default() as _,
		}
	}
}

/// [`PBM_SETPOS`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setpos)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetPos {
	pub position: u32,
}

impl MsgSend for SetPos {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETPOS.into(),
			wparam: self.position as _,
			lparam: 0,
		}
	}
}

/// [`PBM_SETRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setrange)
/// message parameters.
///
/// Return type: `WinResult<(u16, u16)>`.
pub struct SetRange {
	pub min: u16,
	pub max: u16,
}

impl MsgSend for SetRange {
	type RetType = WinResult<(u16, u16)>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			v => Ok((LOWORD(v), HIWORD(v))),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETPOS.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.min, self.max) as _,
		}
	}
}

/// [`PBM_SETRANGE32`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setrange32)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRange32 {
	pub min: u32,
	pub max: u32,
}

impl MsgSend for SetRange32 {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETRANGE32.into(),
			wparam: self.min as _,
			lparam: self.max as _,
		}
	}
}

/// [`PBM_SETSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setstate)
/// message parameters.
///
/// Return type: `PBST`.
pub struct SetState {
	pub state: co::PBST,
}

impl MsgSend for SetState {
	type RetType = co::PBST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::PBST(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETSTATE.into(),
			wparam: self.state.0 as _,
			lparam: 0,
		}
	}
}

/// [`PBM_SETSTEP`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setstep)
/// message parameters.
///
/// Return value: `u32`.
pub struct SetStep {
	pub step: u32,
}

impl MsgSend for SetStep {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETSTEP.into(),
			wparam: self.step as _,
			lparam: 0,
		}
	}
}

/// [`PBM_STEPIT`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-stepit)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct StepIt {}

impl MsgSend for StepIt {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::STEPIT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}
