const_ordinary! { CLSCTX: u32: "ole";
	/// [`CLSCTX`](https://docs.microsoft.com/en-us/windows/win32/api/wtypesbase/ne-wtypesbase-clsctx)
	/// enumeration (`u32`).
	=>
	=>
	/// Same process.
	///
	/// The code that creates and manages objects of this class is a DLL that
	/// runs in the same process as the caller of the function specifying the
	/// class context.
	INPROC_SERVER 0x1
	/// The code that manages objects of this class is an in-process handler.
	/// This is a DLL that runs in the client process and implements client-side
	/// structures of this class when instances of the class are accessed
	/// remotely.
	INPROC_HANDLER 0x2
	/// Different process, same computer.
	///
	/// The EXE code that creates and manages objects of this class runs on same
	/// machine but is loaded in a separate process space.
	LOCAL_SERVER 0x4
	/// Different computer.
	///
	/// A remote context. The `LocalServer32` or `LocalService` code that creates
	/// and manages objects of this class is run on a different computer.
	REMOTE_SERVER 0x10
	/// Disables the downloading of code from the directory service or the
	/// Internet. This flag cannot be set at the same time as
	/// `CLSCTX::ENABLE_CODE_DOWNLOAD`.
	NO_CODE_DOWNLOAD 0x400
	/// Specify if you want the activation to fail if it uses custom marshalling.
	NO_CUSTOM_MARSHAL 0x1000
	/// Enables the downloading of code from the directory service or the
	/// Internet. This flag cannot be set at the same time as
	/// `CLSCTX::NO_CODE_DOWNLOAD`.
	ENABLE_CODE_DOWNLOAD 0x2000
	/// The `CLSCTX::NO_FAILURE_LOG` can be used to override the logging of
	/// failures in [`CoCreateInstanceEx`](crate::CoCreateInstanceEx).
	NO_FAILURE_LOG 0x4000
	/// Disables activate-as-activator (AAA) activations for this activation only.
	DISABLE_AAA 0x8000
	/// Enables activate-as-activator (AAA) activations for this activation only.
	ENABLE_AAA 0x1_0000
	/// Begin this activation from the default context of the current apartment.
	FROM_DEFAULT_CONTEXT 0x2_0000
	/// Activate or connect to a 32-bit version of the server; fail if one is not
	/// registered.
	ACTIVATE_X86_SERVER 0x4_0000
	/// Activate or connect to a 32-bit version of the server; fail if one is not
	/// registered.
	ACTIVATE_32_BIT_SERVER Self::ACTIVATE_X86_SERVER.0
	/// Activate or connect to a 64 bit version of the server; fail if one is not
	/// registered.
	ACTIVATE_64_BIT_SERVER 0x8_0000
	/// Specify this flag for Interactive User activation behavior for
	/// As-Activator servers.
	ACTIVATE_AAA_AS_IU 0x80_0000
	/// (No official docs for this entry.)
	ACTIVATE_ARM32_SERVER 0x200_0000
}

const_bitflag! { COINIT: u32: "ole";
	/// [`COINIT`](https://docs.microsoft.com/en-us/windows/win32/api/objbase/ne-objbase-coinit)
	/// enumeration (`u32`).
	=>
	=>
	/// Initializes the thread for apartment-threaded object concurrency.
	///
	/// Use this when in a thread that creates a window.
	APARTMENTTHREADED 0x2
	/// Initializes the thread for multithreaded object concurrency.
	///
	/// Use this when in a thread that doesn't create a window.
	MULTITHREADED 0x0
	/// Disables DDE for OLE1 support.
	///
	/// It's a good idea to add this flag, since it avoids some overhead
	/// associated with OLE 1.0, an obsolete technology.
	DISABLE_OLE1DDE 0x4
	/// Increase memory usage in an attempt to increase performance.
	SPEED_OVER_MEMORY 0x8
}

const_ordinary! { FACILITY: u32: "ole";
	/// [`HRESULT`](crate::co::HRESULT) facility (`u32`).
	=>
	=>
	NULL 0
	RPC 1
	DISPATCH 2
	STORAGE 3
	ITF 4
	WIN32 7
	WINDOWS 8
	SSPI 9
	SECURITY 9
	CONTROL 10
	CERT 11
	INTERNET 12
	MEDIASERVER 13
	MSMQ 14
	SETUPAPI 15
	SCARD 16
	COMPLUS 17
	AAF 18
	URT 19
	ACS 20
	DPLAY 21
	UMI 22
	SXS 23
	WINDOWS_CE 24
	HTTP 25
	USERMODE_COMMONLOG 26
	WER 27
	USERMODE_FILTER_MANAGER 31
	BACKGROUNDCOPY 32
	CONFIGURATION 33
	WIA 33
	STATE_MANAGEMENT 34
	METADIRECTORY 35
	WINDOWSUPDATE 36
	DIRECTORYSERVICE 37
	GRAPHICS 38
	SHELL 39
	NAP 39
	TPM_SERVICES 40
	TPM_SOFTWARE 41
	UI 42
	XAML 43
	ACTION_QUEUE 44
	PLA 48
	WINDOWS_SETUP 48
	FVE 49
	FWP 50
	WINRM 51
	NDIS 52
	USERMODE_HYPERVISOR 53
	CMI 54
	USERMODE_VIRTUALIZATION 55
	USERMODE_VOLMGR 56
	BCD 57
	USERMODE_VHD 58
	USERMODE_HNS 59
	SDIAG 60
	WEBSERVICES 61
	WINPE 61
	WPN 62
	WINDOWS_STORE 63
	INPUT 64
	EAP 66
	WINDOWS_DEFENDER 80
	OPC 81
	XPS 82
	MBN 84
	POWERSHELL 84
	RAS 83
	P2P_INT 98
	P2P 99
	DAF 100
	BLUETOOTH_ATT 101
	AUDIO 102
	STATEREPOSITORY 103
	VISUALCPP 109
	SCRIPT 112
	PARSE 113
	BLB 120
	BLB_CLI 121
	WSBAPP 122
	BLBUI 128
	USN 129
	USERMODE_VOLSNAP 130
	TIERING 131
	WSB_ONLINE 133
	ONLINE_ID 134
	DEVICE_UPDATE_AGENT 135
	DRVSERVICING 136
	DLS 153
	DELIVERY_OPTIMIZATION 208
	USERMODE_SPACES 231
	USER_MODE_SECURITY_CORE 232
	USERMODE_LICENSING 234
	SOS 160
	DEBUGGERS 176
	SPP 256
	RESTORE 256
	DMSERVER 256
	DEPLOYMENT_SERVICES_SERVER 257
	DEPLOYMENT_SERVICES_IMAGING 258
	DEPLOYMENT_SERVICES_MANAGEMENT 259
	DEPLOYMENT_SERVICES_UTIL 260
	DEPLOYMENT_SERVICES_BINLSVC 261
	DEPLOYMENT_SERVICES_PXE 263
	DEPLOYMENT_SERVICES_TFTP 264
	DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT 272
	DEPLOYMENT_SERVICES_DRIVER_PROVISIONING 278
	DEPLOYMENT_SERVICES_MULTICAST_SERVER 289
	DEPLOYMENT_SERVICES_MULTICAST_CLIENT 290
	DEPLOYMENT_SERVICES_CONTENT_PROVIDER 293
	LINGUISTIC_SERVICES 305
	AUDIOSTREAMING 1094
	ACCELERATOR 1536
	WMAAECMA 1996
	DIRECTMUSIC 2168
	DIRECT3D10 2169
	DXGI 2170
	DXGI_DDI 2171
	DIRECT3D11 2172
	DIRECT3D11_DEBUG 2173
	DIRECT3D12 2174
	DIRECT3D12_DEBUG 2175
	LEAP 2184
	AUDCLNT 2185
	WINCODEC_DWRITE_DWM 2200
	WINML 2192
	DIRECT2D 2201
	DEFRAG 2304
	USERMODE_SDBUS 2305
	JSCRIPT 2306
	PIDGENX 2561
	EAS 85
	WEB 885
	WEB_SOCKET 886
	MOBILE 1793
	SQLITE 1967
	UTC 1989
	WEP 2049
	SYNCENGINE 2050
	XBOX 2339
	GAME 2340
	PIX 2748
}
