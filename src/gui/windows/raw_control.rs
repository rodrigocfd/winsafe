use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::prelude::*;

struct RawControlObj {
	raw_base: RawBase,
	ctrl_id: u16,
	_pin: PhantomPinned,
}

/// An ordinary custom control window.
#[derive(Clone)]
pub(in crate::gui) struct RawControl(Pin<Arc<RawControlObj>>);

impl RawControl {
	#[must_use]
	pub(in crate::gui) fn new(
		parent: &(impl GuiParent + 'static),
		opts: WindowControlOpts,
	) -> Self
	{
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(
			Arc::pin(
				RawControlObj {
					raw_base: RawBase::new(),
					ctrl_id,
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent.as_ref().before_on().wm(parent.as_ref().is_dlg().create_msg(), move |_| {
			let hinst = parent2.hwnd().hinstance();
			let atom = self2.0.raw_base.register_class(&hinst, &opts.class_name,
				opts.class_style, &opts.class_icon, &opts.class_bg_brush, &opts.class_cursor)?;
			self2.0.raw_base.create_window(opts.ex_style, atom, None, opts.style,
				opts.position.into(), opts.size.into(), Some(parent2.hwnd()),
				IdMenu::Id(ctrl_id), &hinst)?;

			parent2.as_ref().add_to_layout(self2.0.raw_base.base().hwnd(), opts.resize_behavior)?;
			Ok(0) // ignored
		});

		new_self.default_message_handlers();
		new_self
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.0.raw_base.base().before_on().wm_nc_paint(move |p| {
			paint_control_borders(self2.0.raw_base.base().hwnd(), p)?;
			Ok(())
		});
	}

	#[must_use]
	pub(in crate::gui) fn raw_base(&self) -> &RawBase {
		&self.0.raw_base
	}

	#[must_use]
	pub(in crate::gui) fn ctrl_id(&self) -> u16 {
		self.0.ctrl_id
	}
}
