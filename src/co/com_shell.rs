//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! constants.

const_type! { DROPEFFECT, u32,
	/// [`DROPEFFECT`](https://docs.microsoft.com/en-us/windows/win32/com/dropeffect-constants)
	/// constants (`u32`).
	=>
	NONE, 0
	COPY, 1
	MOVE, 2
	LINK, 4
	SCROLL, 0x80000000
}

const_type! { FOS, u32,
	/// [`_FILEOPENDIALOGOPTIONS`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-_fileopendialogoptions)
	/// enumeration (`u32`).
	=>
	OVERWRITEPROMPT, 0x2
	STRICTFILETYPES, 0x4
	NOCHANGEDIR, 0x8
	PICKFOLDERS, 0x20
	FORCEFILESYSTEM, 0x40
	ALLNONSTORAGEITEMS, 0x80
	NOVALIDATE, 0x100
	ALLOWMULTISELECT, 0x200
	PATHMUSTEXIST, 0x800
	FILEMUSTEXIST, 0x1000
	CREATEPROMPT, 0x2000
	SHAREAWARE, 0x4000
	NOREADONLYRETURN, 0x8000
	NOTESTFILECREATE, 0x10000
	HIDEMRUPLACES, 0x20000
	HIDEPINNEDPLACES, 0x40000
	NODEREFERENCELINKS, 0x100000
	OKBUTTONNEEDSINTERACTION, 0x200000
	DONTADDTORECENT, 0x2000000
	FORCESHOWHIDDEN, 0x10000000
	DEFAULTNOMINIMODE, 0x20000000
	FORCEPREVIEWPANEON, 0x40000000
	SUPPORTSTREAMABLEITEMS, 0x80000000
}

const_type! { SFGAO, u32,
	/// [`SFGAO`](https://docs.microsoft.com/en-us/windows/win32/shell/sfgao)
	/// constants (`u32`).
	=>
	CANCOPY, DROPEFFECT::COPY.0
	CANMOVE, DROPEFFECT::MOVE.0
	CANLINK, DROPEFFECT::LINK.0
	STORAGE, 0x00000008
	CANRENAME, 0x00000010
	CANDELETE, 0x00000020
	HASPROPSHEET, 0x00000040
	DROPTARGET, 0x00000100
	CAPABILITYMASK, 0x00000177
	SYSTEM, 0x00001000
	ENCRYPTED, 0x00002000
	ISSLOW, 0x00004000
	GHOSTED, 0x00008000
	LINK, 0x00010000
	SHARE, 0x00020000
	READONLY, 0x00040000
	HIDDEN, 0x00080000
	FILESYSANCESTOR, 0x10000000
	FOLDER, 0x20000000
	FILESYSTEM, 0x40000000
	HASSUBFOLDER, 0x80000000
	CONTENTSMASK, 0x80000000
	VALIDATE, 0x01000000
	REMOVABLE, 0x02000000
	COMPRESSED, 0x04000000
	BROWSABLE, 0x08000000
	NONENUMERATED, 0x00100000
	NEWCONTENT, 0x00200000
	CANMONIKER, 0x00400000
	HASSTORAGE, 0x00400000
	STREAM, 0x00400000
	STORAGEANCESTOR, 0x00800000
	STORAGECAPMASK, 0x70c50008
	PKEYSFGAOMASK, 0x81044000
}

const_type! { SIGDN, u32,
	/// [`SIGDN`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-sigdn)
	/// enumeration (`u32`).
	=>
	/// Returns the display name relative to the parent folder. In UI this name
	/// is generally ideal for display to the user.
	NORMALDISPLAY, 0
	/// Returns the parsing name relative to the parent folder. This name is not
	/// suitable for use in UI.
	PARENTRELATIVEPARSING, 0x80018001
	/// Returns the parsing name relative to the desktop. This name is not
	/// suitable for use in UI.
	DESKTOPABSOLUTEPARSING, 0x80028000
	/// Returns the editing name relative to the parent folder. In UI this name
	/// is suitable for display to the user.
	PARENTRELATIVEEDITING, 0x80031001
	/// Returns the editing name relative to the desktop. In UI this name is
	/// suitable for display to the user.
	DESKTOPABSOLUTEEDITING, 0x8004c000
	/// Returns the item's file system path, if it has one. Only items that
	/// report [`SFGAO::FILESYSTEM`](crate::co::SFGAO::FILESYSTEM) have a file
	/// system path. When an item does not have a file system path, a call to
	/// [`IShellItem::GetDisplayName`](crate::shell::IShellItem::GetDisplayName)
	/// on that item will fail. In UI this name is suitable for display to the
	/// user in some cases, but note that it might not be specified for all
	/// items.
	FILESYSPATH, 0x80058000
	/// Returns the item's URL, if it has one. Some items do not have a URL, and
	/// in those cases a call to
	/// [`IShellItem::GetDisplayName`](crate::shell::IShellItem::GetDisplayName)
	/// will fail. This name is suitable for display to the user in some cases,
	/// but note that it might not be specified for all items.
	URL, 0x80068000
	/// Returns the path relative to the parent folder in a friendly format as
	/// displayed in an address bar. This name is suitable for display to the
	/// user.
	PARENTRELATIVEFORADDRESSBAR, 0x8007c001
	/// Returns the path relative to the parent folder.
	PARENTRELATIVE, 0x80080001
	/// Introduced in Windows 8.
	PARENTRELATIVEFORUI, 0x80094001
}

const_type! { TBPF, u32,
	/// [`ITaskbarList3::SetProgressState`](crate::shell::ITaskbarList3::SetProgressState)
	/// `tbpFlags` (`u32`).
	=>
	NOPROGRESS, 0
	INDETERMINATE, 0x1
	NORMAL, 0x2
	ERROR, 0x4
	PAUSED, 0x8
}
