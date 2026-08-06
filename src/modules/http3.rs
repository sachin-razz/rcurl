//! HTTP/3 & QPACK Header Compression Control Stream Engine (RFC 9114 & RFC 9204)

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Http3FrameType {
    Data = 0x0,
    Headers = 0x1,
    CancelPush = 0x3,
    Settings = 0x4,
    PushPromise = 0x5,
    GoAway = 0x7,
    MaxPushId = 0xD,
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Http3ProtocolEngine;

#[allow(dead_code)]
impl Http3ProtocolEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn encode_varint(val: u64, out: &mut Vec<u8>) {
        if val <= 63 {
            out.push(val as u8);
        } else if val <= 16383 {
            let v = (val as u16) | 0x4000;
            out.extend_from_slice(&v.to_be_bytes());
        } else if val <= 1073741823 {
            let v = (val as u32) | 0x80000000;
            out.extend_from_slice(&v.to_be_bytes());
        } else {
            let v = val | 0xC000000000000000;
            out.extend_from_slice(&v.to_be_bytes());
        }
    }

    pub fn build_frame(frame_type: Http3FrameType, payload: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(8 + payload.len());
        Self::encode_varint(frame_type as u64, &mut out);
        Self::encode_varint(payload.len() as u64, &mut out);
        out.extend_from_slice(payload);
        out
    }

    pub fn build_settings_frame(max_field_section_size: u64) -> Vec<u8> {
        let mut payload = Vec::new();
        Self::encode_varint(0x6, &mut payload);
        Self::encode_varint(max_field_section_size, &mut payload);

        Self::build_frame(Http3FrameType::Settings, &payload)
    }

    pub fn qpack_static_get_method() -> [u8; 2] {
        [0x00, 0xD1]
    }
}
