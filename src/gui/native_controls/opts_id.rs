/// Variant field for child controls: options or just a control ID.
pub enum OptsId<Op> {
	/// The control will be created with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	Wnd(Op),
	/// The control belongs to a dialog and will be attached with
	/// [`GetDlgItem`](crate::HWND::GetDlgItem).
	Dlg(u16),
}
