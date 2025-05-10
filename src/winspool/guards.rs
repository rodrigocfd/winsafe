use crate::decl::*;
use crate::prelude::*;
use crate::winspool::ffi;

handle_guard! { ClosePrinterGuard: HPRINTER;
	ffi::ClosePrinter;
	/// RAII implementation for [`HPRINTER`](crate::HPRINTER) which
	/// automatically calls
	/// [`ClosePrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/closeprinter)
	/// when the object goes out of scope.
}
