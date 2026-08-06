//! TFTP Trivial File Transfer Protocol Binary Packet Builder (RFC 1350)

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TftpOpcode {
    Rrq = 1,
    Wrq = 2,
    Data = 3,
    Ack = 4,
    Error = 5,
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct TftpProtocolEngine;

#[allow(dead_code)]
impl TftpProtocolEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn build_request_packet(opcode: TftpOpcode, filename: &str, mode: &str) -> Vec<u8> {
        let mut packet = Vec::with_capacity(4 + filename.len() + mode.len());
        packet.extend_from_slice(&(opcode as u16).to_be_bytes());
        packet.extend_from_slice(filename.as_bytes());
        packet.push(0x00);
        packet.extend_from_slice(mode.as_bytes());
        packet.push(0x00);
        packet
    }

    pub fn build_ack_packet(block_number: u16) -> [u8; 4] {
        let mut packet = [0u8; 4];
        packet[0..2].copy_from_slice(&(TftpOpcode::Ack as u16).to_be_bytes());
        packet[2..4].copy_from_slice(&block_number.to_be_bytes());
        packet
    }

    pub fn build_data_packet(block_number: u16, data: &[u8]) -> Vec<u8> {
        let mut packet = Vec::with_capacity(4 + data.len());
        packet.extend_from_slice(&(TftpOpcode::Data as u16).to_be_bytes());
        packet.extend_from_slice(&block_number.to_be_bytes());
        packet.extend_from_slice(data);
        packet
    }
}
