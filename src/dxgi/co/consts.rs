#![allow(non_camel_case_types)]

const_bitflag! { DXGI_MWA: u32;
	/// [`IDXGIFactory::GetWindowAssociation`](crate::prelude::dgxi_IDXGIFactory::GetWindowAssociation)
	/// `flags` (`u32`).
	=>
	=>
	NO_WINDOW_CHANGES (1 << 0)
	NO_ALT_ENTER (1 << 1)
	NO_PRINT_SCREEN (1 << 2)
}
