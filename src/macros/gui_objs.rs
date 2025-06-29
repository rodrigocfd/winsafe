#![allow(unused_macros)]

/// Declares a native control, optionally with a single generic parameter.
macro_rules! native_ctrl {
	(
		$name:ident : $innerobj:ty $( , $genp:tt )? $( => $events:ty )?;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub struct $name<$( $genp: 'static = () )?>(Pin<Arc<$innerobj>>);

		unsafe impl<$( $genp )?> Send for $name<$( $genp )?> {}

		// A simple #[derive(Clone)] works for non-generic controls, but for
		// generic ones, we need manual implementation.
		// https://stackoverflow.com/q/39415052/6923555
		impl<$( $genp )?> Clone for $name<$( $genp )?> {
			fn clone(&self) -> Self {
				Self(self.0.clone())
			}
		}

		impl<$( $genp )?> crate::prelude::GuiWindow for $name<$( $genp )?> {
			fn hwnd(&self) -> &crate::HWND {
				self.0.base.hwnd()
			}

			fn as_any(&self) -> &dyn Any {
				self
			}
		}

		impl<$( $genp )?> crate::prelude::GuiControl for $name<$( $genp )?> {
			fn ctrl_id(&self) -> u16 {
				self.0.base.ctrl_id()
			}
		}

		impl<$( $genp )?> $name<$( $genp )?> {
			/// Exposes the subclass events. If at least one event exists, the control
			/// will be
			/// [subclassed](https://learn.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
			///
			/// **Note:** Subclassing may impact performance, use with care.
			///
			/// # Panics
			///
			/// Panics if the control or the parent window are already created. Events
			/// must be set before control and parent window creation.
			#[must_use]
			pub fn on_subclass(&self) -> &crate::gui::events::WindowEvents {
				self.0.base.on_subclass()
			}

			$(
				/// Exposes the specific control events.
				///
				/// # Panics
				///
				/// Panics if the control is already created. Events must be set before
				/// control creation.
				#[must_use]
				pub fn on(&self) -> &$events {
					if *self.hwnd() != crate::HWND::NULL {
						panic!("Cannot add events after control creation.");
					}
					&self.0.events
				}
			)?
		}
	};
}
