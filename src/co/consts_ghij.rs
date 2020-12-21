const_type! { GA, u32,
	/// [`GetAncestor`](crate::HWND::GetAncestor) `gaFlags`.

	PARENT, 1
	ROOOT, 2
	ROOTOWNER, 3
}

const_type! { GW, u32,
	/// [`GetWindow`](crate::HWND::GetWindow) `uCmd`.

	HWNDFIRST, 0
	HWNDLAST, 1
	HWNDNEXT, 2
	HWNDPREV, 3
	OWNER, 4
	CHILD, 5
	ENABLEDPOPUP, 6
	MAX, 6
}

const_type! { GWLP, i32,
	/// [`GetWindowLongPtr`](crate::HWND::GetWindowLongPtr) and
	/// [`SetWindowLongPtr`](crate:HWND::SetWindowLongPtr) `nIndex`.

	STYLE, -16
	EXSTYLE, -20
	WNDPROC, -4
	HINSTANCE, -6
	HWNDPARENT, -8
	USERDATA, -21
	ID, -12
	DWLP_DLGPROC, 8 //std::mem::size_of::<isize> as i32 https://github.com/rust-lang/rust/issues/51910
	GWLP_DWLP_MSGRESULT, 0
	GWLP_DWLP_USER, Self::DWLP_DLGPROC.0 + 8 //std::mem::size_of::<isize> as i32 https://github.com/rust-lang/rust/issues/51910
}

const_type! { HICF, u32,
	/// [NMBCHOTITEM](crate::NMBCHOTITEM) `dwFlags`.

	OTHER, 0x00000000
	ARROWKEYS, 0x00000002
	ACCELERATOR, 0x00000004
	DUPACCEL, 0x00000008
	ENTERING, 0x00000010
	LEAVING, 0x00000020
	RESELECT, 0x00000040
	LMOUSE, 0x00000080
	TOGGLEDROPDOWN, 0x00000100
}

const_type! { IDC, usize,
	/// [`LoadCursor`](crate::HINSTANCE::LoadCursor) `lpCursorName`.

	ARROW, 32512
	IBEAM, 32513
	WAIT, 32514
	CROSS, 32515
	UPARROW, 32516
	SIZENWSE, 32642
	SIZENESW, 32643
	SIZEWE, 32644
	SIZENS, 32645
	SIZEALL, 32646
	NO, 32648
	HAND, 32649
	APPSTARTING, 32650
	HELP, 32651
	PIN, 32671
	PERSON, 32672
}

const_type! { IDI, usize,
	/// [`LoadIcon`](crate::HINSTANCE::LoadIcon) `lpIconName`.

	APPLICATION, 32512
	HAND, 32513
	QUESTION, 32514
	EXCLAMATION, 32515
	ASTERISK, 32516
	WINLOGO, 32517
	SHIELD, 32518
	WARNING, Self::EXCLAMATION.0
	ERROR, Self::HAND.0
	INFORMATION, Self::ASTERISK.0
}