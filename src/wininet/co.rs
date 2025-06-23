#![allow(non_camel_case_types, non_upper_case_globals)]

const_bitflag! { INTERNET_FLAG: u32;
	/// Internet API
	/// [flags](https://learn.microsoft.com/en-us/windows/win32/wininet/api-flags)
	/// (`u32`).
	=>
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

const_ordinary! { INTERNET_PORT: u16;
	/// [`HINTERNET::InternetOpen`](crate::HINTERNET::InternetOpen) `port`
	/// (`u16`).
	///
	/// Originally has `INTERNET_DEFAULT` prefix and `PORT` suffix.
	=>
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
