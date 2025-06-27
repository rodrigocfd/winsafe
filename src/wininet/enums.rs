use crate::decl::*;

/// Variant parameter for:
///
/// * [`HINTERNETREQUEST::HttpQueryInfo`](crate::HINTERNETREQUEST::HttpQueryInfo)
pub enum HttpInfo {
	/// If
	/// [`co::HTTP_QUERY_FLAG::NUMBER`](crate::co::HTTP_QUERY_FLAG::NUMBER)
	/// is present.
	Number(u32),
	/// If
	/// [`co::HTTP_QUERY_FLAG::NUMBER64`](crate::co::HTTP_QUERY_FLAG::NUMBER64)
	/// is present.
	Number64(u64),
	/// Default value format.
	Str(String),
	/// If
	/// [`co::HTTP_QUERY_FLAG::SYSTEMTIME`](crate::co::HTTP_QUERY_FLAG::SYSTEMTIME)
	/// is present.
	Time(SYSTEMTIME),
}

impl HttpInfo {
	/// If the value is [`HttpInfo::Number`](crate::HttpInfo::Number), returns
	/// it; otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// `HttpInfo` content, its general use is discouraged.
	#[must_use]
	pub const fn unwrap_number(&self) -> u32 {
		match self {
			Self::Number(n) => *n,
			_ => panic!("HttpInfo does not contain Number."),
		}
	}

	/// If the value is [`HttpInfo::Number64`](crate::HttpInfo::Number64),
	/// returns it; otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// `HttpInfo` content, its general use is discouraged.
	#[must_use]
	pub const fn unwrap_number64(&self) -> u64 {
		match self {
			Self::Number64(n) => *n,
			_ => panic!("HttpInfo does not contain Number64."),
		}
	}

	/// If the value is [`HttpInfo::Str`](crate::HttpInfo::Str), returns a clone
	/// it; otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// `HttpInfo` content, its general use is discouraged.
	#[must_use]
	pub fn unwrap_str(&self) -> String {
		match self {
			Self::Str(s) => s.clone(),
			_ => panic!("HttpInfo does not contain Str."),
		}
	}

	/// If the value is [`HttpInfo::Time`](crate::HttpInfo::Time), returns a
	/// copy of it; otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// `HttpInfo` content, its general use is discouraged.
	#[must_use]
	pub const fn unwrap_time(&self) -> SYSTEMTIME {
		match self {
			Self::Time(st) => *st,
			_ => panic!("HttpInfo does not contain Time."),
		}
	}
}
