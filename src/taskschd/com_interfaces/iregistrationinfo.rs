#![allow(non_camel_case_types, non_snake_case)]

use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { IRegistrationInfo: "416d8b73-cb41-4ea1-805c-9be9a5ac4a74";
	/// [`IRegistrationInfo`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iregistrationinfo)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for IRegistrationInfo {}
impl taskschd_IRegistrationInfo for IRegistrationInfo {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IRegistrationInfo`](crate::IRegistrationInfo).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IRegistrationInfo: oleaut_IDispatch {
	fn_com_bstr_get! { get_Author: IRegistrationInfoVT;
		/// [`IRegistrationInfo::get_Author`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregistrationinfo-get_author)
		/// method.
	}

	fn_com_bstr_set! { put_Author: IRegistrationInfoVT, author;
		/// [`IRegistrationInfo::put_Author`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregistrationinfo-put_author)
		/// method.
	}
}
