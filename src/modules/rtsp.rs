//! RTSP/1.0 Real-Time Streaming Protocol Command Formatter (RFC 2326)

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct RtspProtocolEngine {
    pub cseq: u32,
}

#[allow(dead_code)]
impl RtspProtocolEngine {
    pub fn new() -> Self {
        Self { cseq: 1 }
    }

    pub fn next_cseq(&mut self) -> u32 {
        let seq = self.cseq;
        self.cseq += 1;
        seq
    }

    pub fn format_options(&mut self, url: &str) -> String {
        let seq = self.next_cseq();
        format!("OPTIONS {} RTSP/1.0\r\nCSeq: {}\r\nUser-Agent: rcurl/1.0.0\r\n\r\n", url, seq)
    }

    pub fn format_describe(&mut self, url: &str) -> String {
        let seq = self.next_cseq();
        format!(
            "DESCRIBE {} RTSP/1.0\r\nCSeq: {}\r\nAccept: application/sdp\r\nUser-Agent: rcurl/1.0.0\r\n\r\n",
            url, seq
        )
    }

    pub fn format_setup(&mut self, url: &str, transport: &str) -> String {
        let seq = self.next_cseq();
        format!(
            "SETUP {} RTSP/1.0\r\nCSeq: {}\r\nTransport: {}\r\nUser-Agent: rcurl/1.0.0\r\n\r\n",
            url, seq, transport
        )
    }

    pub fn format_play(&mut self, url: &str, session_id: &str) -> String {
        let seq = self.next_cseq();
        format!(
            "PLAY {} RTSP/1.0\r\nCSeq: {}\r\nSession: {}\r\nRange: npt=0.000-\r\nUser-Agent: rcurl/1.0.0\r\n\r\n",
            url, seq, session_id
        )
    }

    pub fn format_teardown(&mut self, url: &str, session_id: &str) -> String {
        let seq = self.next_cseq();
        format!(
            "TEARDOWN {} RTSP/1.0\r\nCSeq: {}\r\nSession: {}\r\nUser-Agent: rcurl/1.0.0\r\n\r\n",
            url, seq, session_id
        )
    }
}
