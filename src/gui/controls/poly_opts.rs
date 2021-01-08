/// Polymorphic options of a native child control to be created.
pub enum PolyOpts<COpt> {
	/// The control will be created with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	Wnd(COpt),
	/// The control belongs to a dialog and will be attached with
	/// [`GetDlgItem`](crate::HWND::GetDlgItem).
	Dlg(u16),
}
