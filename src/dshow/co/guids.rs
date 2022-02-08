#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::co::CLSID;

const_guid_values! { CLSID: "dshow";
	AviDest "e2510970-f137-11ce-8b67-00aa00a3f1a6"
	EnhancedVideoRenderer "fa10746c-9b63-4b6c-bc49-fc300ea5f256"
	FileWriter "8596e5f0-0da5-11d0-bd21-00a0c911ce86"
	FilterGraph "e436ebb3-524f-11ce-9f53-0020af0ba770"
	NullRenderer "c1f400a4-3f08-11d3-9f0b-006008039e37"
}

const_guid! { DSHOW_SERVICE: "dshow";
	/// [`IMFGetService::GetService`](`crate::prelude::DshowIMFGetService::GetService`)
	/// `service_guid` (`GUID`).
	=>
	MR_VIDEO_RENDER_SERVICE "1092a86c-ab1a-459a-a336-831fbc4d11ff"
	MR_VIDEO_MIXER_SERVICE "073cd2fc-6cf4-40b7-8859-e89552c841f8"
	MR_VIDEO_ACCELERATION_SERVICE "efef5175-5c7d-4ce2-bbbd-34ff8bca6554"
	MR_BUFFER_SERVICE "a562248c-9ac6-4ffc-9fba-3af8f8ad1a4d"
	VIDEO_ZOOM_RECT "7aaa1638-1b7f-4c93-bd89-5b9c9fb6fcf0"
}

const_guid! { MEDIA_FORMAT: "dshow";
	/// [`AM_MEDIA_TYPE`](crate::AM_MEDIA_TYPE) `formattype`, originally with
	/// `FORMAT` prefix (`GUID`).
	=>
	DvInfo "05589f84-c356-11ce-bf01-00aa0055595a"
	MPEG2Video "e06d80e3-db46-11cf-b4d1-00805f6cbbea"
	MPEGStreams "05589f83-c356-11ce-bf01-00aa0055595a"
	MPEGVideo "05589f82-c356-11ce-bf01-00aa0055595a"
	None "0f6417d6-c318-11d0-a43f-00a0c9223196"
	VideoInfo "05589f80-c356-11ce-bf01-00aa0055595a"
	VideoInfo2 "f72a76a0-eb0a-11d0-ace4-0000c0cc16ba"
	WaveFormatEx "05589f81-c356-11ce-bf01-00aa0055595a"
}

const_guid! { MEDIATYPE: "dshow";
	/// [`AM_MEDIA_TYPE`](crate::AM_MEDIA_TYPE) `majortype` (`GUID`).
	=>
	AnalogAudio "0482dee1-7817-11cf-8a03-00aa006ecb65"
	AnalogVideo "0482dde1-7817-11cf-8a03-00aa006ecb65"
	Audio "73647561-0000-0010-8000-00aa00389b71"
	AUXLine21Data "670aea80-3a82-11d0-b79b-00aa003767a7"
	File "656c6966-0000-0010-8000-00aa00389b71"
	Interleaved "73766169-0000-0010-8000-00aa00389b71"
	Midi "7364696d-0000-0010-8000-00aa00389b71"
	MPEG2_PES "e06d8020-db46-11cf-b4d1-00805f6cbbea"
	MPEG2_SECTIONS "455f176c-4b06-47ce-9aef-8caef73df7b5"
	ScriptCommand "73636d64-0000-0010-8000-00aa00389b71"
	Stream "e436eb83-524f-11ce-9f53-0020af0ba770"
	Text "73747874-0000-0010-8000-00aa00389b71"
	Timecode "0482dee3-7817-11cf-8a03-00aa006ecb65"
	VBI "f72a76e1-eb0a-11d0-ace4-0000c0cc16ba"
	Video "73646976-0000-0010-8000-00aa00389b71"
}

const_guid! { TIME_FORMAT: "dshow";
	/// [`IMediaSeeking::SetTimeFormat`](crate::prelude::DshowIMediaSeeking::SetTimeFormat)
	/// `format` (`GUID`).
	=>
	NONE "00000000-0000-0000-0000-000000000000"
	FRAME "7b785570-8c82-11cf-bc0c-00aa00ac74f6"
	BYTE "7b785571-8c82-11cf-bc0c-00aa00ac74f6"
	SAMPLE "7b785572-8c82-11cf-bc0c-00aa00ac74f6"
	FIELD "7b785573-8c82-11cf-bc0c-00aa00ac74f6"
	MEDIA_TIME "7b785574-8c82-11cf-bc0c-00aa00ac74f6"
}
