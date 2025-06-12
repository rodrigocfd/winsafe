use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::prelude::*;

struct RawModelessObj {
	raw_base: RawBase,
	_pin: PhantomPinned,
}

/// An ordinary modeless window.
///
/// Hierarchy: `BaseWnd` -> `RawBase` -> `RawModeless`.
#[derive(Clone)]
pub(in crate::gui) struct RawModeless(Pin<Arc<RawModelessObj>>);

impl RawModeless {
	#[must_use]
	pub(in crate::gui) fn new(
		parent: &(impl GuiParent + 'static),
		opts: WindowModelessOpts,
	) -> Self {
		let new_self = Self(Arc::pin(RawModelessObj {
			raw_base: RawBase::new(),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().wnd_ty().creation_msg(), move |_| {
				let hinst = parent2.hwnd().hinstance();
				let atom = self2.0.raw_base.register_class(
					&hinst,
					&opts.class_name,
					opts.class_style,
					&opts.class_icon,
					&opts.class_bg_brush,
					&opts.class_cursor,
				)?;

				let rc_parent = parent2
					.hwnd()
					.ClientToScreenRc(parent2.hwnd().GetClientRect()?)?;
				self2.0.raw_base.create_window(
					opts.ex_style,
					atom,
					None,
					opts.style,
					POINT::with(opts.position.0 + rc_parent.left, opts.position.1 + rc_parent.top),
					opts.size.into(),
					Some(parent2.hwnd()),
					IdMenu::None,
					&hinst,
				);

				Ok(0) // ignored
			});

		new_self
	}

	#[must_use]
	pub(in crate::gui) fn raw_base(&self) -> &RawBase {
		&self.0.raw_base
	}
}
