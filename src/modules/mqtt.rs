//! MQTT 3.1.1 & 5.0 Control Packet Encoder (OASIS Standard)

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct MqttProtocolEngine;

#[allow(dead_code)]
impl MqttProtocolEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn encode_remaining_length(mut length: usize, out: &mut Vec<u8>) {
        loop {
            let mut encoded_byte = (length % 128) as u8;
            length /= 128;
            if length > 0 {
                encoded_byte |= 128;
            }
            out.push(encoded_byte);
            if length == 0 {
                break;
            }
        }
    }

    pub fn build_connect_packet(client_id: &str, keep_alive_secs: u16) -> Vec<u8> {
        let mut variable_header = Vec::new();
        variable_header.extend_from_slice(&[0x00, 0x04, b'M', b'Q', b'T', b'T']);
        variable_header.push(0x04);
        variable_header.push(0x02);
        variable_header.extend_from_slice(&keep_alive_secs.to_be_bytes());

        let mut payload = Vec::new();
        let cid_bytes = client_id.as_bytes();
        payload.extend_from_slice(&(cid_bytes.len() as u16).to_be_bytes());
        payload.extend_from_slice(cid_bytes);

        let mut remaining = variable_header;
        remaining.extend_from_slice(&payload);

        let mut packet = Vec::with_capacity(2 + remaining.len());
        packet.push(0x10);
        Self::encode_remaining_length(remaining.len(), &mut packet);
        packet.extend_from_slice(&remaining);

        packet
    }

    pub fn build_publish_packet(topic: &str, payload_data: &[u8]) -> Vec<u8> {
        let mut variable_header = Vec::new();
        let topic_bytes = topic.as_bytes();
        variable_header.extend_from_slice(&(topic_bytes.len() as u16).to_be_bytes());
        variable_header.extend_from_slice(topic_bytes);

        let mut remaining = variable_header;
        remaining.extend_from_slice(payload_data);

        let mut packet = Vec::with_capacity(2 + remaining.len());
        packet.push(0x30);
        Self::encode_remaining_length(remaining.len(), &mut packet);
        packet.extend_from_slice(&remaining);

        packet
    }

    pub fn build_pingreq_packet() -> [u8; 2] {
        [0xC0, 0x00]
    }

    pub fn build_disconnect_packet() -> [u8; 2] {
        [0xE0, 0x00]
    }
}
