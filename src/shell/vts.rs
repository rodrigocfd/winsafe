#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::macros::*;
use crate::ole::vts::*;

com_vtbl! { IEnumShellItemsVT : IUnknownVT
	Next(u32, *mut COMPTR, *mut u32) -> HRES
	Skip(u32) -> HRES
	Reset() -> HRES
	Clone(*mut COMPTR) -> HRES
}

com_vtbl! { IFileDialogVT : IModalWindowVT
	SetFileTypes(u32, PCVOID) -> HRES
	SetFileTypeIndex(u32) -> HRES
	GetFileTypeIndex(*mut u32) -> HRES
	Advise(PVOID, *mut u32) -> HRES
	Unadvise(u32) -> HRES
	SetOptions(u32) -> HRES
	GetOptions(*mut u32) -> HRES
	SetDefaultFolder(COMPTR) -> HRES
	SetFolder(COMPTR) -> HRES
	GetFolder(*mut COMPTR) -> HRES
	GetCurrentSelection(*mut COMPTR) -> HRES
	SetFileName(PCSTR) -> HRES
	GetFileName(*mut PSTR) -> HRES
	SetTitle(PCSTR) -> HRES
	SetOkButtonLabel(PCSTR) -> HRES
	SetFileNameLabel(PCSTR) -> HRES
	GetResult(*mut COMPTR) -> HRES
	AddPlace(COMPTR, u32) -> HRES
	SetDefaultExtension(PCSTR) -> HRES
	Close(HRES) -> HRES
	SetClientGuid(PCVOID) -> HRES
	ClearClientData() -> HRES
	SetFilter(PVOID) -> HRES
}

com_vtbl! { IFileDialogEventsVT : IUnknownVT
	OnFileOk(COMPTR) -> HRES
	OnFolderChanging(COMPTR, COMPTR) -> HRES
	OnFolderChange(COMPTR) -> HRES
	OnSelectionChange(COMPTR) -> HRES
	OnShareViolation(COMPTR, COMPTR, *mut u32) -> HRES
	OnTypeChange(COMPTR) -> HRES
	OnOverwrite(COMPTR, COMPTR, *mut u32) -> HRES
}

com_vtbl! { IFileOpenDialogVT : IFileDialogVT
	GetResults(*mut COMPTR) -> HRES
	GetSelectedItems(*mut COMPTR) -> HRES
}

com_vtbl! { IFileOperationProgressSinkVT : IUnknownVT
	StartOperations() -> HRES
	FinishOperations(HRES) -> HRES
	PreRenameItem(u32, COMPTR, PCSTR) -> HRES
	PostRenameItem(u32, COMPTR, PCSTR, HRES, COMPTR) -> HRES
	PreMoveItem(u32, COMPTR, COMPTR, PCSTR) -> HRES
	PostMoveItem(u32, COMPTR, COMPTR, PCSTR, HRES, COMPTR) -> HRES
	PreCopyItem(u32, COMPTR, COMPTR, PCSTR) -> HRES
	PostCopyItem(u32, COMPTR, COMPTR, PCSTR, HRES, COMPTR) -> HRES
	PreDeleteItem(u32, COMPTR) -> HRES
	PostDeleteItem(u32, COMPTR, HRES, COMPTR) -> HRES
	PreNewItem(u32, COMPTR, PCSTR) -> HRES
	PostNewItem(u32, COMPTR, PCSTR, PCSTR, u32, HRES, COMPTR) -> HRES
	UpdateProgress(u32, u32) -> HRES
	ResetTimer() -> HRES
	PauseTimer() -> HRES
	ResumeTimer() -> HRES
}

com_vtbl! { IFileOperationVT : IUnknownVT
	Advise(COMPTR, *mut u32) -> HRES
	Unadvise(u32) -> HRES
	SetOperationFlags(u32) -> HRES
	SetProgressMessage(PCSTR) -> HRES
	SetProgressDialog(COMPTR) -> HRES
	SetProperties(COMPTR) -> HRES
	SetOwnerWindow(HANDLE) -> HRES
	ApplyPropertiesToItem(COMPTR) -> HRES
	ApplyPropertiesToItems(COMPTR) -> HRES
	RenameItem(COMPTR, PCSTR, COMPTR) -> HRES
	RenameItems(COMPTR, PCSTR) -> HRES
	MoveItem(COMPTR, COMPTR, PCSTR, COMPTR) -> HRES
	MoveItems(COMPTR, COMPTR) -> HRES
	CopyItem(COMPTR, COMPTR, PCSTR, COMPTR) -> HRES
	CopyItems(COMPTR, COMPTR) -> HRES
	DeleteItem(COMPTR, COMPTR) -> HRES
	DeleteItems(COMPTR) -> HRES
	NewItem(COMPTR, u32, PCSTR, PCSTR, COMPTR) -> HRES
	PerformOperations() -> HRES
	GetAnyOperationsAborted(*mut BOOL) -> HRES
}

com_vtbl! { IFileSaveDialogVT : IFileDialogVT
	SetSaveAsItem(COMPTR) -> HRES
	SetProperties(COMPTR) -> HRES
	SetCollectedProperties(COMPTR, BOOL) -> HRES
	GetProperties(*mut COMPTR) -> HRES
	ApplyProperties(COMPTR, COMPTR, HANDLE, COMPTR) -> HRES
}

com_vtbl! { IModalWindowVT : IUnknownVT
	Show(HANDLE) -> u32
}

com_vtbl! { IOperationsProgressDialogVT : IUnknownVT
	StartProgressDialog(HANDLE, u32) -> HRES
	StopProgressDialog() -> HRES
	SetOperation(u32) -> HRES
	SetMode(u32) -> HRES
	UpdateProgress(u64, u64, u64, u64, u64, u64) -> HRES
	UpdateLocations(COMPTR, COMPTR, COMPTR) -> HRES
	ResetTimer() -> HRES
	PauseTimer() -> HRES
	ResumeTimer() -> HRES
	GetMilliseconds(*mut u64, *mut u64) -> HRES
	GetOperationStatus(*mut u32) -> HRES
}

com_vtbl! { IShellFolderVT : IUnknownVT
	ParseDisplayName(HANDLE, COMPTR, PCSTR, *mut u32, PCVOID, *mut u32) -> HRES
	EnumObjects(HANDLE, u32, *mut COMPTR) -> HRES
	BindToObject(PCVOID, COMPTR, PCVOID, *mut COMPTR) -> HRES
	BindToStorage(PCVOID, COMPTR, PCVOID, *mut COMPTR) -> HRES
	CompareIDs(isize, PCVOID, PCVOID) -> HRES
	CreateViewObject(HANDLE, PCVOID, *mut COMPTR) -> HRES
	GetAttributesOf(u32, PCVOID, *mut u32) -> HRES
	GetUIObjectOf(HANDLE, u32, PCVOID, PCVOID, *mut u32, *mut COMPTR) -> HRES
	GetDisplayNameOf(PCVOID, u32, PVOID) -> HRES
	SetNameOf(HANDLE, PCVOID, PCSTR, u32, PVOID) -> HRES
}

com_vtbl! { IShellItemFilterVT : IUnknownVT
	IncludeItem(COMPTR) -> HRES
	GetEnumFlagsForItem(COMPTR, *mut u32) -> HRES
}

com_vtbl! { IShellItemVT : IUnknownVT
	BindToHandler(PVOID, PCVOID, PCVOID, *mut COMPTR) -> HRES
	GetParent(*mut COMPTR) -> HRES
	GetDisplayName(u32, *mut PSTR) -> HRES
	GetAttributes(u32, *mut u32) -> HRES
	Compare(PVOID, u32, *mut i32) -> HRES
}

com_vtbl! { IShellItem2VT : IShellItemVT
	GetPropertyStore(u32, PCVOID, *mut COMPTR) -> HRES
	GetPropertyStoreWithCreateObject(u32, COMPTR, PCVOID, *mut COMPTR) -> HRES
	GetPropertyStoreForKeys(PCVOID, u32, u32, PCVOID, *mut COMPTR) -> HRES
	GetPropertyDescriptionList(PCVOID, PCVOID, *mut COMPTR) -> HRES
	Update(COMPTR) -> HRES
	GetProperty(PCVOID, PVOID) -> HRES
	GetCLSID(PCVOID, PVOID) -> HRES
	GetFileTime(PCVOID, PVOID) -> HRES
	GetInt32(PCVOID, *mut i32) -> HRES
	GetString(PCVOID, *mut PSTR) -> HRES
	GetUInt32(PCVOID, *mut u32) -> HRES
	GetUInt64(PCVOID, *mut u64) -> HRES
	GetBool(PCVOID, *mut BOOL) -> HRES
}

com_vtbl! { IShellItemArrayVT : IUnknownVT
	BindToHandler(PVOID, PCVOID, PCVOID, *mut COMPTR) -> HRES
	GetPropertyStore(u32, PCVOID, *mut COMPTR) -> HRES
	GetPropertyDescriptionList(PVOID, PCVOID, *mut COMPTR) -> HRES
	GetAttributes(u32, u32, PVOID) -> HRES
	GetCount(*mut u32) -> HRES
	GetItemAt(u32, *mut COMPTR) -> HRES
	EnumItems(*mut PVOID) -> HRES
}

com_vtbl! { IShellLinkVT : IUnknownVT
	GetPath(PCSTR, i32, PVOID, u32) -> HRES
	GetIDList(PVOID) -> HRES
	SetIDList(PVOID) -> HRES
	GetDescription(PSTR, i32) -> HRES
	SetDescription(PCSTR) -> HRES
	GetWorkingDirectory(PSTR, i32) -> HRES
	SetWorkingDirectory(PCSTR) -> HRES
	GetArguments(PSTR, i32) -> HRES
	SetArguments(PCSTR) -> HRES
	GetHotkey(*mut u16) -> HRES
	SetHotkey(u16) -> HRES
	GetShowCmd(*mut i32) -> HRES
	SetShowCmd(i32) -> HRES
	GetIconLocation(PSTR, i32, *mut i32) -> HRES
	SetIconLocation(PCSTR, i32) -> HRES
	SetRelativePath(PCSTR, u32) -> HRES
	Resolve(HANDLE, u32) -> HRES
	SetPath(PCSTR) -> HRES
}

com_vtbl! { ITaskbarListVT : IUnknownVT
	HrInit() -> HRES
	AddTab(HANDLE) -> HRES
	DeleteTab(HANDLE) -> HRES
	ActivateTab(HANDLE) -> HRES
	SetActiveAlt(HANDLE) -> HRES
}

com_vtbl! { ITaskbarList2VT : ITaskbarListVT
	MarkFullscreenWindow(HANDLE, BOOL) -> HRES
}

com_vtbl! { ITaskbarList3VT : ITaskbarList2VT
	SetProgressValue(HANDLE, u64, u64) -> HRES
	SetProgressState(HANDLE, u32) -> HRES
	RegisterTab(HANDLE, HANDLE) -> HRES
	UnregisterTab(HANDLE) -> HRES
	SetTabOrder(HANDLE, HANDLE) -> HRES
	SetTabActive(HANDLE, HANDLE, u32) -> HRES
	ThumbBarAddButtons(HANDLE, u32, PVOID) -> HRES
	ThumbBarUpdateButtons(HANDLE, u32, PVOID) -> HRES
	ThumbBarSetImageList(HANDLE, HANDLE) -> HRES
	SetOverlayIcon(HANDLE, HANDLE, PCSTR) -> HRES
	SetThumbnailTooltip(HANDLE, PCSTR) -> HRES
	SetThumbnailClip(HANDLE, PCVOID) -> HRES
}

com_vtbl! { ITaskbarList4VT : ITaskbarList3VT
	SetTabProperties(HANDLE, u32) -> HRES
}
