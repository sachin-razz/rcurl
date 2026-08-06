//! SSH 2.0 / SCP / SFTP Secure Shell Transport & Dynamic SFTP Packet Builder Engine

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct SshEngine;

#[allow(dead_code)]
impl SshEngine {
    pub fn build_ssh_identification_banner(version: &str) -> Vec<u8> {
        format!("SSH-2.0-rcurl_{}\r\n", version).into_bytes()
    }

    pub fn format_ssh_auth_request(user: &str, service: &str, method: &str) -> String {
        format!("SSH_MSG_USERAUTH_REQUEST user: {}, service: {}, method: {}", user, service, method)
    }

    pub fn format_scp_command(remote_path: &str) -> String {
        format!("scp -f {}\n", remote_path)
    }

    /// Dynamically build SFTP packet with message type, request ID, and length prefix (RFC draft-ietf-secsh-filexfer)
    pub fn build_sftp_packet(msg_type: u8, request_id: u32, payload: &[u8]) -> Vec<u8> {
        let payload_len = (1 + 4 + payload.len()) as u32;
        let mut packet = Vec::with_capacity(4 + 1 + 4 + payload.len());
        packet.extend_from_slice(&payload_len.to_be_bytes());
        packet.push(msg_type);
        packet.extend_from_slice(&request_id.to_be_bytes());
        packet.extend_from_slice(payload);
        packet
    }

    /// Dynamically format SFTP SSH_FXP_INIT packet with protocol version
    pub fn format_sftp_init(version: u32) -> Vec<u8> {
        let len = 5u32; // 1 byte msg_type (0x01) + 4 bytes version
        let mut packet = Vec::with_capacity(9);
        packet.extend_from_slice(&len.to_be_bytes());
        packet.push(0x01); // SSH_FXP_INIT
        packet.extend_from_slice(&version.to_be_bytes());
        packet
    }
}
