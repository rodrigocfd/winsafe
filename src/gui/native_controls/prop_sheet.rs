use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::prelude::*;

struct PropSheetObj {
	title: String,
	page_titles: Vec<WString>,
	pages: Vec<PropSheetPage>,
	apply_button: bool,
	context_help: bool,
	margin: bool,
}

/// Native
/// [property sheet](https://learn.microsoft.com/en-us/windows/win32/controls/property-sheets)
/// control.
///
/// # Examples
///
/// The example below creates a property sheet with one
/// [`PropSheetPage`](crate::gui::PropSheetPage) page.
///
/// ```no_run
/// use winsafe::{self as w, co, gui, prelude::*, seq_ids};
///
/// seq_ids! {
///     DLG_PAGE_1 = 100; // must match the resource dialog
/// }
///
/// fn main() {
///     let ps = gui::PropSheet::new(gui::PropSheetOpts {
///         title: "Main title",
///         pages: &[
///             ("First page", FirstPage::new().into()),
///         ],
///         ..Default::default()
///     });
///
///     // In a real application, the parent window is passed to show().
///     if let Err(e) = ps.show(None::<&gui::WindowMain>) {
///         eprintln!("{}", e);
///     }
/// }
///
/// #[derive(Clone)]
/// struct FirstPage {
///     wnd: gui::PropSheetPage,
/// }
/// impl Into<gui::PropSheetPage> for FirstPage {
///     fn into(self) -> gui::PropSheetPage {
///         self.wnd.clone() // so we can pass our custom struct to PropSheet::new()
///     }
/// }
/// impl FirstPage {
///     fn new() -> Self {
///         let wnd = gui::PropSheetPage::new_dlg(DLG_PAGE_1);
///         let new_self = Self { wnd };
///         new_self.events();
///         new_self
///     }
///     fn events(&self) {
///         let self2 = self.clone();
///         self.wnd.on().psn_apply(move || {
///             // When the user clicks OK, each page will receive this notification.
///             println!("OK from the first page");
///             Ok(co::PSNRET::NOERROR)
///         });
///     }
/// }
/// ```
#[derive(Clone)]
pub struct PropSheet(Pin<Arc<PropSheetObj>>);

unsafe impl Send for PropSheet {}

impl PropSheet {
	/// Instantiates a new `PropSheet` object, to be created with
	/// [`PropertySheet`](crate::PropertySheet).
	///
	/// # Panics
	///
	/// Panics if [`PropSheetOpts`](crate::gui::PropSheetOpts) has no pages.
	#[must_use]
	pub fn new(opts: PropSheetOpts) -> Self {
		if opts.pages.is_empty() {
			panic!("Property sheet has no pages.");
		}

		let (page_titles, pages): (Vec<WString>, Vec<PropSheetPage>) = opts
			.pages
			.into_iter()
			.map(|(page_title, page)| (WString::from_str(*page_title), page.clone()))
			.unzip();

		Self(Arc::pin(PropSheetObj {
			title: opts.title.to_owned(),
			page_titles,
			pages,
			apply_button: opts.apply_button,
			context_help: opts.context_help,
			margin: opts.margin,
		}))
	}

	/// Physically creates the property sheet window. This method will block
	/// until the window is closed.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn show(&self, parent: Option<&impl GuiParent>) -> AnyResult<()> {
		let ps_pages = self
			.0
			.pages
			.iter()
			.zip(&self.0.page_titles)
			.map(|(page, page_title)| {
				let mut ps_page = page.generate_propsheetpage();
				ps_page.pszTitle = page_title.as_ptr();
				ps_page
			})
			.collect::<Vec<_>>();

		let mut psh = PROPSHEETHEADER::default();
		psh.dwFlags = co::PSH::PROPSHEETPAGE;
		if !self.0.apply_button {
			psh.dwFlags |= co::PSH::NOAPPLYNOW;
		}
		if !self.0.context_help {
			psh.dwFlags |= co::PSH::NOCONTEXTHELP;
		}
		if !self.0.margin {
			psh.dwFlags |= co::PSH::NOMARGIN;
		}
		// psh.dwFlags = co::PSH::USECALLBACK;
		// psh.pfnCallback = Some(Self::ps_proc);

		match parent {
			Some(parent) => {
				psh.hInstance = parent.hwnd().hinstance();
				psh.hwndParent = unsafe { parent.hwnd().raw_copy() };
			},
			None => {
				psh.hInstance = HINSTANCE::GetModuleHandle(None).expect(DONTFAIL);
			},
		}

		let mut wcaption = WString::from_str(&self.0.title);
		psh.set_pszCaption(Some(&mut wcaption));

		psh.nPages = ps_pages.len() as _;
		psh.set_ppsp(&ps_pages);

		unsafe {
			PropertySheet(&psh)?;
		}
		Ok(())
	}
}

/// Options to create a [`PropSheet`](crate::gui::PropSheet) programmatically
/// with [`PropSheet::new`](crate::gui::PropSheet::new).
pub struct PropSheetOpts<'a> {
	/// Title of the property sheet container window.
	///
	/// Defaults to an empty string.
	pub title: &'a str,
	/// Pages to be added to the property sheet container window. The tuple
	/// contains the title of the page and the window to be rendered inside of
	/// it.
	///
	/// Defaults to none.
	pub pages: &'a [(&'a str, PropSheetPage)],
	/// Display the "Apply" button?
	///
	/// Defaults to `true`.
	pub apply_button: bool,
	/// Display the context help button in the titlebar?
	///
	/// Defaults to `false`.
	pub context_help: bool,
	/// Insert a margin between the page and the frame?
	///
	/// Defaults to `true`.
	pub margin: bool,
}

impl<'a> Default for PropSheetOpts<'a> {
	fn default() -> Self {
		Self {
			title: "",
			pages: &[],
			apply_button: true,
			context_help: false,
			margin: true,
		}
	}
}
