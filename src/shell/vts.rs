#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::ole::vts::*;

#[repr(C)]
pub struct IEnumShellItemsVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(COMPTR, u32, *mut COMPTR, *mut u32) -> HRES,
	pub Skip: fn(COMPTR, u32) -> HRES,
	pub Reset: fn(COMPTR) -> HRES,
	pub Clone: fn(COMPTR, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IFileDialogVT {
	pub IModalWindowVT: IModalWindowVT,
	pub SetFileTypes: fn(COMPTR, u32, PCVOID) -> HRES,
	pub SetFileTypeIndex: fn(COMPTR, u32) -> HRES,
	pub GetFileTypeIndex: fn(COMPTR, *mut u32) -> HRES,
	pub Advise: fn(COMPTR, PVOID, *mut u32) -> HRES,
	pub Unadvise: fn(COMPTR, u32) -> HRES,
	pub SetOptions: fn(COMPTR, u32) -> HRES,
	pub GetOptions: fn(COMPTR, *mut u32) -> HRES,
	pub SetDefaultFolder: fn(COMPTR, COMPTR) -> HRES,
	pub SetFolder: fn(COMPTR, COMPTR) -> HRES,
	pub GetFolder: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetCurrentSelection: fn(COMPTR, *mut COMPTR) -> HRES,
	pub SetFileName: fn(COMPTR, PCSTR) -> HRES,
	pub GetFileName: fn(COMPTR, *mut PSTR) -> HRES,
	pub SetTitle: fn(COMPTR, PCSTR) -> HRES,
	pub SetOkButtonLabel: fn(COMPTR, PCSTR) -> HRES,
	pub SetFileNameLabel: fn(COMPTR, PCSTR) -> HRES,
	pub GetResult: fn(COMPTR, *mut COMPTR) -> HRES,
	pub AddPlace: fn(COMPTR, COMPTR, u32) -> HRES,
	pub SetDefaultExtension: fn(COMPTR, PCSTR) -> HRES,
	pub Close: fn(COMPTR, HRES) -> HRES,
	pub SetClientGuid: fn(COMPTR, PCVOID) -> HRES,
	pub ClearClientData: fn(COMPTR) -> HRES,
	pub SetFilter: fn(COMPTR, PVOID) -> HRES,
}

#[repr(C)]
pub struct IFileDialogEventsVT {
	pub IUnknownVT: IUnknownVT,
	pub OnFileOk: fn(COMPTR, COMPTR) -> HRES,
	pub OnFolderChanging: fn(COMPTR, COMPTR, COMPTR) -> HRES,
	pub OnFolderChange: fn(COMPTR, COMPTR) -> HRES,
	pub OnSelectionChange: fn(COMPTR, COMPTR) -> HRES,
	pub OnShareViolation: fn(COMPTR, COMPTR, COMPTR, *mut u32) -> HRES,
	pub OnTypeChange: fn(COMPTR, COMPTR) -> HRES,
	pub OnOverwrite: fn(COMPTR, COMPTR, COMPTR, *mut u32) -> HRES,
}

#[repr(C)]
pub struct IFileOpenDialogVT {
	pub IFileDialogVT: IFileDialogVT,
	pub GetResults: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetSelectedItems: fn(COMPTR, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IFileOperationProgressSinkVT {
	pub IUnknownVT: IUnknownVT,
	pub StartOperations: fn(COMPTR) -> HRES,
	pub FinishOperations: fn(COMPTR, HRES) -> HRES,
	pub PreRenameItem: fn(COMPTR, u32, COMPTR, PCSTR) -> HRES,
	pub PostRenameItem: fn(COMPTR, u32, COMPTR, PCSTR, HRES, COMPTR) -> HRES,
	pub PreMoveItem: fn(COMPTR, u32, COMPTR, COMPTR, PCSTR) -> HRES,
	pub PostMoveItem: fn(COMPTR, u32, COMPTR, COMPTR, PCSTR, HRES, COMPTR) -> HRES,
	pub PreCopyItem: fn(COMPTR, u32, COMPTR, COMPTR, PCSTR) -> HRES,
	pub PostCopyItem: fn(COMPTR, u32, COMPTR, COMPTR, PCSTR, HRES, COMPTR) -> HRES,
	pub PreDeleteItem: fn(COMPTR, u32, COMPTR) -> HRES,
	pub PostDeleteItem: fn(COMPTR, u32, COMPTR, HRES, COMPTR) -> HRES,
	pub PreNewItem: fn(COMPTR, u32, COMPTR, PCSTR) -> HRES,
	pub PostNewItem: fn(COMPTR, u32, COMPTR, PCSTR, PCSTR, u32, HRES, COMPTR) -> HRES,
	pub UpdateProgress: fn(COMPTR, u32, u32) -> HRES,
	pub ResetTimer: fn(COMPTR) -> HRES,
	pub PauseTimer: fn(COMPTR) -> HRES,
	pub ResumeTimer: fn(COMPTR) -> HRES,
}

#[repr(C)]
pub struct IFileOperationVT {
	pub IUnknownVT: IUnknownVT,
	pub Advise: fn(COMPTR, COMPTR, *mut u32) -> HRES,
	pub Unadvise: fn(COMPTR, u32) -> HRES,
	pub SetOperationFlags: fn(COMPTR, u32) -> HRES,
	pub SetProgressMessage: fn(COMPTR, PCSTR) -> HRES,
	pub SetProgressDialog: fn(COMPTR, COMPTR) -> HRES,
	pub SetProperties: fn(COMPTR, COMPTR) -> HRES,
	pub SetOwnerWindow: fn(COMPTR, HANDLE) -> HRES,
	pub ApplyPropertiesToItem: fn(COMPTR, COMPTR) -> HRES,
	pub ApplyPropertiesToItems: fn(COMPTR, COMPTR) -> HRES,
	pub RenameItem: fn(COMPTR, COMPTR, PCSTR, COMPTR) -> HRES,
	pub RenameItems: fn(COMPTR, COMPTR, PCSTR) -> HRES,
	pub MoveItem: fn(COMPTR, COMPTR, COMPTR, PCSTR, COMPTR) -> HRES,
	pub MoveItems: fn(COMPTR, COMPTR, COMPTR) -> HRES,
	pub CopyItem: fn(COMPTR, COMPTR, COMPTR, PCSTR, COMPTR) -> HRES,
	pub CopyItems: fn(COMPTR, COMPTR, COMPTR) -> HRES,
	pub DeleteItem: fn(COMPTR, COMPTR, COMPTR) -> HRES,
	pub DeleteItems: fn(COMPTR, COMPTR) -> HRES,
	pub NewItem: fn(COMPTR, COMPTR, u32, PCSTR, PCSTR, COMPTR) -> HRES,
	pub PerformOperations: fn(COMPTR) -> HRES,
	pub GetAnyOperationsAborted: fn(COMPTR, *mut BOOL) -> HRES,
}

#[repr(C)]
pub struct IFileSaveDialogVT {
	pub IFileDialogVT: IFileDialogVT,
	pub SetSaveAsItem: fn(COMPTR, COMPTR) -> HRES,
	pub SetProperties: fn(COMPTR, COMPTR) -> HRES,
	pub SetCollectedProperties: fn(COMPTR, COMPTR, BOOL) -> HRES,
	pub GetProperties: fn(COMPTR, *mut COMPTR) -> HRES,
	pub ApplyProperties: fn(COMPTR, COMPTR, COMPTR, HANDLE, COMPTR) -> HRES,
}

#[repr(C)]
pub struct IModalWindowVT {
	pub IUnknownVT: IUnknownVT,
	pub Show: fn(COMPTR, HANDLE) -> u32,
}

#[repr(C)]
pub struct IShellFolderVT {
	pub IUnknownVT: IUnknownVT,
	pub ParseDisplayName: fn(COMPTR, HANDLE, COMPTR, PCSTR, *mut u32, PCVOID, *mut u32) -> HRES,
	pub EnumObjects: fn(COMPTR, HANDLE, u32, *mut COMPTR) -> HRES,
	pub BindToObject: fn(COMPTR, PCVOID, COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub BindToStorage: fn(COMPTR, PCVOID, COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub CompareIDs: fn(COMPTR, isize, PCVOID, PCVOID) -> HRES,
	pub CreateViewObject: fn(COMPTR, HANDLE, PCVOID, *mut COMPTR) -> HRES,
	pub GetAttributesOf: fn(COMPTR, u32, PCVOID, *mut u32) -> HRES,
	pub GetUIObjectOf: fn(COMPTR, HANDLE, u32, PCVOID, PCVOID, *mut u32, *mut COMPTR) -> HRES,
	pub GetDisplayNameOf: fn(COMPTR, PCVOID, u32, PVOID) -> HRES,
	pub SetNameOf: fn(COMPTR, HANDLE, PCVOID, PCSTR, u32, PVOID) -> HRES,
}

#[repr(C)]
pub struct IShellItemFilterVT {
	pub IUnknownVT: IUnknownVT,
	pub IncludeItem: fn(COMPTR, COMPTR) -> HRES,
	pub GetEnumFlagsForItem: fn(COMPTR, COMPTR, *mut u32) -> HRES,
}

#[repr(C)]
pub struct IShellItemVT {
	pub IUnknownVT: IUnknownVT,
	pub BindToHandler: fn(COMPTR, PVOID, PCVOID, PCVOID, *mut COMPTR) -> HRES,
	pub GetParent: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetDisplayName: fn(COMPTR, u32, *mut PSTR) -> HRES,
	pub GetAttributes: fn(COMPTR, u32, *mut u32) -> HRES,
	pub Compare: fn(COMPTR, PVOID, u32, *mut i32) -> HRES,
}

#[repr(C)]
pub struct IShellItem2VT {
	pub IShellItemVT: IShellItemVT,
	pub GetPropertyStore: fn(COMPTR, u32, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyStoreWithCreateObject: fn(COMPTR, u32, COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyStoreForKeys: fn(COMPTR, PCVOID, u32, u32, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyDescriptionList: fn(COMPTR, PCVOID, PCVOID, *mut COMPTR) -> HRES,
	pub Update: fn(COMPTR, COMPTR) -> HRES,
	pub GetProperty: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub GetCLSID: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub GetFileTime: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub GetInt32: fn(COMPTR, PCVOID, *mut i32) -> HRES,
	pub GetString: fn(COMPTR, PCVOID, *mut PSTR) -> HRES,
	pub GetUInt32: fn(COMPTR, PCVOID, *mut u32) -> HRES,
	pub GetUInt64: fn(COMPTR, PCVOID, *mut u64) -> HRES,
	pub GetBool: fn(COMPTR, PCVOID, *mut BOOL) -> HRES,
}

#[repr(C)]
pub struct IShellItemArrayVT {
	pub IUnknownVT: IUnknownVT,
	pub BindToHandler: fn(COMPTR, PVOID, PCVOID, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyStore: fn(COMPTR, u32, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyDescriptionList: fn(COMPTR, PVOID, PCVOID, *mut COMPTR) -> HRES,
	pub GetAttributes: fn(COMPTR, u32, u32, PVOID) -> HRES,
	pub GetCount: fn(COMPTR, *mut u32) -> HRES,
	pub GetItemAt: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub EnumItems: fn(COMPTR, *mut PVOID) -> HRES,
}

#[repr(C)]
pub struct IShellLinkVT {
	pub IUnknownVT: IUnknownVT,
	pub GetPath: fn(COMPTR, PCSTR, i32, PVOID, u32) -> HRES,
	pub GetIDList: fn(COMPTR, PVOID) -> HRES,
	pub SetIDList: fn(COMPTR, PVOID) -> HRES,
	pub GetDescription: fn(COMPTR, PSTR, i32) -> HRES,
	pub SetDescription: fn(COMPTR, PCSTR) -> HRES,
	pub GetWorkingDirectory: fn(COMPTR, PSTR, i32) -> HRES,
	pub SetWorkingDirectory: fn(COMPTR, PCSTR) -> HRES,
	pub GetArguments: fn(COMPTR, PSTR, i32) -> HRES,
	pub SetArguments: fn(COMPTR, PCSTR) -> HRES,
	pub GetHotkey: fn(COMPTR, *mut u16) -> HRES,
	pub SetHotkey: fn(COMPTR, u16) -> HRES,
	pub GetShowCmd: fn(COMPTR, *mut i32) -> HRES,
	pub SetShowCmd: fn(COMPTR, i32) -> HRES,
	pub GetIconLocation: fn(COMPTR, PSTR, i32, *mut i32) -> HRES,
	pub SetIconLocation: fn(COMPTR, PCSTR, i32) -> HRES,
	pub SetRelativePath: fn(COMPTR, PCSTR, u32) -> HRES,
	pub Resolve: fn(COMPTR, HANDLE, u32) -> HRES,
	pub SetPath: fn(COMPTR, PCSTR) -> HRES,
}

#[repr(C)]
pub struct ITaskbarListVT {
	pub IUnknownVT: IUnknownVT,
	pub HrInit: fn(COMPTR) -> HRES,
	pub AddTab: fn(COMPTR, HANDLE) -> HRES,
	pub DeleteTab: fn(COMPTR, HANDLE) -> HRES,
	pub ActivateTab: fn(COMPTR, HANDLE) -> HRES,
	pub SetActiveAlt: fn(COMPTR, HANDLE) -> HRES,
}

#[repr(C)]
pub struct ITaskbarList2VT {
	pub ITaskbarListVT: ITaskbarListVT,
	pub MarkFullscreenWindow: fn(COMPTR, HANDLE, BOOL) -> HRES,
}

#[repr(C)]
pub struct ITaskbarList3VT {
	pub ITaskbarList2VT: ITaskbarList2VT,
	pub SetProgressValue: fn(COMPTR, HANDLE, u64, u64) -> HRES,
	pub SetProgressState: fn(COMPTR, HANDLE, u32) -> HRES,
	pub RegisterTab: fn(COMPTR, HANDLE, HANDLE) -> HRES,
	pub UnregisterTab: fn(COMPTR, HANDLE) -> HRES,
	pub SetTabOrder: fn(COMPTR, HANDLE, HANDLE) -> HRES,
	pub SetTabActive: fn(COMPTR, HANDLE, HANDLE, u32) -> HRES,
	pub ThumbBarAddButtons: fn(COMPTR, HANDLE, u32, PVOID) -> HRES,
	pub ThumbBarUpdateButtons: fn(COMPTR, HANDLE, u32, PVOID) -> HRES,
	pub ThumbBarSetImageList: fn(COMPTR, HANDLE, HANDLE) -> HRES,
	pub SetOverlayIcon: fn(COMPTR, HANDLE, HANDLE, PCSTR) -> HRES,
	pub SetThumbnailTooltip: fn(COMPTR, HANDLE, PCSTR) -> HRES,
	pub SetThumbnailClip: fn(COMPTR, HANDLE, PCVOID) -> HRES,
}

#[repr(C)]
pub struct ITaskbarList4VT {
	pub ITaskbarList3VT: ITaskbarList3VT,
	pub SetTabProperties: fn(COMPTR, HANDLE, u32) -> HRES,
}
