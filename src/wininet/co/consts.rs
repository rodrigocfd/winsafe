#![allow(non_camel_case_types, non_upper_case_globals)]

const_ordinary! { HTTP_QUERY: u32;
	/// [`HINTERNETREQUEST::HttpQueryInfo`](crate::HINTERNETREQUEST::HttpQueryInfo)
	/// `info` (`u32`).
	=>
	MIME_VERSION 0
	CONTENT_TYPE 1
	CONTENT_TRANSFER_ENCODING 2
	CONTENT_ID 3
	CONTENT_DESCRIPTION 4
	CONTENT_LENGTH 5
	CONTENT_LANGUAGE 6
	ALLOW 7
	PUBLIC 8
	DATE 9
	EXPIRES 10
	LAST_MODIFIED 11
	MESSAGE_ID 12
	URI 13
	DERIVED_FROM 14
	COST 15
	LINK 16
	PRAGMA 17
	VERSION 18
	STATUS_CODE 19
	STATUS_TEXT 20
	RAW_HEADERS 21
	RAW_HEADERS_CRLF 22
	CONNECTION 23
	ACCEPT 24
	ACCEPT_CHARSET 25
	ACCEPT_ENCODING 26
	ACCEPT_LANGUAGE 27
	AUTHORIZATION 28
	CONTENT_ENCODING 29
	FORWARDED 30
	FROM 31
	IF_MODIFIED_SINCE 32
	LOCATION 33
	ORIG_URI 34
	REFERER 35
	RETRY_AFTER 36
	SERVER 37
	TITLE 38
	USER_AGENT 39
	WWW_AUTHENTICATE 40
	PROXY_AUTHENTICATE 41
	ACCEPT_RANGES 42
	SET_COOKIE 43
	COOKIE 44
	REQUEST_METHOD 45
	REFRESH 46
	CONTENT_DISPOSITION 47

	AGE 48
	CACHE_CONTROL 49
	CONTENT_BASE 50
	CONTENT_LOCATION 51
	CONTENT_MD5 52
	CONTENT_RANGE 53
	ETAG 54
	HOST 55
	IF_MATCH 56
	IF_NONE_MATCH 57
	IF_RANGE 58
	IF_UNMODIFIED_SINCE 59
	MAX_FORWARDS 60
	PROXY_AUTHORIZATION 61
	RANGE 62
	TRANSFER_ENCODING 63
	UPGRADE 64
	VARY 65
	VIA 66
	WARNING 67
	EXPECT 68
	PROXY_CONNECTION 69
	UNLESS_MODIFIED_SINCE 70

	ECHO_REQUEST 71
	ECHO_REPLY 72

	ECHO_HEADERS 73
	ECHO_HEADERS_CRLF 74

	PROXY_SUPPORT 75
	AUTHENTICATION_INFO 76
	PASSPORT_URLS 77
	PASSPORT_CONFIG 78

	X_CONTENT_TYPE_OPTIONS 79
	P3P 80
	X_P2P_PEERDIST 81
	TRANSLATE 82
	X_UA_COMPATIBLE 83
	DEFAULT_STYLE 84
	X_FRAME_OPTIONS 85
	X_XSS_PROTECTION 86

	SET_COOKIE2 87

	DO_NOT_TRACK 88

	KEEP_ALIVE 89

	HTTP2_SETTINGS 90

	STRICT_TRANSPORT_SECURITY 91

	TOKEN_BINDING 92

	INCLUDE_REFERRED_TOKEN_BINDING_ID 93
	INCLUDE_REFERER_TOKEN_BINDING_ID Self::INCLUDE_REFERRED_TOKEN_BINDING_ID.0

	PUBLIC_KEY_PINS 94
	PUBLIC_KEY_PINS_REPORT_ONLY 95
}

const_ordinary! { HTTP_QUERY_FLAG: u32;
	/// [`HINTERNETREQUEST::HttpQueryInfo`](crate::HINTERNETREQUEST::HttpQueryInfo)
	/// `flags` (`u32`).
	=>
	/// None of the actual values (zero).
	NoValue 0
	COALESCE 0x1000_0000
	COALESCE_WITH_COMMA 0x0400_0000
	NUMBER 0x2000_0000
	NUMBER64 0x0800_0000
	REQUEST_HEADERS 0x8000_0000
	SYSTEMTIME 0x4000_0000
}

const_bitflag! { ICU: u32;
	/// [`InternetCanonicalizeUrl`](crate::InternetCanonicalizeUrl) `flags`
	/// (`u32`).
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Don't convert unsafe characters to escape sequence.
	NO_ENCODE 0x2000_0000
	/// Convert %XX escape sequences to characters.
	DECODE 0x1000_0000
	/// Don't convert .. etc. meta path sequences.
	NO_META 0x0800_0000
	/// Encode spaces only.
	ENCODE_SPACES_ONLY 0x0400_0000
	/// Special encode/decode rules for browser.
	BROWSER_MODE 0x0200_0000
	/// Encode any percent (ASCII25).
	ENCODE_PERCENT 0x0000_1000
}

const_bitflag! { INTERNET_FLAG: u32;
	/// Internet API
	/// [flags](https://learn.microsoft.com/en-us/windows/win32/wininet/api-flags)
	/// (`u32`).
	=>
	/// None of the actual values (zero).
	NoValue 0

	/// IDN enabled for direct connections.
	IDN_DIRECT 0x0000_0001
	/// IDN enabled for proxy.
	IDN_PROXY 0x0000_0002

	/// Retrieve the original item.
	RELOAD 0x8000_0000

	/// FTP/gopher find: receive the item as raw (structured) data.
	RAW_DATA 0x4000_0000
	/// FTP: use existing InternetConnect handle for server if possible.
	EXISTING_CONNECT 0x2000_0000

	/// This request is asynchronous (where supported).
	ASYNC 0x1000_0000

	/// Used for FTP connections.
	PASSIVE 0x0800_0000

	/// Don't write this item to the cache (same as `DONT_CACHE`).
	NO_CACHE_WRITE 0x0400_0000
	/// Don't write this item to the cache (same as `NO_CACHE_WRITE`).
	DONT_CACHE Self::NO_CACHE_WRITE.0
	/// Make this item persistent in cache.
	MAKE_PERSISTENT 0x0200_0000
	/// Use offline semantics (same as `OFFLINE`).
	FROM_CACHE 0x0100_0000
	/// Use offline semantics (same as `FROM_CACHE`).
	OFFLINE Self::FROM_CACHE.0

	/// Use PCT/SSL if applicable (HTTP).
	SECURE 0x0080_0000
	/// Use keep-alive semantics.
	KEEP_CONNECTION 0x0040_0000
	/// Don't handle redirections automatically.
	NO_AUTO_REDIRECT 0x0020_0000
	/// Do background read prefetch.
	READ_PREFETCH 0x0010_0000
	/// No automatic cookie handling.
	NO_COOKIES 0x0008_0000
	/// No automatic authentication handling.
	NO_AUTH 0x0004_0000
	/// Apply restricted zone policies for cookies, auth.
	RESTRICTED_ZONE 0x0002_0000
	/// Return cache file if net request fails.
	CACHE_IF_NET_FAIL 0x0001_0000

	/// Ex: `https://` to `http://`.
	IGNORE_REDIRECT_TO_HTTP 0x0000_8000
	/// Ex: `http://` to `https://`.
	IGNORE_REDIRECT_TO_HTTPS 0x0000_4000
	/// Expired X509 Cert.
	IGNORE_CERT_DATE_INVALID 0x0000_2000
	/// Bad common name in X509 Cert.
	IGNORE_CERT_CN_INVALID 0x0000_1000

	/// Asking wininet to update an item if it is newer.
	RESYNCHRONIZE 0x0000_0800
	/// Asking wininet to do hyperlinking semantic which works right for
	/// scripts.
	HYPERLINK 0x0000_0400
	/// No cookie popup.
	NO_UI 0x0000_0200
	/// Asking wininet to add "pragma: no-cache".
	PRAGMA_NOCACHE 0x0000_0100
	/// Ok to perform lazy cache-write.
	CACHE_ASYNC 0x0000_0080
	/// This is a forms submit.
	FORMS_SUBMIT 0x0000_0040
	/// Fwd-back button op.
	FWD_BACK 0x0000_0020
	/// Need a file for this request (same as `MUST_CACHE_REQUEST`).
	NEED_FILE 0x0000_0010
	/// Need a file for this request (same as `NEED_FILE`).
	MUST_CACHE_REQUEST Self::NEED_FILE.0

	TRANSFER_ASCII 0x0000_0001 // FTP_TRANSFER_TYPE_ASCII
	TRANSFER_BINARY 0x0000_0002 // FTP_TRANSFER_TYPE_BINARY
}

const_ordinary! { INTERNET_OPEN_TYPE: u32;
	/// [`HINTERNET::InternetOpen`](crate::HINTERNET::InternetOpen)
	/// `access_type` (`u32`).
	=>
	/// Retrieves the proxy or direct configuration from the registry.
	PRECONFIG 0
	/// Resolves all host names locally.
	DIRECT 1
	/// Passes requests to the proxy unless a proxy bypass list is supplied and
	/// the name to be resolved bypasses the proxy. In this case, the function
	/// uses `DIRECT`.
	PROXY 3
	/// Retrieves the proxy or direct configuration from the registry and
	/// prevents the use of a startup Microsoft JScript or Internet Setup (INS)
	/// file.
	PRECONFIG_WITH_NO_AUTOPROXY 4
}

const_ordinary! { INTERNET_DEFAULT_PORT: u16;
	/// [`HINTERNET::InternetOpen`](crate::HINTERNET::InternetOpen) `port`
	/// (`u16`).
	///
	/// Originally has `INTERNET_DEFAULT` prefix and `PORT` suffix.
	=>
	/// None of the actual values (zero).
	NoValue 0

	/// Uses the default port for the service specified by `service``.
	INVALID 0
	/// Uses the default port for FTP servers (port 21).
	FTP 21
	/// Uses the default port for Gopher servers (port 70).
	GOPHER 70
	/// Uses the default port for HTTP servers (port 80).
	HTTP 80
	/// Uses the default port for Secure Hypertext Transfer Protocol (HTTPS)
	/// servers (port 443).
	HTTPS 443
	/// Uses the default port for SOCKS firewall servers (port 1080).
	SOCKS 1080
}

const_ordinary! { INTERNET_SERVICE: u32;
	/// [`HINTERNET::InternetOpen`](crate::HINTERNET::InternetOpen) `service`
	/// (`u32`).
	=>
	FTP 1
	GOPHER 2
	HTTP 3
}
