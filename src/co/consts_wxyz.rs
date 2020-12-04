const_type!(WS, u32,
	"Window
	[styles](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles).");
impl WS {
	const_val!(OVERLAPPED, 0x00000000);
	const_val!(POPUP, 0x80000000);
	const_val!(CHILD, 0x40000000);
	const_val!(MINIMIZE, 0x20000000);
	const_val!(VISIBLE, 0x10000000);
	const_val!(DISABLED, 0x08000000);
	const_val!(CLIPSIBLINGS, 0x04000000);
	const_val!(CLIPCHILDREN, 0x02000000);
	const_val!(MAXIMIZE, 0x01000000);
	const_val!(CAPTION, 0x00c00000);
	const_val!(BORDER, 0x00800000);
	const_val!(DLGFRAME, 0x00400000);
	const_val!(VSCROLL, 0x00200000);
	const_val!(HSCROLL, 0x00100000);
	const_val!(SYSMENU, 0x00080000);
	const_val!(THICKFRAME, 0x00040000);
	const_val!(GROUP, 0x00020000);
	const_val!(TABSTOP, 0x00010000);
	const_val!(MINIMIZEBOX, 0x00020000);
	const_val!(MAXIMIZEBOX, 0x00010000);
	const_val!(TILED, Self::OVERLAPPED.0);
	const_val!(ICONIC, Self::MINIMIZE.0);
	const_val!(SIZEBOX, Self::THICKFRAME.0);
	const_val!(TILEDWINDOW, Self::OVERLAPPEDWINDOW.0);
	const_val!(OVERLAPPEDWINDOW, Self::OVERLAPPED.0 | Self::CAPTION.0 | Self::SYSMENU.0 | Self::THICKFRAME.0 | Self::MINIMIZEBOX.0 | Self::MAXIMIZEBOX.0);
	const_val!(POPUPWINDOW, Self::POPUP.0 | Self::BORDER.0 | Self::SYSMENU.0);
	const_val!(CHILDWINDOW, Self::CHILD.0);
}

const_type!(WS_EX, u32,
	"Extended window
	[styles](https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles).");
impl WS_EX {
	const_val!(DLGMODALFRAME, 0x00000001);
	const_val!(NOPARENTNOTIFY, 0x00000004);
	const_val!(TOPMOST, 0x00000008);
	const_val!(ACCEPTFILES, 0x00000010);
	const_val!(TRANSPARENT, 0x00000020);
	const_val!(MDICHILD, 0x00000040);
	const_val!(TOOLWINDOW, 0x00000080);
	const_val!(WINDOWEDGE, 0x00000100);
	const_val!(CLIENTEDGE, 0x00000200);
	const_val!(CONTEXTHELP, 0x00000400);
	const_val!(RIGHT, 0x00001000);
	const_val!(LEFT, 0x00000000);
	const_val!(RTLREADING, 0x00002000);
	const_val!(LTRREADING, 0x00000000);
	const_val!(LEFTSCROLLBAR, 0x00004000);
	const_val!(RIGHTSCROLLBAR, 0x00000000);
	const_val!(CONTROLPARENT, 0x00010000);
	const_val!(STATICEDGE, 0x00020000);
	const_val!(APPWINDOW, 0x00040000);
	const_val!(OVERLAPPEDWINDOW, Self::WINDOWEDGE.0 | Self::CLIENTEDGE.0);
	const_val!(PALETTEWINDOW, Self::WINDOWEDGE.0 | Self::TOOLWINDOW.0 | Self::TOPMOST.0);
	const_val!(LAYERED, 0x00080000);
	const_val!(NOINHERITLAYOUT, 0x00100000);
	const_val!(NOREDIRECTIONBITMAP, 0x00200000);
	const_val!(LAYOUTRTL, 0x00400000);
	const_val!(COMPOSITED, 0x02000000);
	const_val!(NOACTIVATE, 0x08000000);
}