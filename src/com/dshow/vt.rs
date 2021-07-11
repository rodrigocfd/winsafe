

#![allow(non_snake_case)]

use crate::com::{IDispatchVT, IPersistVT, IUnknownVT, PPComVT};
use crate::ffi::{BOOL, HANDLE, HRESULT, PCSTR, PCVOID, PSTR, PVOID};

type IUnkPP = PPComVT<IUnknownVT>;

pub_struct_vtable! { IBaseFilterVT,

	=>
	,



}

pub_struct_vtable! { IEnumFiltersVT,

	=>
	,



}

pub_struct_vtable! { IFileSinkFilterVT,

	=>
	,



}

pub_struct_vtable! { IFilterGraphVT,

	=>
	,



}

pub_struct_vtable! { IGraphBuilderVT,

	=>
	,

}

pub_struct_vtable! { IMediaControlVT,

	=>
	,



}

pub_struct_vtable! { IMediaFilterVT,

	=>
	,



}

pub_struct_vtable! { IMediaSeekingVT,

	=>
	,



}

pub_struct_vtable! { IMFGetServiceVT,

	=>
	,



}

pub_struct_vtable! { IMFVideoDisplayControlVT,

	=>
	,



}

pub_struct_vtable! { IPinVT,

	=>
	,

}
