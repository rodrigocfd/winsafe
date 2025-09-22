#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { ITaskDefinition: "f5bc8fc5-536d-4f77-b852-fbc1356fdeb6";
	/// [`ITaskDefinition`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itaskdefinition)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for ITaskDefinition {}
impl taskschd_ITaskDefinition for ITaskDefinition {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`ITaskDefinition`](crate::ITaskDefinition).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITaskDefinition: oleaut_IDispatch {
	fn_com_interface_get! { get_Actions: ITaskDefinitionVT => IActionCollection;
		/// [`ITaskDefinition::get_Actions`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_actions)
		/// method.
	}

	fn_com_bstr_get! { get_Data: ITaskDefinitionVT;
		/// [`ITaskDefinition::get_Data`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_data)
		/// method.
	}

	fn_com_interface_get! { get_Principal: ITaskDefinitionVT => IPrincipal;
		/// [`ITaskDefinition::get_Principal`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_principal)
		/// method.
	}

	fn_com_interface_get! { get_RegistrationInfo: ITaskDefinitionVT => IRegistrationInfo;
		/// [`ITaskDefinition::get_RegistrationInfo`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_registrationinfo)
		/// method.
	}

	fn_com_interface_get! { get_Triggers: ITaskDefinitionVT => ITriggerCollection;
		/// [`ITaskDefinition::get_Triggers`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_triggers)
		/// method.
	}

	fn_com_bstr_get! { get_XmlText: ITaskDefinitionVT;
		/// [`ITaskDefinition::get_XmlText`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_xmltext)
		/// method.
	}

	fn_com_bstr_set! { put_Data: ITaskDefinitionVT, data;
		/// [`ITaskDefinition::put_Data`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-put_data)
		/// method.
	}

	fn_com_bstr_set! { put_XmlText: ITaskDefinitionVT, xml;
		/// [`ITaskDefinition::put_XmlText`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-put_xmltext)
		/// method.
	}
}
