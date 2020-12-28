/// Result of
/// [message processing](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#lresult).
/// Type returned by [`HWND::SendMessage`](crate::HWND::SendMessage) method.
///
/// This value can be generated only by the `lresult` methods of message
/// parameter structs, like [`WmSize::lresult`](crate::msg::WmSize::lresult).
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::msg::{LResult, Wm, WmAny};
///
/// fn handle_msg(wm_any: WmAny) -> LResult {
///   if let Wm::Size(wm_size) = wm.any_message() {
///     println!("{}", wm_size.width);
///     wm_size.lresult(0) // return value of WM_SIZE
///   } else {
///     wm_any.lresult(0) // return value of generic message
///   }
/// }
/// ```
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LResult(pub(crate) isize);

impl From<LResult> for isize {
	fn from(r: LResult) -> isize {
		r.0 as isize
	}
}