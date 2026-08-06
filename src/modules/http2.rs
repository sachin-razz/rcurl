//! HTTP/2 Frame Builder & HPACK Header Compression Engine (RFC 7540 & RFC 7541)

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Http2FrameType {
    Data = 0x0,
    Headers = 0x1,
    Priority = 0x2,
    RstStream = 0x3,
    Settings = 0x4,
    PushPromise = 0x5,
    Ping = 0x6,
    GoAway = 0x7,
    WindowUpdate = 0x8,
    Continuation = 0x9,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Http2Frame {
    pub length: u32,
    pub frame_type: Http2FrameType,
    pub flags: u8,
    pub stream_id: u32,
    pub payload: Vec<u8>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Http2ProtocolEngine {
    pub stream_counter: u32,
}

#[allow(dead_code)]
impl Http2ProtocolEngine {
    pub fn new() -> Self {
        Self { stream_counter: 1 }
    }

    pub fn connection_preface() -> &'static [u8] {
        b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"
    }

    pub fn build_frame(frame_type: Http2FrameType, flags: u8, stream_id: u32, payload: &[u8]) -> Vec<u8> {
        let len = payload.len() as u32;
        let mut bytes = Vec::with_capacity(9 + payload.len());

        bytes.push(((len >> 16) & 0xFF) as u8);
        bytes.push(((len >> 8) & 0xFF) as u8);
        bytes.push((len & 0xFF) as u8);

        bytes.push(frame_type as u8);
        bytes.push(flags);

        let sid = stream_id & 0x7F_FF_FF_FF;
        bytes.push(((sid >> 24) & 0xFF) as u8);
        bytes.push(((sid >> 16) & 0xFF) as u8);
        bytes.push(((sid >> 8) & 0xFF) as u8);
        bytes.push((sid & 0xFF) as u8);

        bytes.extend_from_slice(payload);
        bytes
    }

    pub fn build_settings_frame(header_table_size: u32, max_concurrent_streams: u32) -> Vec<u8> {
        let mut payload = Vec::with_capacity(12);
        payload.extend_from_slice(&1u16.to_be_bytes());
        payload.extend_from_slice(&header_table_size.to_be_bytes());
        payload.extend_from_slice(&3u16.to_be_bytes());
        payload.extend_from_slice(&max_concurrent_streams.to_be_bytes());

        Self::build_frame(Http2FrameType::Settings, 0x0, 0, &payload)
    }

    pub fn hpack_index_method_get() -> u8 {
        0x82
    }

    pub fn hpack_index_scheme_https() -> u8 {
        0x87
    }

    pub fn hpack_index_status_200() -> u8 {
        0x88
    }
}
