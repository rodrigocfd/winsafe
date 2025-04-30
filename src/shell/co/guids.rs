#![allow(non_upper_case_globals)]

use crate::co::*;

const_guid_values! { CLSID;
	FileOpenDialog "dc1c5a9c-e88a-4dde-a5a1-60f82a20aef7"
	FileOperation "3ad05575-8857-4850-9277-11b85bdb8e09"
	FileSaveDialog "c0b4e2f3-ba21-4773-8dba-335ec946eb8b"
	ShellLink "00021401-0000-0000-c000-000000000046"
	TaskbarList "56fdf344-fd6d-11d0-958a-006097c9a090"
}

const_guid! { BHID;
	/// [`IShellItem::BindToHandler`](crate::prelude::shell_IShellItem::BindToHandler)
	/// `bhid` (`GUID`).
	=>
	SFObject "3981e224-f559-11d3-8e3a-00c04f6837d5"
	SFUIObject "3981e225-f559-11d3-8e3a-00c04f6837d5"
	SFViewObject "3981e226-f559-11d3-8e3a-00c04f6837d5"
	Storage "3981e227-f559-11d3-8e3a-00c04f6837d5"
	Stream "1cebb3ab-7c10-499a-a417-92ca16c4cb83"
	RandomAccessStream "f16fc93b-77ae-4cfe-bda7-a866eea6878d"
	LinkTargetItem "3981e228-f559-11d3-8e3a-00c04f6837d5"
	StorageEnum "4621a4e3-f0d6-4773-8a9c-46e77b174840"
	Transfer "d5e346a1-f753-4932-b403-4574800e2498"
	PropertyStore "0384e1a4-1523-439c-a4c8-ab911052f586"
	ThumbnailHandler "7b2e650a-8e20-4f4a-b09e-6597afc72fb0"
	EnumItems "94f60519-2850-4924-aa5a-d15e84868039"
	DataObject "b8c0bd9f-ed24-455c-83e6-d5390c4fe8c4"
	AssociationArray "bea9ef17-82f1-4f60-9284-4f8db75c3be9"
	Filter "38d08778-f557-4690-9ebf-ba54706ad8f7"
	EnumAssocHandlers "b8ab0b9c-c2ec-4f7a-918d-314900e6280a"
	StorageItem "404e2109-77d2-4699-a5a0-4fdf10db9837"
	FilePlaceholder "8677dceb-aae0-4005-8d3d-547fa852f825"
}
