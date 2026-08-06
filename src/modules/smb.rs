//! SMB2 / SMB3 Network File Sharing Protocol Header Engine (MS-SMB2 Specification)

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct SmbProtocolEngine {
    pub message_id: u64,
    pub session_id: u64,
}

#[allow(dead_code)]
impl SmbProtocolEngine {
    pub fn new() -> Self {
        Self {
            message_id: 0,
            session_id: 0,
        }
    }

    pub fn build_smb2_header(&mut self, command_opcode: u16, tree_id: u32) -> [u8; 64] {
        let mut header = [0u8; 64];

        header[0] = 0xFE;
        header[1] = b'S';
        header[2] = b'M';
        header[3] = b'B';

        header[4] = 64;
        header[5] = 0;

        header[12..14].copy_from_slice(&command_opcode.to_le_bytes());

        header[24..32].copy_from_slice(&self.message_id.to_le_bytes());
        self.message_id += 1;

        header[36..40].copy_from_slice(&tree_id.to_le_bytes());
        header[40..48].copy_from_slice(&self.session_id.to_le_bytes());

        header
    }

    pub fn build_negotiate_request(&mut self) -> Vec<u8> {
        let header = self.build_smb2_header(0x0000, 0);
        let mut request = Vec::with_capacity(100);
        request.extend_from_slice(&header);

        request.extend_from_slice(&[36, 0]);
        request.extend_from_slice(&[2, 0]);
        request.extend_from_slice(&[1, 0]);
        request.extend_from_slice(&[0, 0, 0, 0]);
        request.extend_from_slice(&[0; 16]);
        request.extend_from_slice(&[0x02, 0x02]);
        request.extend_from_slice(&[0x00, 0x03]);

        request
    }
}
