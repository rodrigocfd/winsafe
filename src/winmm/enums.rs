use crate::co;
use crate::decl::*;
use crate::prelude::*;

/// Variable parameter for:
///
/// * [`PlaySound`](crate::PlaySound)
pub enum Snd<'a> {
	/// Plays the system sound of the given
	/// [`co::SND_ALIAS`](crate::co::SND_ALIAS) asynchronously.
	AliasAsync {
		/// System sound alias.
		alias: co::SND_ALIAS,
		/// If `true`, stops any sound currently playing asynchronously.
		///
		/// If set to `false` and a sound is currently playing, `PlaySound` will
		/// return an error.
		stop: bool,
		/// Calls [`SoundSentry`](crate::SoundSentry) to give a visual cue as an
		/// accessibility feature.
		sentry: bool,
		/// Plays the sound repeatedly until you call `PlaySound` with
		/// [`Snd::Stop`](crate::Snd::Stop).
		loops: bool,
	},
	/// Plays the system sound of the given
	/// [`co::SND_ALIAS`](crate::co::SND_ALIAS) asynchronously.
	AliasSync {
		/// System sound alias.
		alias: co::SND_ALIAS,
		/// If `true`, stops any sound currently playing asynchronously.
		///
		/// If set to `false` and a sound is currently playing, `PlaySound` will
		/// return an error.
		stop: bool,
		/// Calls [`SoundSentry`](crate::SoundSentry) to give a visual cue as an
		/// accessibility feature.
		sentry: bool,
	},
	/// Loads a file to be played asynchronously.
	FileAsync {
		/// File path to be loaded.
		path: &'a str,
		/// If `true`, a default sound will be played if `path` is not found.
		default: bool,
		/// If `true`, stops any sound currently playing asynchronously.
		///
		/// If set to `false` and a sound is currently playing, `PlaySound` will
		/// return an error.
		stop: bool,
		/// Calls [`SoundSentry`](crate::SoundSentry) to give a visual cue as an
		/// accessibility feature.
		sentry: bool,
		/// Plays the sound repeatedly until you call `PlaySound` with
		/// [`Snd::Stop`](crate::Snd::Stop).
		loops: bool,
	},
	/// Loads a file to be played synchronously.
	FileSync {
		/// File path to be loaded.
		path: &'a str,
		/// If `true`, a default sound will be played if `path` is not found.
		default: bool,
		/// If `true`, stops any sound currently playing asynchronously.
		///
		/// If set to `false` and a sound is currently playing, `PlaySound` will
		/// return an error.
		stop: bool,
		/// Calls [`SoundSentry`](crate::SoundSentry) to give a visual cue as an
		/// accessibility feature.
		sentry: bool,
	},
	/// Reads the sound from memory and plays it synchronously.
	///
	/// Since `PlaySound` doesn't provide a mechanism to notify its exit, there
	/// is no safe way to play from memory asynchronously.
	MemSync {
		/// Source to be read from.
		src: &'a [u8],
		/// If `true`, a default sound will be played if `src` loading fails.
		default: bool,
		/// If `true`, stops any sound currently playing.
		///
		/// If set to `false` and a sound is currently playing, `PlaySound` will
		/// return an error.
		stop: bool,
		/// Calls [`SoundSentry`](crate::SoundSentry) to give a visual cue as an
		/// accessibility feature.
		sentry: bool,
	},
	/// Loads a sound from the resource and plays it asynchronously.
	ResAsync {
		/// Resource identifier.
		id: IdStr,
		/// Instance handle.
		hinst: &'a HINSTANCE,
		/// If `true`, a default sound will be played if `src` loading fails.
		default: bool,
		/// If `true`, stops any sound currently playing.
		///
		/// If set to `false` and a sound is currently playing, `PlaySound` will
		/// return an error.
		stop: bool,
		/// Calls [`SoundSentry`](crate::SoundSentry) to give a visual cue as an
		/// accessibility feature.
		sentry: bool,
		/// Plays the sound repeatedly until you call `PlaySound` with
		/// [`Snd::Stop`](crate::Snd::Stop).
		loops: bool,
	},
	/// Loads a sound from the resource and plays it synchronously.
	ResSync {
		/// Resource identifier.
		id: IdStr,
		/// Instance handle.
		hinst: &'a HINSTANCE,
		/// If `true`, a default sound will be played if `src` loading fails.
		default: bool,
		/// If `true`, stops any sound currently playing.
		///
		/// If set to `false` and a sound is currently playing, `PlaySound` will
		/// return an error.
		stop: bool,
		/// Calls [`SoundSentry`](crate::SoundSentry) to give a visual cue as an
		/// accessibility feature.
		sentry: bool,
	},
	/// Stops any sound currently playing asynchronously.
	Stop,
}

impl<'a> Snd<'a> {
	#[must_use]
	pub(in crate::winmm) fn serialize(
		&self,
		str_buf: &mut WString,
	) -> (*const std::ffi::c_void, HINSTANCE, co::SND) {
		use Snd::*;
		match self {
			AliasAsync { alias, stop, sentry, loops } => {
				*str_buf = WString::from_str(&alias.to_string());
				(
					str_buf.as_ptr() as _,
					HINSTANCE::NULL,
					Self::assembly_flags(true, *stop, *sentry, *loops)
						| co::SND::ALIAS | co::SND::ASYNC,
				)
			},
			AliasSync { alias, stop, sentry } => {
				*str_buf = WString::from_str(&alias.to_string());
				(
					str_buf.as_ptr() as _,
					HINSTANCE::NULL,
					Self::assembly_flags(true, *stop, *sentry, false)
						| co::SND::ALIAS | co::SND::SYNC,
				)
			},
			FileAsync { path, default, stop, sentry, loops } => {
				*str_buf = WString::from_str(path);
				(
					str_buf.as_ptr() as _,
					HINSTANCE::NULL,
					Self::assembly_flags(*default, *stop, *sentry, *loops)
						| co::SND::FILENAME
						| co::SND::ASYNC,
				)
			},
			FileSync { path, default, stop, sentry } => {
				*str_buf = WString::from_str(path);
				(
					str_buf.as_ptr() as _,
					HINSTANCE::NULL,
					Self::assembly_flags(*default, *stop, *sentry, false)
						| co::SND::FILENAME
						| co::SND::SYNC,
				)
			},
			MemSync { src, default, stop, sentry } => (
				src.as_ptr() as _,
				HINSTANCE::NULL,
				Self::assembly_flags(*default, *stop, *sentry, false)
					| co::SND::MEMORY
					| co::SND::SYNC,
			),
			ResAsync { id, hinst, default, stop, sentry, loops } => (
				id.as_ptr() as _,
				unsafe { hinst.raw_copy() },
				Self::assembly_flags(*default, *stop, *sentry, *loops)
					| co::SND::RESOURCE
					| co::SND::ASYNC,
			),
			ResSync { id, hinst, default, stop, sentry } => (
				id.as_ptr() as _,
				unsafe { hinst.raw_copy() },
				Self::assembly_flags(*default, *stop, *sentry, false)
					| co::SND::RESOURCE
					| co::SND::SYNC,
			),
			Stop => (std::ptr::null(), HINSTANCE::NULL, co::SND::SYNC | co::SND::PURGE),
		}
	}

	#[must_use]
	fn assembly_flags(default: bool, stop: bool, sentry: bool, loops: bool) -> co::SND {
		let mut flags = co::SND::default();
		if !default {
			flags |= co::SND::NODEFAULT;
		}
		if !stop {
			flags |= co::SND::NOSTOP;
		}
		if sentry {
			flags |= co::SND::SENTRY;
		}
		if loops {
			flags |= co::SND::LOOP;
		}
		flags
	}
}
