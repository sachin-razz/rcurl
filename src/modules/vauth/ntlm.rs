//! NTLMSSP NTLM Type 1 & Type 3 Authentication Packet Builder (MS-NLMP Specification)

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct NtlmAuth;

#[allow(dead_code)]
impl NtlmAuth {
    pub fn new() -> Self {
        Self
    }

    pub fn build_type1_negotiate_packet(domain: &str, workstation: &str) -> Vec<u8> {
        let mut packet = Vec::with_capacity(32 + domain.len() + workstation.len());
        packet.extend_from_slice(b"NTLMSSP\0");
        packet.extend_from_slice(&1u32.to_le_bytes());
        packet.extend_from_slice(&0x00080201u32.to_le_bytes());

        let d_bytes = domain.as_bytes();
        packet.extend_from_slice(&(d_bytes.len() as u16).to_le_bytes());
        packet.extend_from_slice(&(d_bytes.len() as u16).to_le_bytes());
        packet.extend_from_slice(&32u32.to_le_bytes());

        let w_bytes = workstation.as_bytes();
        let w_offset = (32 + d_bytes.len()) as u32;
        packet.extend_from_slice(&(w_bytes.len() as u16).to_le_bytes());
        packet.extend_from_slice(&(w_bytes.len() as u16).to_le_bytes());
        packet.extend_from_slice(&w_offset.to_le_bytes());

        packet.extend_from_slice(d_bytes);
        packet.extend_from_slice(w_bytes);
        packet
    }

    pub fn build_ntlm_header(domain: &str, workstation: &str) -> String {
        let type1 = Self::build_type1_negotiate_packet(domain, workstation);
        let b64 = crate::modules::vauth::basic::base64_encode(&type1);
        format!("NTLM {}", b64)
    }
}
