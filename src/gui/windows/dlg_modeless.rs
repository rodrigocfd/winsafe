use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::prelude::*;

struct DlgModelessObj {
	dlg_base: DlgBase,
	_pin: PhantomPinned,
}

/// A dialog-based modeless window.
#[derive(Clone)]
pub(in crate::gui) struct DlgModeless(Pin<Arc<DlgModelessObj>>);

impl DlgModeless {
	#[must_use]
	pub(in crate::gui) fn new(
		parent: &(impl GuiParent + 'static),
		dlg_id: u16,
		position: (i32, i32),
	) -> Self {
		let new_self = Self(Arc::pin(DlgModelessObj {
			dlg_base: DlgBase::new(dlg_id),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().wnd_ty().creation_msg(), move |_| {
				let hinst = parent2.hwnd().hinstance();
				self2.0.dlg_base.create_dialog_param(&hinst)?;
				self2.0.dlg_base.base().hwnd().ShowWindow(co::SW::SHOW);

				let rc_parent = parent2
					.hwnd()
					.ClientToScreenRc(parent2.hwnd().GetClientRect()?)?;
				self2.0.dlg_base.base().hwnd().SetWindowPos(
					HwndPlace::None,
					POINT::with(position.0 + rc_parent.left, position.1 + rc_parent.top),
					SIZE::default(),
					co::SWP::NOZORDER | co::SWP::NOSIZE,
				)?;

				Ok(0) // ignored
			});

		new_self
	}

	#[must_use]
	pub(in crate::gui) fn dlg_base(&self) -> &DlgBase {
		&self.0.dlg_base
	}
}
