#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::decl::*;

/// [`PRINTER_DEFAULTS`](https://learn.microsoft.com/en-us/windows/win32/printdocs/printer-defaults)
/// struct.
pub struct PRINTER_DEFAULTS<'a, 'b> {
	pDataType: *mut u16,
	pDevMode: *mut DEVMODE,
	pub DesiredAccess: co::PRINTER_ACCESS,

	_pDataType: PhantomData<&'a mut u16>,
	_pDevMode: PhantomData<&'b mut DEVMODE>,
}

impl_default!(PRINTER_DEFAULTS, 'a, 'b);

impl<'a, 'b> PRINTER_DEFAULTS<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, pDataType, set_pDataType);
	pub_fn_ptr_get_set!('b, pDevMode, set_pDevMode, DEVMODE);
}

/// [`PRINTER_INFO_2`](https://learn.microsoft.com/en-us/windows/win32/printdocs/printer-info-2)
/// struct.
#[repr(C)]
#[derive(Clone)]
pub struct PRINTER_INFO_2<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm> {
	pServerName: *mut u16,
	pPrinterName: *mut u16,
	pShareName: *mut u16,
	pPortName: *mut u16,
	pDriverName: *mut u16,
	pComment: *mut u16,
	pLocation: *mut u16,
	pDevMode: *mut DEVMODE,
	pSepFile: *mut u16,
	pPrintProcessor: *mut u16,
	pDataType: *mut u16,
	pParameters: *mut u16,
	pSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
	pub Attributes: co::PRINTER_ATTRIBUTE_2,
	pub Priority: u32,
	pub DefaultPriority: u32,
	pub StartTime: u32,
	pub UntilTime: u32,
	pub Status: co::PRINTER_STATUS,
	pub cJobs: u32,
	pub AveragePPM: u32,

	_pServerName: PhantomData<&'a mut u16>,
	_pPrinterName: PhantomData<&'b mut u16>,
	_pShareName: PhantomData<&'c mut u16>,
	_pPortName: PhantomData<&'d mut u16>,
	_pDriverName: PhantomData<&'e mut u16>,
	_pComment: PhantomData<&'f mut u16>,
	_pLocation: PhantomData<&'g mut u16>,
	_pDevMode: PhantomData<&'h mut DEVMODE>,
	_pSepFile: PhantomData<&'i mut u16>,
	_pPrintProcessor: PhantomData<&'j mut u16>,
	_pDataType: PhantomData<&'k mut u16>,
	_pParameters: PhantomData<&'l mut u16>,
	_pSecurityDescriptor: PhantomData<&'m mut SECURITY_DESCRIPTOR>,
}

impl_default!(PRINTER_INFO_2, 'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm);

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm>
	PRINTER_INFO_2<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm>
{
	pub_fn_string_ptr_get_set!('a, pServerName, set_pServerName);
	pub_fn_string_ptr_get_set!('b, pPrinterName, set_pPrinterName);
	pub_fn_string_ptr_get_set!('c, pShareName, set_pShareName);
	pub_fn_string_ptr_get_set!('d, pPortName, set_pPortName);
	pub_fn_string_ptr_get_set!('e, pDriverName, set_pDriverName);
	pub_fn_string_ptr_get_set!('f, pComment, set_pComment);
	pub_fn_string_ptr_get_set!('g, pLocation, set_pLocation);
	pub_fn_ptr_get_set!('h, pDevMode, set_pDevMode, DEVMODE);
	pub_fn_string_ptr_get_set!('i, pSepFile, set_pSepFile);
	pub_fn_string_ptr_get_set!('j, pPrintProcessor, set_pPrintProcessor);
	pub_fn_string_ptr_get_set!('k, pDataType, set_pDataType);
	pub_fn_string_ptr_get_set!('l, pParameters, set_pParameters);
	pub_fn_ptr_get_set!('m, pSecurityDescriptor, set_pSecurityDescriptor, SECURITY_DESCRIPTOR);
}

/// [`PRINTER_INFO_4`](https://learn.microsoft.com/en-us/windows/win32/printdocs/printer-info-4)
/// struct.
#[repr(C)]
#[derive(Clone)]
pub struct PRINTER_INFO_4<'a, 'b> {
	pPrinterName: *mut u16,
	pServerName: *mut u16,
	pub Attributes: co::PRINTER_ATTRIBUTE_4,

	_pPrinterName: PhantomData<&'a mut u16>,
	_pServerName: PhantomData<&'b mut u16>,
}

impl_default!(PRINTER_INFO_4, 'a, 'b);

impl<'a, 'b> PRINTER_INFO_4<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, pPrinterName, set_pPrinterName);
	pub_fn_string_ptr_get_set!('b, pServerName, set_pServerName);
}
