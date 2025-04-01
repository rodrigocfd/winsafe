#![allow(non_camel_case_types, non_snake_case)]

use std::mem::ManuallyDrop;
use std::sync::atomic::AtomicU32;

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::mf::vts::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;

com_interface_userdef! { IMFAsyncCallback, IMFAsyncCallbackImpl: "a27003cf-2354-4f2a-8d6a-ab7cff15437e";
	/// [`IMFAsyncCallback`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nn-mfobjects-imfasynccallback)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl IMFAsyncCallback {
	fn_com_userdef_closure! { GetParameters: Fn(&mut co::MFASYNC, &mut u32) -> AnyResult<()>;
		/// [`IMFAsyncCallback::GetParameters`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfasynccallback-getparameters)
		/// method.
	}

	fn_com_userdef_closure! { Invoke: Fn(&IMFAsyncResult) -> AnyResult<()>;
		/// [`IMFAsyncCallback::Invoke`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfasynccallback-invoke)
		/// method.
	}
}

#[repr(C)]
pub struct IMFAsyncCallbackImpl {
	vt: IMFAsyncCallbackVT,
	counter: AtomicU32,
	GetParameters: Option<Box<dyn Fn(&mut co::MFASYNC, &mut u32) -> AnyResult<()>>>,
	Invoke: Option<Box<dyn Fn(&IMFAsyncResult) -> AnyResult<()>>>,
}

impl IMFAsyncCallbackImpl {
	fn new() -> Self {
		Self {
			vt: IMFAsyncCallbackVT {
				IUnknownVT: IUnknownVT {
					QueryInterface: Self::QueryInterface,
					AddRef: Self::AddRef,
					Release: Self::Release,
				},
				GetParameters: Self::GetParameters,
				Invoke: Self::Invoke,
			},
			counter: AtomicU32::new(1),
			GetParameters: None,
			Invoke: None,
		}
	}

	com_interface_userdef_iunknown_methods!(Self);

	fn GetParameters(p: COMPTR, pdwFlags: *mut u32, pdwQueue: *mut u32) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.GetParameters {
			Some(func) => {
				let pflags = unsafe { &mut *(pdwFlags as *mut co::MFASYNC) };
				let pqueue = unsafe { &mut *(pdwQueue) };
				anyresult_to_hresult(func(pflags, pqueue))
			},
			None => Ok(()),
		})
	}

	fn Invoke(p: COMPTR, pAsyncResult: COMPTR) -> HRES {
		let box_impl = box_impl_of::<Self>(p);
		hrresult_to_hres(match &box_impl.Invoke {
			Some(func) => {
				let ar = ManuallyDrop::new(unsafe { IMFAsyncResult::from_ptr(pAsyncResult) });
				anyresult_to_hresult(func(&ar))
			},
			None => Ok(()),
		})
	}
}
