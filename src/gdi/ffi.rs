use crate::ffi_types::{BOOL, HANDLE, PCSTR, PCVOID, PVOID};

extern_sys! { "gdi32";
	AbortPath(HANDLE) -> BOOL
	AngleArc(HANDLE, i32, i32, u32, f32, f32) -> BOOL
	BeginPath(HANDLE) -> BOOL
	BitBlt(HANDLE, i32, i32, i32, i32, HANDLE, i32, i32, u32) -> BOOL
	CancelDC(HANDLE) -> BOOL
	Chord(HANDLE, i32, i32, i32, i32, i32, i32, i32, i32) -> BOOL
	CreateBitmap(i32, i32, u32, u32, PVOID) -> HANDLE
	CreateBrushIndirect(PCVOID) -> HANDLE
	CreateCompatibleBitmap(HANDLE, i32, i32) -> HANDLE
	CreateCompatibleDC(HANDLE) -> HANDLE
	CreateFontIndirectW(PCVOID) -> HANDLE
	CreateFontW(i32, i32, i32, i32, i32, u32, u32, u32, u32, u32, u32, u32, u32, PCSTR) -> HANDLE
	CreateHatchBrush(i32, u32) -> HANDLE
	CreatePatternBrush(HANDLE) -> HANDLE
	CreatePen(i32, i32, u32) -> HANDLE
	CreatePenIndirect(PCVOID) -> HANDLE
	CreateRectRgn(i32, i32, i32, i32) -> HANDLE
	CreateRectRgnIndirect(PVOID) -> HANDLE
	CreateRoundRectRgn(i32, i32, i32, i32, i32, i32) -> HANDLE
	CreateSolidBrush(u32) -> HANDLE
	DeleteDC(HANDLE) -> BOOL
	DeleteObject(HANDLE) -> BOOL
	EndPath(HANDLE) -> BOOL
	FillPath(HANDLE) -> BOOL
	FillRect(HANDLE, PCVOID, HANDLE) -> i32
	GetDCBrushColor(HANDLE) -> u32
	GetDCPenColor(HANDLE) -> u32
	GetDeviceCaps(HANDLE, i32) -> i32
	GetObjectW(HANDLE, i32, PVOID) -> i32
	GetStockObject(i32) -> HANDLE
	GetStretchBltMode(HANDLE) -> i32
	GetSysColorBrush(i32) -> HANDLE
	GetTextColor(HANDLE) -> u32
	GetTextExtentPoint32W(HANDLE, PCSTR, i32, PVOID) -> BOOL
	GetTextMetricsW(HANDLE, PVOID) -> BOOL
	LineTo(HANDLE, i32, i32) -> BOOL
	MoveToEx(HANDLE, i32, i32, PVOID) -> BOOL
	OffsetClipRgn(HANDLE, i32, i32) -> i32
	OffsetRgn(HANDLE, i32, i32) -> i32
	PatBlt(HANDLE, i32, i32, i32, i32, u32) -> BOOL
	PathToRegion(HANDLE) -> HANDLE
	Pie(HANDLE, i32, i32, i32, i32, i32, i32, i32, i32) -> BOOL
	PolyBezier(HANDLE, PCVOID, u32) -> BOOL
	PolyBezierTo(HANDLE, PCVOID, u32) -> BOOL
	Polyline(HANDLE, PCVOID, u32) -> BOOL
	PolylineTo(HANDLE, PCVOID, u32) -> BOOL
	PtInRegion(HANDLE, i32, i32) -> BOOL
	PtVisible(HANDLE, i32, i32) -> BOOL
	Rectangle(HANDLE, i32, i32, i32, i32) -> BOOL
	RectInRegion(HANDLE, PCVOID) -> BOOL
	RestoreDC(HANDLE, i32) -> BOOL
	RoundRect(HANDLE, i32, i32, i32, i32, i32, i32) -> BOOL
	SaveDC(HANDLE) -> i32
	SelectObject(HANDLE, HANDLE) -> HANDLE
	SetArcDirection(HANDLE, i32) -> i32
	SetBkMode(HANDLE, i32) -> i32
	SetBrushOrgEx(HANDLE, i32, i32, PVOID) -> BOOL
	SetDCBrushColor(HANDLE, u32) -> u32
	SetDCPenColor(HANDLE, u32) -> u32
	SetGraphicsMode(HANDLE, i32) -> i32
	SetStretchBltMode(HANDLE, i32) -> i32
	SetTextAlign(HANDLE, u32) -> u32
	SetTextColor(HANDLE, u32) -> u32
	SetTextJustification(HANDLE, i32, i32) -> BOOL
	SetViewportExtEx(HANDLE, i32, i32, PVOID) -> BOOL
	SetViewportOrgEx(HANDLE, i32, i32, PVOID) -> BOOL
	SetWindowExtEx(HANDLE, i32, i32, PVOID) -> BOOL
	SetWindowOrgEx(HANDLE, i32, i32, PVOID) -> BOOL
	StretchBlt(HANDLE, i32, i32, i32, i32, HANDLE, i32, i32, i32, i32, u32) -> BOOL
	StrokeAndFillPath(HANDLE) -> BOOL
	StrokePath(HANDLE) -> BOOL
	TextOutW(HANDLE, i32, i32, PCSTR, i32) -> BOOL
	WidenPath(HANDLE) -> BOOL
}
